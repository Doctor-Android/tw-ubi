-- TW-UBI Database Schema
-- Event-sourced, forkable state

-- Users (personhood registry)
CREATE TABLE IF NOT EXISTS users (
    person_id BYTEA PRIMARY KEY,
    wallet_address TEXT NOT NULL UNIQUE,
    region_id INTEGER NOT NULL,
    expiry_epoch INTEGER NOT NULL,
    last_reset_epoch INTEGER NOT NULL DEFAULT 0,
    is_active BOOLEAN NOT NULL DEFAULT true,
    mfa_secret TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- UE balances
CREATE TABLE IF NOT EXISTS ue_balances (
    wallet_address TEXT PRIMARY KEY,
    balance TEXT NOT NULL DEFAULT '0',
    CHECK (balance >= '0')
);

-- BU balances
CREATE TABLE IF NOT EXISTS bu_balances (
    wallet_address TEXT PRIMARY KEY,
    balance TEXT NOT NULL DEFAULT '0',
    CHECK (balance >= '0')
);

-- UBI claims
CREATE TABLE IF NOT EXISTS ubi_claims (
    id BIGSERIAL PRIMARY KEY,
    person_id BYTEA NOT NULL,
    epoch INTEGER NOT NULL,
    amount_ue TEXT NOT NULL,
    claimed_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    UNIQUE(person_id, epoch)
);

-- Last claimed epoch (per person, per region)
CREATE TABLE IF NOT EXISTS last_claimed_epoch (
    person_id BYTEA NOT NULL,
    region_id INTEGER NOT NULL,
    epoch INTEGER NOT NULL,
    PRIMARY KEY (person_id, region_id)
);

-- Pending conversions
CREATE TABLE IF NOT EXISTS pending_conversions (
    id BIGSERIAL PRIMARY KEY,
    person_id BYTEA NOT NULL,
    amount_ue TEXT NOT NULL,
    amount_bu TEXT NOT NULL,
    rate_index TEXT NOT NULL,
    unlock_epoch INTEGER NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    CHECK (status IN ('pending', 'unlocked', 'claimed'))
);

-- Converted this epoch (per person, per epoch)
CREATE TABLE IF NOT EXISTS converted_this_epoch (
    person_id BYTEA NOT NULL,
    epoch INTEGER NOT NULL,
    amount_ue TEXT NOT NULL,
    PRIMARY KEY (person_id, epoch)
);

-- Rate index (per region)
CREATE TABLE IF NOT EXISTS rate_index (
    region_id INTEGER PRIMARY KEY,
    rate_index_wad TEXT NOT NULL,
    last_epoch INTEGER NOT NULL,
    current_decay_rate_wad TEXT NOT NULL,
    last_decay_update_epoch INTEGER NOT NULL
);

-- Region oracle data
CREATE TABLE IF NOT EXISTS region_oracle_data (
    region_id INTEGER PRIMARY KEY,
    current_basket_index_wad TEXT NOT NULL,
    current_inflation_rate_wad TEXT NOT NULL,
    last_update_timestamp BIGINT NOT NULL
);

-- Treasury
CREATE TABLE IF NOT EXISTS treasury (
    id BIGSERIAL PRIMARY KEY,
    balance_bu TEXT NOT NULL DEFAULT '0',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    CHECK (balance_bu >= '0')
);

-- Events (append-only, for forkability)
CREATE TABLE IF NOT EXISTS events (
    id BIGSERIAL PRIMARY KEY,
    event_type TEXT NOT NULL,
    event_data JSONB NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_users_wallet ON users(wallet_address);
CREATE INDEX IF NOT EXISTS idx_claims_person ON ubi_claims(person_id);
CREATE INDEX IF NOT EXISTS idx_claims_epoch ON ubi_claims(epoch);
CREATE INDEX IF NOT EXISTS idx_conversions_person ON pending_conversions(person_id);
CREATE INDEX IF NOT EXISTS idx_conversions_unlock ON pending_conversions(unlock_epoch, status);
CREATE INDEX IF NOT EXISTS idx_events_type ON events(event_type);
CREATE INDEX IF NOT EXISTS idx_events_created ON events(created_at);

-- Initialize treasury (1M BU)
INSERT INTO treasury (balance_bu) VALUES ('1000000000000000000000000')
ON CONFLICT DO NOTHING;

