-- Add up migration script here
CREATE TABLE IF NOT EXISTS recent_files
(
    path      TEXT PRIMARY KEY,
    name      TEXT    NOT NULL,
    opened_at INTEGER NOT NULL
);
