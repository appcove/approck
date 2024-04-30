pub use redis::AsyncCommands;
use redis::{FromRedisValue, ToRedisArgs};

#[derive(Debug, serde::Deserialize)]
pub struct RedisConfig {
    pub host: String,
    pub port: Option<u16>,
    pub database: u16,
    pub connect_timeout: Option<u64>,
}

impl RedisConfig {
    pub fn to_url(&self) -> String {
        format!(
            "redis://{}:{}/{}",
            self.host,
            self.port.unwrap_or(6379),
            self.database
        )
    }

    pub async fn create_pool(&self) -> granite::Result<RedisPool> {
        RedisPool::new(self).await
    }

    pub async fn into_system(self) -> granite::Result<RedisSystem> {
        let pool = self.create_pool().await?;
        Ok(RedisSystem { config: self, pool })
    }
}

#[allow(dead_code)]
pub struct RedisSystem {
    config: RedisConfig,
    pool: RedisPool,
}

impl RedisSystem {
    pub async fn new(config: RedisConfig) -> granite::Result<Self> {
        let pool = RedisPool::new(&config).await?;
        Ok(Self { config, pool })
    }

    pub async fn get_dbcx(&self) -> granite::Result<RedisCX> {
        self.pool.get().await
    }
}

pub trait RedisModule {
    fn redis_dbcx(
        &self,
    ) -> impl std::future::Future<
        // LUKE: is this different than RedisCX?
        Output = granite::Result<RedisCX>,
    > + Send;
}

#[derive(Debug, Clone)]
pub struct RedisPool {
    pool: bb8::Pool<bb8_redis::RedisConnectionManager>,
}

impl RedisPool {
    pub async fn new(config: &RedisConfig) -> granite::Result<Self> {
        let url = format!("redis://{}:{}/", config.host, config.port.unwrap_or(6379));

        let manager = bb8_redis::RedisConnectionManager::new(url)?;

        let pool = bb8::Pool::builder()
            .connection_timeout(std::time::Duration::from_secs(
                config.connect_timeout.unwrap_or(5),
            ))
            .build(manager)
            .await?;

        Ok(Self { pool })
    }

    pub async fn get(&self) -> granite::Result<RedisCX> {
        Ok(RedisCX {
            cx: self.pool.get().await?,
        })
    }
}

#[derive(Debug)]
pub struct RedisCX<'a> {
    cx: bb8::PooledConnection<'a, bb8_redis::RedisConnectionManager>,
}

impl<'a> RedisCX<'a> {
    pub async fn get_val<T>(&mut self, key: &str) -> granite::Result<T>
    where
        T: redis::FromRedisValue,
    {
        let value: T = self.cx.get(key).await?;
        Ok(value)
    }

    pub async fn mget_val<T>(&mut self, keys: &[&str]) -> granite::Result<Vec<T>>
    where
        T: redis::FromRedisValue,
    {
        let values = self.cx.mget(keys).await?;
        Ok(values)
    }

    pub async fn set_val<T>(&mut self, key: &str, value: T) -> granite::Result<()>
    where
        T: redis::ToRedisArgs + Send + Sync,
    {
        self.cx.set(key, value).await?;
        Ok(())
    }

    pub async fn keys_str(&mut self, pattern: &str) -> granite::Result<Vec<String>> {
        let keys: Vec<String> = self.cx.keys(pattern).await?;
        Ok(keys)
    }

    pub async fn set_json<T>(&mut self, key: &str, value: &T) -> granite::Result<()>
    where
        T: serde::Serialize,
    {
        let value = serde_json::to_string(value)?;
        self.cx.set(key, value).await?;
        Ok(())
    }

    pub async fn get_json<T>(&mut self, key: &str) -> granite::Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let value: String = self.cx.get(key).await?;
        let value: T = serde_json::from_str(&value)?;
        Ok(value)
    }

    pub async fn mget_json<T>(&mut self, keys: &[&str]) -> granite::Result<Vec<T>>
    where
        T: serde::de::DeserializeOwned,
    {
        let values: Vec<String> = self.cx.mget(keys).await?;
        let values: Vec<T> = values
            .into_iter()
            .map(|value| serde_json::from_str(&value))
            .collect::<Result<Vec<T>, _>>()?;
        Ok(values)
    }

    pub async fn append_str(&mut self, key: &str, value: &str) -> granite::Result<()> {
        self.cx.append(key, value).await?;
        Ok(())
    }

    // Hash operations
    pub async fn hkeys_str(&mut self, key: &str) -> granite::Result<Vec<String>> {
        let keys: Vec<String> = self.cx.hkeys(key).await?;
        Ok(keys)
    }

    pub async fn hvals_json<T>(&mut self, key: &str) -> granite::Result<Vec<T>>
    where
        T: serde::de::DeserializeOwned,
    {
        let values: Vec<String> = self.cx.hvals(key).await?;
        let values: Vec<T> = values
            .into_iter()
            .map(|value| serde_json::from_str(&value))
            .collect::<Result<Vec<T>, _>>()?;
        Ok(values)
    }

    pub async fn hset_val<T>(&mut self, key: &str, field: &str, value: T) -> granite::Result<()>
    where
        T: redis::ToRedisArgs + Send + Sync,
    {
        self.cx.hset(key, field, value).await?;
        Ok(())
    }

    pub async fn hget_val<T>(&mut self, key: &str, field: &str) -> granite::Result<T>
    where
        T: redis::FromRedisValue,
    {
        let value: T = self.cx.hget(key, field).await?;
        Ok(value)
    }

    pub async fn hget_json<T>(&mut self, key: &str, field: &str) -> granite::Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let value: String = self.cx.hget(key, field).await?;
        let value: T = serde_json::from_str(&value)?;
        Ok(value)
    }

    pub async fn hset_json<T>(&mut self, key: &str, field: &str, value: &T) -> granite::Result<()>
    where
        T: serde::Serialize,
    {
        let value = serde_json::to_string(value)?;
        self.cx.hset(key, field, value).await?;
        Ok(())
    }

    // List operations
    pub async fn lindex_val<T>(&mut self, key: &str, index: isize) -> granite::Result<T>
    where
        T: redis::FromRedisValue,
    {
        let value: T = self.cx.lindex(key, index).await?;
        Ok(value)
    }

    pub async fn lindex_json<T>(&mut self, key: &str, index: isize) -> granite::Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let value: String = self.cx.lindex(key, index).await?;
        let value: T = serde_json::from_str(&value)?;
        Ok(value)
    }

    pub async fn lpop_val<T>(&mut self, key: &str) -> granite::Result<T>
    where
        T: redis::FromRedisValue,
    {
        let value: T = self.cx.lpop(key, None).await?;
        Ok(value)
    }

    pub async fn lpop_json<T>(&mut self, key: &str) -> granite::Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let value: String = self.cx.lpop(key, None).await?;
        let value: T = serde_json::from_str(&value)?;
        Ok(value)
    }

    pub async fn lpush_val<T>(&mut self, key: &str, value: T) -> granite::Result<()>
    where
        T: redis::ToRedisArgs + Send + Sync,
    {
        self.cx.lpush(key, value).await?;
        Ok(())
    }

    pub async fn lpush_json<T>(&mut self, key: &str, value: &T) -> granite::Result<()>
    where
        T: serde::Serialize,
    {
        let value = serde_json::to_string(value)?;
        self.cx.lpush(key, value).await?;
        Ok(())
    }

    pub async fn lrange_val<T>(
        &mut self,
        key: &str,
        start: isize,
        stop: isize,
    ) -> granite::Result<Vec<T>>
    where
        T: redis::FromRedisValue,
    {
        let values: Vec<T> = self.cx.lrange(key, start, stop).await?;
        Ok(values)
    }

    pub async fn lrange_json<T>(
        &mut self,
        key: &str,
        start: isize,
        stop: isize,
    ) -> granite::Result<Vec<T>>
    where
        T: serde::de::DeserializeOwned,
    {
        let values: Vec<String> = self.cx.lrange(key, start, stop).await?;
        let values: Vec<T> = values
            .into_iter()
            .map(|value| serde_json::from_str(&value))
            .collect::<Result<Vec<T>, _>>()?;
        Ok(values)
    }

    pub async fn lset_val<T>(&mut self, key: &str, index: isize, value: T) -> granite::Result<()>
    where
        T: redis::ToRedisArgs + Send + Sync,
    {
        self.cx.lset(key, index, value).await?;
        Ok(())
    }

    pub async fn lset_json<T>(&mut self, key: &str, index: isize, value: &T) -> granite::Result<()>
    where
        T: serde::Serialize,
    {
        let value = serde_json::to_string(value)?;
        self.cx.lset(key, index, value).await?;
        Ok(())
    }

    pub async fn rpop_val<T>(&mut self, key: &str) -> granite::Result<T>
    where
        T: redis::FromRedisValue,
    {
        let value: T = self.cx.rpop(key, None).await?;
        Ok(value)
    }

    pub async fn rpop_json<T>(&mut self, key: &str) -> granite::Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let value: String = self.cx.rpop(key, None).await?;
        let value: T = serde_json::from_str(&value)?;
        Ok(value)
    }

    pub async fn rpush_val<T>(&mut self, key: &str, value: T) -> granite::Result<()>
    where
        T: redis::ToRedisArgs + Send + Sync,
    {
        self.cx.rpush(key, value).await?;
        Ok(())
    }

    pub async fn rpush_json<T>(&mut self, key: &str, value: &T) -> granite::Result<()>
    where
        T: serde::Serialize,
    {
        let value = serde_json::to_string(value)?;
        self.cx.rpush(key, value).await?;
        Ok(())
    }

    // Counter operations
    pub async fn incr<T>(&mut self, key: &str, delta: T) -> granite::Result<T>
    where
        T: ToRedisArgs + FromRedisValue + Send + Sync,
    {
        let value: T = self.cx.incr(key, delta).await?;
        Ok(value)
    }

    pub async fn decr<T>(&mut self, key: &str, delta: T) -> granite::Result<T>
    where
        T: ToRedisArgs + FromRedisValue + Send + Sync,
    {
        let value: T = self.cx.decr(key, delta).await?;
        Ok(value)
    }
}
