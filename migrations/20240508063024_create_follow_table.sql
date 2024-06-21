-- Add migration script here
CREATE TABLE follow (
    uuid TEXT NOT NULL,
    created_at INTEGER,
    updated_at INTEGER,
    account_id TEXT NOT NULL,
    target_account_id TEXT NOT NULL,
    show_reblogs BOOLEAN DEFAULT true NOT NULL,
    uri TEXT,
    notify BOOLEAN DEFAULT false NOT NULL,
    languages TEXT
);