CREATE TABLE IF NOT EXISTS songs (
    id TEXT PRIMARY KEY NOT NULL, -- Using filename as ID for now
    name TEXT NOT NULL,
    file_path TEXT NOT NULL UNIQUE -- Store the full path or relative path if needed
);

CREATE TABLE IF NOT EXISTS votes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    voter_id TEXT NOT NULL, -- Will be a UUID string
    song_id TEXT NOT NULL,
    decision INTEGER NOT NULL, -- Value from -127 to 127 (matching slider)
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(song_id) REFERENCES songs(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_votes_song_id ON votes(song_id);
CREATE INDEX IF NOT EXISTS idx_votes_voter_id ON votes(voter_id);
CREATE UNIQUE INDEX IF NOT EXISTS idx_voter_song_unique ON votes(voter_id, song_id);
