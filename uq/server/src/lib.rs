pub mod api;
pub mod db;

pub mod uq_proto {
    include!(concat!(env!("OUT_DIR"), "/uq.v1.rs"));
}

use api::UqServiceHandler;
use axum::Router;
use uq_proto::uqservice;

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

        uqservice::UqServiceBuilder::new()
            .sync(move |req| {
                let h = handler.clone();
                async move { h.sync(req).await }
            })
            .build_connect()
    }
}
