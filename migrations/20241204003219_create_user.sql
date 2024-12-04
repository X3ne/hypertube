CREATE TABLE IF NOT EXISTS user
(
    id          TEXT PRIMARY KEY NOT NULL,
    username    TEXT             NOT NULL UNIQUE,
    email       TEXT             NOT NULL UNIQUE,
    password    TEXT,
    permissions INTEGER          NOT NULL DEFAULT 0,
    created_at  DATETIME         NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at  DATETIME         NOT NULL DEFAULT CURRENT_TIMESTAMP
);
