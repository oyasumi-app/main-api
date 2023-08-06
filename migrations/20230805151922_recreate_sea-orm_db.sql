-- Add migration script here
CREATE TABLE IF NOT EXISTS user (
    id INTEGER NOT NULL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS user_token (
    id INTEGER NOT NULL PRIMARY KEY,
    token TEXT NOT NULL UNIQUE,
    user_id INTEGER NOT NULL REFERENCES user(id),
    created_by_ip TEXT NOT NULL,
    expires_unix_time INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS registration (
    id INTEGER NOT NULL,
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_by_ip TEXT NOT NULL,
    confirm_token TEXT NOT NULL UNIQUE,
    expires_unix_time INTEGER NOT NULL,
    email_resend_after_unix_time INTEGER NOT NULL
);