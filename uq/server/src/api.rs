use crate::{
    db::{self, Db},
    uq_proto::{Event, SyncRequest, SyncResponse},
};
use connectrpc_axum::{ConnectError, ConnectRequest, ConnectResponse};
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use sha2::{Digest, Sha256};

#[derive(Clone)]
pub struct UqServiceHandler {
    db: Db,
}

impl UqServiceHandler {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub async fn sync(
        &self,
        req: ConnectRequest<SyncRequest>,
    ) -> Result<ConnectResponse<SyncResponse>, ConnectError> {
        let events = req.0.events;
        let since = req.0.since_timestamp_ms;

        // 1. Process pushed events
        for mut event in events {
            if !self.verify_signature(&event) {
                tracing::warn!("Invalid signature for event");
                continue;
            }

            event.server_timestamp_ms = chrono::Utc::now().timestamp_millis();

            if let Err(e) = db::insert_event(&self.db, &event).await {
                tracing::error!("DB Error: {}", e);
                // Continue despite errors? For now, we'll just log.
                // return Err(ConnectError::new_internal("Database error"));
            }
        }

        // 2. Fetch new events
        let events = db::get_events_since(&self.db, since).await.map_err(|e| {
            tracing::error!("DB Error: {}", e);
            ConnectError::new_internal("Database error")
        })?;

        let server_timestamp_ms = chrono::Utc::now().timestamp_millis();

        Ok(ConnectResponse::new(SyncResponse {
            events,
            server_timestamp_ms,
        }))
    }

    fn verify_signature(&self, event: &Event) -> bool {
        if event.author_pk.len() != 32 || event.signature.len() != 64 {
            return false;
        }

        let Ok(vk) = VerifyingKey::from_bytes(event.author_pk.as_slice().try_into().unwrap())
        else {
            return false;
        };

        let sig = Signature::from_bytes(event.signature.as_slice().try_into().unwrap());

        // Construct message: sha256(payload) + topic_pk
        let mut hasher = Sha256::new();
        hasher.update(&event.payload);
        let payload_hash = hasher.finalize();

        let mut msg = Vec::with_capacity(32 + event.topic_pk.len());
        msg.extend_from_slice(&payload_hash);
        msg.extend_from_slice(&event.topic_pk);

        vk.verify(&msg, &sig).is_ok()
    }
}
