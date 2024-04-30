mod approck_generated;
mod ui;
mod user;
mod web;

use approck::server::response::Response;
use approck::traits::{Document, DocumentModule};

#[derive(serde::Deserialize)]
pub struct AppConfig {
    pub redis: granite_redis::RedisConfig,
    pub postgres: granite_postgres::PostgresConfig,
    pub webserver: approck::server::WebServerConfig,
}

impl AppConfig {
    pub async fn into_system(self) -> granite::Result<AppSystem> {
        Ok(AppSystem {
            redis_system: self.redis.into_system().await?,
            postgres_system: self.postgres.into_system().await?,
            webserver_system: self.webserver.into_system(),
        })
    }
}

pub struct AppSystem {
    pub redis_system: granite_redis::RedisSystem,
    pub postgres_system: granite_postgres::PostgresSystem,
    pub webserver_system: approck::server::WebServerSystem,
}

impl approck::traits::DocumentModule for AppSystem {
    fn get_document(&self) -> impl approck::traits::Document {
        let mut doc = crate::ui::Document::default();
        doc.set_title("approck-example");
        doc.add_js("/app.js");
        doc.add_css("/app.css");
        doc
    }
}

impl granite_postgres::PostgresModule for AppSystem {
    async fn postgres_dbcx(&self) -> granite::Result<granite_postgres::DBCX> {
        self.postgres_system.get_dbcx().await
    }
}

impl granite_redis::RedisModule for AppSystem {
    async fn redis_dbcx(&self) -> granite::Result<granite_redis::RedisCX> {
        self.redis_system.get_dbcx().await
    }
}

impl approck::server::WebServerModule for AppSystem {
    fn webserver_system(&self) -> &approck::server::WebServerSystem {
        &self.webserver_system
    }

    async fn webserver_route<'a>(
        &'static self,
        req: approck::server::Request<'a>,
    ) -> granite::Result<approck::server::response::Response> {
        approck_generated::router(self, req).await
    }

    fn webserver_handle_error(&self, error: granite::Error) -> approck::server::response::Result {
        let mut doc = self.get_document();

        doc.add_body(maud::html! {
            div.container.bg-white {
                h1 { "Error" }
                pre { (error) }
            }
        });

        Ok(Response::HTML(doc.into()))
    }
}
