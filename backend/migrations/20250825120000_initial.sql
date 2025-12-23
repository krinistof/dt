CREATE TABLE IF NOT EXISTS event_queue (
    pub_key BLOB NOT NULL,
    signature BLOB PRIMARY KEY NOT NULL,
    event_blob BLOB NOT NULL,
    server_timestamp_ms INTEGER NOT NULL
) STRICT;

CREATE INDEX IF NOT EXISTS idx_event_queue_server_timestamp ON event_queue (server_timestamp_ms);
