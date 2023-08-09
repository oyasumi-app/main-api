-- Add migration script here
CREATE TABLE IF NOT EXISTS sleep_state (
    id INTEGER NOT NULL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES user(id),
    started_at_unix_time INTEGER NOT NULL,
    ended_at_unix_time INTEGER,
    comment TEXT
);