CREATE TABLE IF NOT EXISTS exchangev2_admin_changed1 (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "new_admin" VARCHAR(40),
    "previous_admin" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS exchangev2_admin_changed2 (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "new_admin" VARCHAR(40),
    "previous_admin" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS exchangev2_beacon_upgraded1 (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "beacon" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS exchangev2_beacon_upgraded2 (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "beacon" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS exchangev2_cancel_trade (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "amount" UInt256,
    "hash" TEXT,
    "index" UInt256,
    "user" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS exchangev2_execution (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "listing_index" UInt256,
    "order_hash" TEXT,
    "order_type" UInt8,
    "price" UInt256
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS exchangev2_execution721_maker_fee_packed (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "collection_price_side" UInt256,
    "maker_fee_recipient_rate" UInt256,
    "order_hash" TEXT,
    "token_id_listing_index_trader" UInt256
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS exchangev2_execution721_packed (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "collection_price_side" UInt256,
    "order_hash" TEXT,
    "token_id_listing_index_trader" UInt256
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS exchangev2_execution721_taker_fee_packed (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "collection_price_side" UInt256,
    "order_hash" TEXT,
    "taker_fee_recipient_rate" UInt256,
    "token_id_listing_index_trader" UInt256
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS exchangev2_initialized (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "version" UInt8
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS exchangev2_new_block_range (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "block_range" UInt256
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS exchangev2_new_governor (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "governor" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS exchangev2_new_protocol_fee (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "rate" UInt16,
    "recipient" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS exchangev2_nonce_incremented (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "new_nonce" UInt256,
    "user" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS exchangev2_ownership_transfer_started (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "new_owner" VARCHAR(40),
    "previous_owner" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS exchangev2_ownership_transferred (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "new_owner" VARCHAR(40),
    "previous_owner" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS exchangev2_set_oracle (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "approved" BOOL,
    "user" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS exchangev2_upgraded1 (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "implementation" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS exchangev2_upgraded2 (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "implementation" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");

