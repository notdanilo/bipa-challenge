-- Add migration script here
CREATE TABLE nodes (
    public_key TEXT PRIMARY KEY,
    alias TEXT NOT NULL,
    capacity BIGINT NOT NULL,
    first_seen BIGINT NOT NULL,
    updated_at BIGINT NOT NULL
);
