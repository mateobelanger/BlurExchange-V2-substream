CREATE TABLE IF NOT EXISTS proxy_admin_changed1 (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "new_admin" VARCHAR(40),
    "previous_admin" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS proxy_admin_changed2 (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "new_admin" VARCHAR(40),
    "previous_admin" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS proxy_beacon_upgraded1 (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "beacon" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS proxy_beacon_upgraded2 (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "beacon" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS proxy_cancel_trade (
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
CREATE TABLE IF NOT EXISTS proxy_execution (
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
CREATE TABLE IF NOT EXISTS proxy_execution721_maker_fee_packed (
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
CREATE TABLE IF NOT EXISTS proxy_execution721_packed (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "collection_price_side" DECIMAL,
    "order_hash" TEXT,
    "token_id_listing_index_trader" DECIMAL,
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS proxy_execution721_taker_fee_packed (
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
CREATE TABLE IF NOT EXISTS proxy_initialized (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "version" INT,
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS proxy_new_block_range (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "block_range" DECIMAL,
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS proxy_new_governor (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "governor" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS proxy_new_protocol_fee (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "rate" INT,
    "recipient" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS proxy_nonce_incremented (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "new_nonce" DECIMAL,
    "user" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS proxy_ownership_transfer_started (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "new_owner" VARCHAR(40),
    "previous_owner" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS proxy_ownership_transferred (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "new_owner" VARCHAR(40),
    "previous_owner" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS proxy_set_oracle (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "approved" BOOL,
    "user" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS proxy_upgraded1 (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "implementation" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS proxy_upgraded2 (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "implementation" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS source_admin_changed (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "new_admin" VARCHAR(40),
    "previous_admin" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS source_beacon_upgraded (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "beacon" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS source_cancel_trade (
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
CREATE TABLE IF NOT EXISTS source_execution (
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
CREATE TABLE IF NOT EXISTS source_execution721_maker_fee_packed (
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
CREATE TABLE IF NOT EXISTS source_execution721_packed (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "collection_price_side" DECIMAL,
    "order_hash" TEXT,
    "token_id_listing_index_trader" DECIMAL,
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS source_execution721_taker_fee_packed (
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
CREATE TABLE IF NOT EXISTS source_initialized (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "version" INT,
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS source_new_block_range (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "block_range" DECIMAL,
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS source_new_governor (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "governor" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS source_new_protocol_fee (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "rate" INT,
    "recipient" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS source_nonce_incremented (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "new_nonce" DECIMAL,
    "user" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS source_ownership_transfer_started (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "new_owner" VARCHAR(40),
    "previous_owner" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS source_ownership_transferred (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "new_owner" VARCHAR(40),
    "previous_owner" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS source_set_oracle (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "approved" BOOL,
    "user" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS source_upgraded (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "implementation" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS source_call_accept_ownership (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS source_call_cancel_trades (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS source_call_increment_nonce (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS source_call_initialize (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS source_call_renounce_ownership (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS source_call_set_block_range (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "u_block_range" DECIMAL,
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS source_call_set_governor (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "u_governor" VARCHAR(40),
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS source_call_set_oracle (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "approved" BOOL,
    "oracle" VARCHAR(40),
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS source_call_set_protocol_fee (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "rate" INT,
    "recipient" VARCHAR(40),
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS source_call_take_ask (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "oracle_signature" TEXT,
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS source_call_take_ask_pool (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "amount_to_withdraw" DECIMAL,
    "oracle_signature" TEXT,
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS source_call_take_ask_single (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "oracle_signature" TEXT,
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS source_call_take_ask_single_pool (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "amount_to_withdraw" DECIMAL,
    "oracle_signature" TEXT,
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS source_call_take_bid (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "oracle_signature" TEXT,
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS source_call_take_bid_single (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "oracle_signature" TEXT,
    "taker_fee" DECIMAL,
    "signatures" TEXT,
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS source_call_transfer_ownership (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "new_owner" VARCHAR(40),
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS source_call_upgrade_to (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "new_implementation" VARCHAR(40),
    PRIMARY KEY(call_tx_hash,call_ordinal)
);
CREATE TABLE IF NOT EXISTS source_call_upgrade_to_and_call (
    "call_tx_hash" VARCHAR(64),
    "call_block_time" TIMESTAMP,
    "call_block_number" DECIMAL,
    "call_ordinal" INT,
    "call_success" BOOL,
    "data" TEXT,
    "new_implementation" VARCHAR(40),
    PRIMARY KEY(call_tx_hash,call_ordinal)
);

CREATE TABLE IF NOT EXISTS order (
    "order_id" INT AUTO_INCREMENT,
    "call_tx_hash" VARCHAR(64),
    "call_ordinal" INT,
    -- other fields from Order...
    PRIMARY KEY(order_id),
    FOREIGN KEY(call_tx_hash, call_ordinal) REFERENCES source_call_take_bid(call_tx_hash, call_ordinal)
);

CREATE TABLE IF NOT EXISTS exchange (
    "exchange_id" INT AUTO_INCREMENT,
    "call_tx_hash" VARCHAR(64),
    "call_ordinal" INT,
    -- other fields from Exchange...
    PRIMARY KEY(exchange_id),
    FOREIGN KEY(call_tx_hash, call_ordinal) REFERENCES source_call_take_bid(call_tx_hash, call_ordinal)
);
