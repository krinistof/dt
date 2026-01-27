CREATE TABLE IF NOT EXISTS events (
    topic_pk BLOB NOT NULL,
    author_pk BLOB NOT NULL,
    signature BLOB NOT NULL PRIMARY KEY,
    payload BLOB NOT NULL,
    server_timestamp_ms INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_events_timestamp ON events (server_timestamp_ms);
CREATE INDEX IF NOT EXISTS idx_events_topic_timestamp ON events (topic_pk, server_timestamp_ms);
