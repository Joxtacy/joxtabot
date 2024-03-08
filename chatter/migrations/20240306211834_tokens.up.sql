CREATE TABLE IF NOT EXISTS tokens
(
    id          BIGSERIAL PRIMARY KEY,
    name         TEXT    NOT NULL,
    access_token TEXT    NOT NULL,
    refresh_token TEXT    NOT NULL,
    created_at   TIMESTAMPTZ NOT NULL,
    expires_at   TIMESTAMPTZ NOT NULL
);
