CREATE TABLE IF NOT EXISTS developer_keys (
    id BIGSERIAL PRIMARY KEY,
    member_id BIGINT NOT NULL UNIQUE REFERENCES members(id) ON DELETE CASCADE,
    api_key VARCHAR(64) NOT NULL UNIQUE,
    label VARCHAR(64) NOT NULL DEFAULT 'Developer Key',
    permissions BIGINT NOT NULL DEFAULT 0,
    rate_limit INT NOT NULL DEFAULT 600,
    request_count BIGINT NOT NULL DEFAULT 0,
    locked BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_developer_keys_api_key ON developer_keys(api_key);
