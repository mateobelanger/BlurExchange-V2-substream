 CREATE TABLE IF NOT EXISTS exchangev2_admin_changed1 (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "new_admin" VARCHAR(40),
    "previous_admin" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS exchangev2_admin_changed2 (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "new_admin" VARCHAR(40),
    "previous_admin" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS exchangev2_beacon_upgraded1 (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "beacon" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS exchangev2_beacon_upgraded2 (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "beacon" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS exchangev2_cancel_trade (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "amount" DECIMAL,
    "hash" TEXT,
    "index" DECIMAL,
    "user" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS exchangev2_execution (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "listing_index" DECIMAL,
    "order_hash" TEXT,
    "order_type" INT,
    "price" DECIMAL,
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS exchangev2_execution721_maker_fee_packed (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "collection_price_side" DECIMAL,
    "maker_fee_recipient_rate" DECIMAL,
    "order_hash" TEXT,
    "token_id_listing_index_trader" DECIMAL,
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS exchangev2_execution721_packed (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "collection_price_side" DECIMAL,
    "order_hash" TEXT,
    "token_id_listing_index_trader" DECIMAL,
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS exchangev2_execution721_taker_fee_packed (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "collection_price_side" DECIMAL,
    "order_hash" TEXT,
    "taker_fee_recipient_rate" DECIMAL,
    "token_id_listing_index_trader" DECIMAL,
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS exchangev2_initialized (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "version" INT,
    PRIMARY KEY(evt_tx_hash,evt_index)
);

CREATE TABLE IF NOT EXISTS exchangev2_new_governor (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "governor" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS exchangev2_new_protocol_fee (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "rate" INT,
    "recipient" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS exchangev2_nonce_incremented (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "new_nonce" DECIMAL,
    "user" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS exchangev2_ownership_transfer_started (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "new_owner" VARCHAR(40),
    "previous_owner" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS exchangev2_ownership_transferred (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "new_owner" VARCHAR(40),
    "previous_owner" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS exchangev2_set_oracle (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "approved" BOOL,
    "user" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS exchangev2_upgraded1 (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "implementation" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS exchangev2_upgraded2 (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "implementation" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);

