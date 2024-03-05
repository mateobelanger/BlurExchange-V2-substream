CREATE TABLE IF NOT EXISTS proxy_admin_changed1 (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "new_admin" VARCHAR(40),
    "previous_admin" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS proxy_admin_changed2 (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "new_admin" VARCHAR(40),
    "previous_admin" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS proxy_beacon_upgraded1 (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "beacon" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS proxy_beacon_upgraded2 (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "beacon" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS proxy_cancel_trade (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "amount" UInt256,
    "hash" TEXT,
    "index" UInt256,
    "user" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS proxy_execution (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "listing_index" UInt256,
    "order_hash" TEXT,
    "order_type" UInt8,
    "price" UInt256
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS proxy_execution721_maker_fee_packed (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "collection_price_side" UInt256,
    "maker_fee_recipient_rate" UInt256,
    "order_hash" TEXT,
    "token_id_listing_index_trader" UInt256
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS proxy_execution721_packed (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "collection_price_side" UInt256,
    "order_hash" TEXT,
    "token_id_listing_index_trader" UInt256
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS proxy_execution721_taker_fee_packed (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "collection_price_side" UInt256,
    "order_hash" TEXT,
    "taker_fee_recipient_rate" UInt256,
    "token_id_listing_index_trader" UInt256
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS proxy_initialized (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "version" UInt8
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS proxy_new_block_range (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "block_range" UInt256
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS proxy_new_governor (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "governor" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS proxy_new_protocol_fee (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "rate" UInt16,
    "recipient" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS proxy_nonce_incremented (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "new_nonce" UInt256,
    "user" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS proxy_ownership_transfer_started (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "new_owner" VARCHAR(40),
    "previous_owner" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS proxy_ownership_transferred (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "new_owner" VARCHAR(40),
    "previous_owner" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS proxy_set_oracle (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "approved" BOOL,
    "user" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS proxy_upgraded1 (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "implementation" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS proxy_upgraded2 (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "implementation" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS source_admin_changed (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "new_admin" VARCHAR(40),
    "previous_admin" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS source_beacon_upgraded (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "beacon" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS source_cancel_trade (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "amount" UInt256,
    "hash" TEXT,
    "index" UInt256,
    "user" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS source_execution (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "listing_index" UInt256,
    "order_hash" TEXT,
    "order_type" UInt8,
    "price" UInt256
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS source_execution721_maker_fee_packed (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "collection_price_side" UInt256,
    "maker_fee_recipient_rate" UInt256,
    "order_hash" TEXT,
    "token_id_listing_index_trader" UInt256
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS source_execution721_packed (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "collection_price_side" UInt256,
    "order_hash" TEXT,
    "token_id_listing_index_trader" UInt256
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS source_execution721_taker_fee_packed (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "collection_price_side" UInt256,
    "order_hash" TEXT,
    "taker_fee_recipient_rate" UInt256,
    "token_id_listing_index_trader" UInt256
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS source_initialized (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "version" UInt8
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS source_new_block_range (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "block_range" UInt256
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS source_new_governor (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "governor" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS source_new_protocol_fee (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "rate" UInt16,
    "recipient" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS source_nonce_incremented (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "new_nonce" UInt256,
    "user" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS source_ownership_transfer_started (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "new_owner" VARCHAR(40),
    "previous_owner" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS source_ownership_transferred (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "new_owner" VARCHAR(40),
    "previous_owner" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS source_set_oracle (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "approved" BOOL,
    "user" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS source_upgraded (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "implementation" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");

CREATE TABLE IF NOT EXISTS source_call_accept_ownership (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" UInt64,
    "call_ordinal" INT,
    "call_success" BOOL
) ENGINE = MergeTree PRIMARY KEY ("call_tx_hash","call_ordinal");
CREATE TABLE IF NOT EXISTS source_call_cancel_trades (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" UInt64,
    "call_ordinal" INT,
    "call_success" BOOL
) ENGINE = MergeTree PRIMARY KEY ("call_tx_hash","call_ordinal");
CREATE TABLE IF NOT EXISTS source_call_increment_nonce (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" UInt64,
    "call_ordinal" INT,
    "call_success" BOOL
) ENGINE = MergeTree PRIMARY KEY ("call_tx_hash","call_ordinal");
CREATE TABLE IF NOT EXISTS source_call_initialize (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" UInt64,
    "call_ordinal" INT,
    "call_success" BOOL
) ENGINE = MergeTree PRIMARY KEY ("call_tx_hash","call_ordinal");
CREATE TABLE IF NOT EXISTS source_call_renounce_ownership (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" UInt64,
    "call_ordinal" INT,
    "call_success" BOOL
) ENGINE = MergeTree PRIMARY KEY ("call_tx_hash","call_ordinal");
CREATE TABLE IF NOT EXISTS source_call_set_block_range (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" UInt64,
    "call_ordinal" INT,
    "call_success" BOOL,
    "u_block_range" UInt256
) ENGINE = MergeTree PRIMARY KEY ("call_tx_hash","call_ordinal");
CREATE TABLE IF NOT EXISTS source_call_set_governor (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" UInt64,
    "call_ordinal" INT,
    "call_success" BOOL,
    "u_governor" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("call_tx_hash","call_ordinal");
CREATE TABLE IF NOT EXISTS source_call_set_oracle (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" UInt64,
    "call_ordinal" INT,
    "call_success" BOOL,
    "approved" BOOL,
    "oracle" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("call_tx_hash","call_ordinal");
CREATE TABLE IF NOT EXISTS source_call_set_protocol_fee (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" UInt64,
    "call_ordinal" INT,
    "call_success" BOOL,
    "rate" UInt16,
    "recipient" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("call_tx_hash","call_ordinal");
CREATE TABLE IF NOT EXISTS source_call_take_ask (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" UInt64,
    "call_ordinal" INT,
    "call_success" BOOL,
    "oracle_signature" TEXT
) ENGINE = MergeTree PRIMARY KEY ("call_tx_hash","call_ordinal");
CREATE TABLE IF NOT EXISTS source_call_take_ask_pool (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" UInt64,
    "call_ordinal" INT,
    "call_success" BOOL,
    "amount_to_withdraw" UInt256,
    "oracle_signature" TEXT
) ENGINE = MergeTree PRIMARY KEY ("call_tx_hash","call_ordinal");
CREATE TABLE IF NOT EXISTS source_call_take_ask_single (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" UInt64,
    "call_ordinal" INT,
    "call_success" BOOL,
    "oracle_signature" TEXT
) ENGINE = MergeTree PRIMARY KEY ("call_tx_hash","call_ordinal");
CREATE TABLE IF NOT EXISTS source_call_take_ask_single_pool (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" UInt64,
    "call_ordinal" INT,
    "call_success" BOOL,
    "amount_to_withdraw" UInt256,
    "oracle_signature" TEXT
) ENGINE = MergeTree PRIMARY KEY ("call_tx_hash","call_ordinal");
CREATE TABLE IF NOT EXISTS source_call_take_bid (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" UInt64,
    "call_ordinal" INT,
    "call_success" BOOL,
    "oracle_signature" TEXT
) ENGINE = MergeTree PRIMARY KEY ("call_tx_hash","call_ordinal");
CREATE TABLE IF NOT EXISTS source_call_take_bid_single (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" UInt64,
    "call_ordinal" INT,
    "call_success" BOOL,
    "oracle_signature" TEXT
) ENGINE = MergeTree PRIMARY KEY ("call_tx_hash","call_ordinal");
CREATE TABLE IF NOT EXISTS source_call_transfer_ownership (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" UInt64,
    "call_ordinal" INT,
    "call_success" BOOL,
    "new_owner" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("call_tx_hash","call_ordinal");
CREATE TABLE IF NOT EXISTS source_call_upgrade_to (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" UInt64,
    "call_ordinal" INT,
    "call_success" BOOL,
    "new_implementation" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("call_tx_hash","call_ordinal");
CREATE TABLE IF NOT EXISTS source_call_upgrade_to_and_call (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" UInt64,
    "call_ordinal" INT,
    "call_success" BOOL,
    "data" TEXT,
    "new_implementation" VARCHAR(40)
) ENGINE = MergeTree PRIMARY KEY ("call_tx_hash","call_ordinal");
