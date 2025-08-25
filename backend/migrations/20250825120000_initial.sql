-- Add migration script here
CREATE TABLE tokens (
    token TEXT PRIMARY KEY NOT NULL
);

CREATE TABLE global_event_queue (
    event_id TEXT PRIMARY KEY NOT NULL,
    event_type TEXT NOT NULL,
    payload TEXT NOT NULL,
    server_created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE posts (
    post_id TEXT PRIMARY KEY NOT NULL,
    content TEXT NOT NULL,
    last_updated TIMESTAMP NOT NULL
);
