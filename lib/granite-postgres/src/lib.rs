pub use tokio_postgres::error::Error as PgError;
pub use tokio_postgres::types::ToSql;
pub use tokio_postgres::types::Type as PgType;

use tokio_postgres::ToStatement;

type PostgresCX<'a> =
    bb8::PooledConnection<'a, bb8_postgres::PostgresConnectionManager<tokio_postgres::NoTls>>;

#[derive(Debug, serde::Deserialize)]
pub struct PostgresConfig {
    pub host: String,
    pub port: Option<u16>,
    pub role: String,
    pub password: String,
    pub database: String,
    pub timeout: Option<u64>,
}

impl PostgresConfig {
    pub async fn into_system(self) -> granite::Result<PostgresSystem> {
        let pool = PostgresPool::new(&self).await?;
        Ok(PostgresSystem { config: self, pool })
    }
}

#[allow(dead_code)]
pub struct PostgresSystem {
    config: PostgresConfig,
    pool: PostgresPool,
}

impl PostgresSystem {
    pub async fn get_dbcx(&self) -> granite::Result<DBCX> {
        self.pool.get().await
    }
}

pub trait PostgresModule {
    fn postgres_dbcx(&self) -> impl std::future::Future<Output = granite::Result<DBCX>> + Send;
}

#[derive(Clone)]
pub struct PostgresPool {
    pool: bb8::Pool<bb8_postgres::PostgresConnectionManager<tokio_postgres::NoTls>>,
}

impl core::fmt::Debug for PostgresPool {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Pool").finish()
    }
}

impl PostgresPool {
    // create a new connection pool with a Config
    pub async fn new(config: &PostgresConfig) -> granite::Result<Self> {
        let mut pgconfig = tokio_postgres::config::Config::new();
        pgconfig.host(&config.host);
        pgconfig.port(config.port.unwrap_or(5432));
        pgconfig.user(&config.role);
        pgconfig.password(&config.password);
        pgconfig.dbname(&config.database);
        pgconfig.connect_timeout(std::time::Duration::from_secs(config.timeout.unwrap_or(4)));

        let manager = bb8_postgres::PostgresConnectionManager::new(pgconfig, tokio_postgres::NoTls);

        let pool = bb8::Pool::builder().build(manager).await?;

        Ok(Self { pool })
    }

    pub async fn get(&self) -> granite::Result<DBCX> {
        let conn = self.pool.get().await?;

        Ok(DBCX::wrap(conn))
    }
}

pub struct DBTX<'a> {
    dbtx: tokio_postgres::Transaction<'a>,
}

impl<'a> DBTX<'a> {
    fn wrap(dbtx: tokio_postgres::Transaction<'a>) -> Self {
        Self { dbtx }
    }

    pub async fn transaction(&mut self) -> Result<DBTX<'_>, tokio_postgres::Error> {
        self.dbtx.transaction().await.map(DBTX::wrap)
    }

    pub async fn rollback(self) -> Result<(), tokio_postgres::Error> {
        self.dbtx.rollback().await
    }
}

impl<'a> DB for DBTX<'a> {
    async fn query_one<T>(
        &self,
        statement: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<tokio_postgres::Row, tokio_postgres::Error>
    where
        T: ?Sized + ToStatement,
    {
        self.dbtx.query_one(statement, params).await
    }

    async fn query<T>(
        &self,
        statement: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Vec<tokio_postgres::Row>, tokio_postgres::Error>
    where
        T: ?Sized + ToStatement + Sync,
    {
        self.dbtx.query(statement, params).await
    }
}

pub struct DBCX<'a> {
    dbcx: PostgresCX<'a>,
}

impl<'a> DBCX<'a> {
    fn wrap(dbcx: PostgresCX<'a>) -> Self {
        Self { dbcx }
    }

    pub async fn transaction(&mut self) -> Result<DBTX<'_>, tokio_postgres::Error> {
        self.dbcx.transaction().await.map(DBTX::wrap)
    }
}

impl<'a> DB for DBCX<'a> {
    async fn query_one<T>(
        &self,
        statement: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<tokio_postgres::Row, tokio_postgres::Error>
    where
        T: ?Sized + ToStatement + Sync,
    {
        self.dbcx.query_one(statement, params).await
    }

    async fn query<T>(
        &self,
        statement: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Vec<tokio_postgres::Row>, tokio_postgres::Error>
    where
        T: ?Sized + ToStatement + Sync,
    {
        self.dbcx.query(statement, params).await
    }
}

pub trait DB {
    fn query_one<T>(
        &self,
        statement: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> impl std::future::Future<Output = Result<tokio_postgres::Row, tokio_postgres::Error>> + Send
    where
        T: ?Sized + ToStatement + Sync;

    fn query<T>(
        &self,
        statement: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> impl std::future::Future<Output = Result<Vec<tokio_postgres::Row>, tokio_postgres::Error>> + Send
    where
        T: ?Sized + ToStatement + Sync;
}
