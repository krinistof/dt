pub mod db;
pub mod api;

pub mod uq_proto {
    include!(concat!(env!("OUT_DIR"), "/uq.v1.rs"));
}

use axum::Router;
use api::UqServiceHandler;
use uq_proto::uqservice; // Generated module name usually matches service name snake_case

pub struct UqServer {
    db: db::Db,
}

impl UqServer {
    pub async fn new(db_url: &str) -> anyhow::Result<Self> {
        let db = db::connect(db_url).await?;
        Ok(Self { db })
    }

    pub fn into_router(self) -> Router {
        let handler = UqServiceHandler::new(self.db);
        let handler_push = handler.clone();
        let handler_pull = handler.clone();
        let handler_log = handler;

        uqservice::UqServiceBuilder::new()
            .push(move |req| {
                let h = handler_push.clone();
                async move { h.push(req).await }
            })
            .pull(move |req| {
                let h = handler_pull.clone();
                async move { h.pull(req).await }
            })
            .log(move |req| {
                let h = handler_log.clone();
                async move { h.log(req).await }
            })
            .build_connect()
    }
}
