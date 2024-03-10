
use crate::pb::custom::v1 as contract;
use substreams::Hex;
use substreams_database_change::tables::Tables as DatabaseChangeTables;

use std::str::FromStr;
use substreams::scalar::BigDecimal;




pub fn db_proxy_out(events: &contract::Events, tables: &mut DatabaseChangeTables) {
    // Loop over all the abis events to create table changes
    events.proxy_admin_changed_1s.iter().for_each(|evt| {
        tables
            .create_row(
                "proxy_admin_changed1",
                [
                    ("evt_tx_hash", evt.evt_tx_hash.to_string()),
                    ("evt_index", evt.evt_index.to_string()),
                ],
            )
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("new_admin", Hex(&evt.new_admin).to_string())
            .set("previous_admin", Hex(&evt.previous_admin).to_string());
    });
    events.proxy_admin_changed_2s.iter().for_each(|evt| {
        tables
            .create_row(
                "proxy_admin_changed2",
                [
                    ("evt_tx_hash", evt.evt_tx_hash.to_string()),
                    ("evt_index", evt.evt_index.to_string()),
                ],
            )
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("new_admin", Hex(&evt.new_admin).to_string())
            .set("previous_admin", Hex(&evt.previous_admin).to_string());
    });
    events.proxy_beacon_upgraded_1s.iter().for_each(|evt| {
        tables
            .create_row(
                "proxy_beacon_upgraded1",
                [
                    ("evt_tx_hash", evt.evt_tx_hash.to_string()),
                    ("evt_index", evt.evt_index.to_string()),
                ],
            )
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("beacon", Hex(&evt.beacon).to_string());
    });
    events.proxy_beacon_upgraded_2s.iter().for_each(|evt| {
        tables
            .create_row(
                "proxy_beacon_upgraded2",
                [
                    ("evt_tx_hash", evt.evt_tx_hash.to_string()),
                    ("evt_index", evt.evt_index.to_string()),
                ],
            )
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("beacon", Hex(&evt.beacon).to_string());
    });
    events.proxy_cancel_trades.iter().for_each(|evt| {
        tables
            .create_row(
                "proxy_cancel_trade",
                [
                    ("evt_tx_hash", evt.evt_tx_hash.to_string()),
                    ("evt_index", evt.evt_index.to_string()),
                ],
            )
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("amount", BigDecimal::from_str(&evt.amount).unwrap())
            .set("hash", Hex(&evt.hash).to_string())
            .set("index", BigDecimal::from_str(&evt.index).unwrap())
            .set("user", Hex(&evt.user).to_string());
    });
    events.proxy_executions.iter().for_each(|evt| {
        tables
            .create_row(
                "proxy_execution",
                [
                    ("evt_tx_hash", evt.evt_tx_hash.to_string()),
                    ("evt_index", evt.evt_index.to_string()),
                ],
            )
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set(
                "listing_index",
                BigDecimal::from_str(&evt.listing_index).unwrap(),
            )
            .set("order_hash", Hex(&evt.order_hash).to_string())
            .set("order_type", evt.order_type)
            .set("price", BigDecimal::from_str(&evt.price).unwrap());
    });
    events
        .proxy_execution721_maker_fee_packeds
        .iter()
        .for_each(|evt| {
            tables
                .create_row(
                    "proxy_execution721_maker_fee_packed",
                    [
                        ("evt_tx_hash", evt.evt_tx_hash.to_string()),
                        ("evt_index", evt.evt_index.to_string()),
                    ],
                )
                .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
                .set("evt_block_number", evt.evt_block_number)
                .set(
                    "collection_price_side",
                    BigDecimal::from_str(&evt.collection_price_side).unwrap(),
                )
                .set(
                    "maker_fee_recipient_rate",
                    BigDecimal::from_str(&evt.maker_fee_recipient_rate).unwrap(),
                )
                .set("order_hash", Hex(&evt.order_hash).to_string())
                .set(
                    "token_id_listing_index_trader",
                    BigDecimal::from_str(&evt.token_id_listing_index_trader).unwrap(),
                );
        });
    events.proxy_execution721_packeds.iter().for_each(|evt| {
        tables
            .create_row(
                "proxy_execution721_packed",
                [
                    ("evt_tx_hash", evt.evt_tx_hash.to_string()),
                    ("evt_index", evt.evt_index.to_string()),
                ],
            )
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set(
                "collection_price_side",
                BigDecimal::from_str(&evt.collection_price_side).unwrap(),
            )
            .set("order_hash", Hex(&evt.order_hash).to_string())
            .set(
                "token_id_listing_index_trader",
                BigDecimal::from_str(&evt.token_id_listing_index_trader).unwrap(),
            );
    });
    events
        .proxy_execution721_taker_fee_packeds
        .iter()
        .for_each(|evt| {
            tables
                .create_row(
                    "proxy_execution721_taker_fee_packed",
                    [
                        ("evt_tx_hash", evt.evt_tx_hash.to_string()),
                        ("evt_index", evt.evt_index.to_string()),
                    ],
                )
                .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
                .set("evt_block_number", evt.evt_block_number)
                .set(
                    "collection_price_side",
                    BigDecimal::from_str(&evt.collection_price_side).unwrap(),
                )
                .set("order_hash", Hex(&evt.order_hash).to_string())
                .set(
                    "taker_fee_recipient_rate",
                    BigDecimal::from_str(&evt.taker_fee_recipient_rate).unwrap(),
                )
                .set(
                    "token_id_listing_index_trader",
                    BigDecimal::from_str(&evt.token_id_listing_index_trader).unwrap(),
                );
        });
    events.proxy_initializeds.iter().for_each(|evt| {
        tables
            .create_row(
                "proxy_initialized",
                [
                    ("evt_tx_hash", evt.evt_tx_hash.to_string()),
                    ("evt_index", evt.evt_index.to_string()),
                ],
            )
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("version", evt.version);
    });
    events.proxy_new_block_ranges.iter().for_each(|evt| {
        tables
            .create_row(
                "proxy_new_block_range",
                [
                    ("evt_tx_hash", evt.evt_tx_hash.to_string()),
                    ("evt_index", evt.evt_index.to_string()),
                ],
            )
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set(
                "block_range",
                BigDecimal::from_str(&evt.block_range).unwrap(),
            );
    });
    events.proxy_new_governors.iter().for_each(|evt| {
        tables
            .create_row(
                "proxy_new_governor",
                [
                    ("evt_tx_hash", evt.evt_tx_hash.to_string()),
                    ("evt_index", evt.evt_index.to_string()),
                ],
            )
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("governor", Hex(&evt.governor).to_string());
    });
    events.proxy_new_protocol_fees.iter().for_each(|evt| {
        tables
            .create_row(
                "proxy_new_protocol_fee",
                [
                    ("evt_tx_hash", evt.evt_tx_hash.to_string()),
                    ("evt_index", evt.evt_index.to_string()),
                ],
            )
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("rate", evt.rate)
            .set("recipient", Hex(&evt.recipient).to_string());
    });
    events.proxy_nonce_incrementeds.iter().for_each(|evt| {
        tables
            .create_row(
                "proxy_nonce_incremented",
                [
                    ("evt_tx_hash", evt.evt_tx_hash.to_string()),
                    ("evt_index", evt.evt_index.to_string()),
                ],
            )
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("new_nonce", BigDecimal::from_str(&evt.new_nonce).unwrap())
            .set("user", Hex(&evt.user).to_string());
    });
    events
        .proxy_ownership_transfer_starteds
        .iter()
        .for_each(|evt| {
            tables
                .create_row(
                    "proxy_ownership_transfer_started",
                    [
                        ("evt_tx_hash", evt.evt_tx_hash.to_string()),
                        ("evt_index", evt.evt_index.to_string()),
                    ],
                )
                .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
                .set("evt_block_number", evt.evt_block_number)
                .set("new_owner", Hex(&evt.new_owner).to_string())
                .set("previous_owner", Hex(&evt.previous_owner).to_string());
        });
    events.proxy_ownership_transferreds.iter().for_each(|evt| {
        tables
            .create_row(
                "proxy_ownership_transferred",
                [
                    ("evt_tx_hash", evt.evt_tx_hash.to_string()),
                    ("evt_index", evt.evt_index.to_string()),
                ],
            )
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("new_owner", Hex(&evt.new_owner).to_string())
            .set("previous_owner", Hex(&evt.previous_owner).to_string());
    });
    events.proxy_set_oracles.iter().for_each(|evt| {
        tables
            .create_row(
                "proxy_set_oracle",
                [
                    ("evt_tx_hash", evt.evt_tx_hash.to_string()),
                    ("evt_index", evt.evt_index.to_string()),
                ],
            )
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("approved", evt.approved)
            .set("user", Hex(&evt.user).to_string());
    });
    events.proxy_upgraded_1s.iter().for_each(|evt| {
        tables
            .create_row(
                "proxy_upgraded1",
                [
                    ("evt_tx_hash", evt.evt_tx_hash.to_string()),
                    ("evt_index", evt.evt_index.to_string()),
                ],
            )
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("implementation", Hex(&evt.implementation).to_string());
    });
    events.proxy_upgraded_2s.iter().for_each(|evt| {
        tables
            .create_row(
                "proxy_upgraded2",
                [
                    ("evt_tx_hash", evt.evt_tx_hash.to_string()),
                    ("evt_index", evt.evt_index.to_string()),
                ],
            )
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("implementation", Hex(&evt.implementation).to_string());
    });
}
pub fn db_source_out(events: &contract::Events, tables: &mut DatabaseChangeTables) {
    // Loop over all the abis events to create table changes
    events.source_admin_changeds.iter().for_each(|evt| {
        tables
            .create_row(
                "source_admin_changed",
                [
                    ("evt_tx_hash", evt.evt_tx_hash.to_string()),
                    ("evt_index", evt.evt_index.to_string()),
                ],
            )
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("new_admin", Hex(&evt.new_admin).to_string())
            .set("previous_admin", Hex(&evt.previous_admin).to_string());
    });
    events.source_beacon_upgradeds.iter().for_each(|evt| {
        tables
            .create_row(
                "source_beacon_upgraded",
                [
                    ("evt_tx_hash", evt.evt_tx_hash.to_string()),
                    ("evt_index", evt.evt_index.to_string()),
                ],
            )
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("beacon", Hex(&evt.beacon).to_string());
    });
    events.source_cancel_trades.iter().for_each(|evt| {
        tables
            .create_row(
                "source_cancel_trade",
                [
                    ("evt_tx_hash", evt.evt_tx_hash.to_string()),
                    ("evt_index", evt.evt_index.to_string()),
                ],
            )
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("amount", BigDecimal::from_str(&evt.amount).unwrap())
            .set("hash", Hex(&evt.hash).to_string())
            .set("index", BigDecimal::from_str(&evt.index).unwrap())
            .set("user", Hex(&evt.user).to_string());
    });
    events.source_executions.iter().for_each(|evt| {
        tables
            .create_row(
                "source_execution",
                [
                    ("evt_tx_hash", evt.evt_tx_hash.to_string()),
                    ("evt_index", evt.evt_index.to_string()),
                ],
            )
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set(
                "listing_index",
                BigDecimal::from_str(&evt.listing_index).unwrap(),
            )
            .set("order_hash", Hex(&evt.order_hash).to_string())
            .set("order_type", evt.order_type)
            .set("price", BigDecimal::from_str(&evt.price).unwrap());
    });
    events
        .source_execution721_maker_fee_packeds
        .iter()
        .for_each(|evt| {
            tables
                .create_row(
                    "source_execution721_maker_fee_packed",
                    [
                        ("evt_tx_hash", evt.evt_tx_hash.to_string()),
                        ("evt_index", evt.evt_index.to_string()),
                    ],
                )
                .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
                .set("evt_block_number", evt.evt_block_number)
                .set(
                    "collection_price_side",
                    BigDecimal::from_str(&evt.collection_price_side).unwrap(),
                )
                .set(
                    "maker_fee_recipient_rate",
                    BigDecimal::from_str(&evt.maker_fee_recipient_rate).unwrap(),
                )
                .set("order_hash", Hex(&evt.order_hash).to_string())
                .set(
                    "token_id_listing_index_trader",
                    BigDecimal::from_str(&evt.token_id_listing_index_trader).unwrap(),
                );
        });
    events.source_execution721_packeds.iter().for_each(|evt| {
        tables
            .create_row(
                "source_execution721_packed",
                [
                    ("evt_tx_hash", evt.evt_tx_hash.to_string()),
                    ("evt_index", evt.evt_index.to_string()),
                ],
            )
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set(
                "collection_price_side",
                BigDecimal::from_str(&evt.collection_price_side).unwrap(),
            )
            .set("order_hash", Hex(&evt.order_hash).to_string())
            .set(
                "token_id_listing_index_trader",
                BigDecimal::from_str(&evt.token_id_listing_index_trader).unwrap(),
            );
    });
    events
        .source_execution721_taker_fee_packeds
        .iter()
        .for_each(|evt| {
            tables
                .create_row(
                    "source_execution721_taker_fee_packed",
                    [
                        ("evt_tx_hash", evt.evt_tx_hash.to_string()),
                        ("evt_index", evt.evt_index.to_string()),
                    ],
                )
                .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
                .set("evt_block_number", evt.evt_block_number)
                .set(
                    "collection_price_side",
                    BigDecimal::from_str(&evt.collection_price_side).unwrap(),
                )
                .set("order_hash", Hex(&evt.order_hash).to_string())
                .set(
                    "taker_fee_recipient_rate",
                    BigDecimal::from_str(&evt.taker_fee_recipient_rate).unwrap(),
                )
                .set(
                    "token_id_listing_index_trader",
                    BigDecimal::from_str(&evt.token_id_listing_index_trader).unwrap(),
                );
        });
    events.source_initializeds.iter().for_each(|evt| {
        tables
            .create_row(
                "source_initialized",
                [
                    ("evt_tx_hash", evt.evt_tx_hash.to_string()),
                    ("evt_index", evt.evt_index.to_string()),
                ],
            )
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("version", evt.version);
    });
    events.source_new_block_ranges.iter().for_each(|evt| {
        tables
            .create_row(
                "source_new_block_range",
                [
                    ("evt_tx_hash", evt.evt_tx_hash.to_string()),
                    ("evt_index", evt.evt_index.to_string()),
                ],
            )
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set(
                "block_range",
                BigDecimal::from_str(&evt.block_range).unwrap(),
            );
    });
    events.source_new_governors.iter().for_each(|evt| {
        tables
            .create_row(
                "source_new_governor",
                [
                    ("evt_tx_hash", evt.evt_tx_hash.to_string()),
                    ("evt_index", evt.evt_index.to_string()),
                ],
            )
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("governor", Hex(&evt.governor).to_string());
    });
    events.source_new_protocol_fees.iter().for_each(|evt| {
        tables
            .create_row(
                "source_new_protocol_fee",
                [
                    ("evt_tx_hash", evt.evt_tx_hash.to_string()),
                    ("evt_index", evt.evt_index.to_string()),
                ],
            )
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("rate", evt.rate)
            .set("recipient", Hex(&evt.recipient).to_string());
    });
    events.source_nonce_incrementeds.iter().for_each(|evt| {
        tables
            .create_row(
                "source_nonce_incremented",
                [
                    ("evt_tx_hash", evt.evt_tx_hash.to_string()),
                    ("evt_index", evt.evt_index.to_string()),
                ],
            )
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("new_nonce", BigDecimal::from_str(&evt.new_nonce).unwrap())
            .set("user", Hex(&evt.user).to_string());
    });
    events
        .source_ownership_transfer_starteds
        .iter()
        .for_each(|evt| {
            tables
                .create_row(
                    "source_ownership_transfer_started",
                    [
                        ("evt_tx_hash", evt.evt_tx_hash.to_string()),
                        ("evt_index", evt.evt_index.to_string()),
                    ],
                )
                .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
                .set("evt_block_number", evt.evt_block_number)
                .set("new_owner", Hex(&evt.new_owner).to_string())
                .set("previous_owner", Hex(&evt.previous_owner).to_string());
        });
    events.source_ownership_transferreds.iter().for_each(|evt| {
        tables
            .create_row(
                "source_ownership_transferred",
                [
                    ("evt_tx_hash", evt.evt_tx_hash.to_string()),
                    ("evt_index", evt.evt_index.to_string()),
                ],
            )
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("new_owner", Hex(&evt.new_owner).to_string())
            .set("previous_owner", Hex(&evt.previous_owner).to_string());
    });
    events.source_upgradeds.iter().for_each(|evt| {
        tables
            .create_row(
                "source_upgraded",
                [
                    ("evt_tx_hash", evt.evt_tx_hash.to_string()),
                    ("evt_index", evt.evt_index.to_string()),
                ],
            )
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("implementation", Hex(&evt.implementation).to_string());
    });
}
pub fn db_source_calls_out(calls: &contract::Calls, tables: &mut DatabaseChangeTables) {
    // Loop over all the abis calls to create table changes
    calls.source_call_accept_ownerships.iter().for_each(|call| {
        tables
            .create_row(
                "source_call_accept_ownership",
                [
                    ("call_tx_hash", call.call_tx_hash.to_string()),
                    ("call_ordinal", call.call_ordinal.to_string()),
                ],
            )
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success);
    });
    calls.source_call_cancel_trades.iter().for_each(|call| {
        tables
            .create_row(
                "source_call_cancel_trades",
                [
                    ("call_tx_hash", call.call_tx_hash.to_string()),
                    ("call_ordinal", call.call_ordinal.to_string()),
                ],
            )
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success);
    });
    calls.source_call_increment_nonces.iter().for_each(|call| {
        tables
            .create_row(
                "source_call_increment_nonce",
                [
                    ("call_tx_hash", call.call_tx_hash.to_string()),
                    ("call_ordinal", call.call_ordinal.to_string()),
                ],
            )
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success);
    });
    calls.source_call_initializes.iter().for_each(|call| {
        tables
            .create_row(
                "source_call_initialize",
                [
                    ("call_tx_hash", call.call_tx_hash.to_string()),
                    ("call_ordinal", call.call_ordinal.to_string()),
                ],
            )
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success);
    });
    calls
        .source_call_renounce_ownerships
        .iter()
        .for_each(|call| {
            tables
                .create_row(
                    "source_call_renounce_ownership",
                    [
                        ("call_tx_hash", call.call_tx_hash.to_string()),
                        ("call_ordinal", call.call_ordinal.to_string()),
                    ],
                )
                .set("call_block_time", call.call_block_time.as_ref().unwrap())
                .set("call_block_number", call.call_block_number)
                .set("call_success", call.call_success);
        });
    calls.source_call_set_block_ranges.iter().for_each(|call| {
        tables
            .create_row(
                "source_call_set_block_range",
                [
                    ("call_tx_hash", call.call_tx_hash.to_string()),
                    ("call_ordinal", call.call_ordinal.to_string()),
                ],
            )
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set(
                "u_block_range",
                BigDecimal::from_str(&call.u_block_range).unwrap(),
            );
    });
    calls.source_call_set_governors.iter().for_each(|call| {
        tables
            .create_row(
                "source_call_set_governor",
                [
                    ("call_tx_hash", call.call_tx_hash.to_string()),
                    ("call_ordinal", call.call_ordinal.to_string()),
                ],
            )
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("u_governor", Hex(&call.u_governor).to_string());
    });
    calls.source_call_set_oracles.iter().for_each(|call| {
        tables
            .create_row(
                "source_call_set_oracle",
                [
                    ("call_tx_hash", call.call_tx_hash.to_string()),
                    ("call_ordinal", call.call_ordinal.to_string()),
                ],
            )
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("approved", call.approved)
            .set("oracle", Hex(&call.oracle).to_string());
    });
    calls.source_call_set_protocol_fees.iter().for_each(|call| {
        tables
            .create_row(
                "source_call_set_protocol_fee",
                [
                    ("call_tx_hash", call.call_tx_hash.to_string()),
                    ("call_ordinal", call.call_ordinal.to_string()),
                ],
            )
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("rate", call.rate)
            .set("recipient", Hex(&call.recipient).to_string());
    });
    calls.source_call_take_asks.iter().for_each(|call| {
        tables
            .create_row(
                "source_call_take_ask",
                [
                    ("call_tx_hash", call.call_tx_hash.to_string()),
                    ("call_ordinal", call.call_ordinal.to_string()),
                ],
            )
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("oracle_signature", Hex(&call.oracle_signature).to_string());
    });
    calls.source_call_take_ask_pools.iter().for_each(|call| {
        tables
            .create_row(
                "source_call_take_ask_pool",
                [
                    ("call_tx_hash", call.call_tx_hash.to_string()),
                    ("call_ordinal", call.call_ordinal.to_string()),
                ],
            )
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set(
                "amount_to_withdraw",
                BigDecimal::from_str(&call.amount_to_withdraw).unwrap(),
            )
            .set("oracle_signature", Hex(&call.oracle_signature).to_string());
    });
    calls.source_call_take_ask_singles.iter().for_each(|call| {
        tables
            .create_row(
                "source_call_take_ask_single",
                [
                    ("call_tx_hash", call.call_tx_hash.to_string()),
                    ("call_ordinal", call.call_ordinal.to_string()),
                ],
            )
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("oracle_signature", Hex(&call.oracle_signature).to_string());
    });
    calls
        .source_call_take_ask_single_pools
        .iter()
        .for_each(|call| {
            tables
                .create_row(
                    "source_call_take_ask_single_pool",
                    [
                        ("call_tx_hash", call.call_tx_hash.to_string()),
                        ("call_ordinal", call.call_ordinal.to_string()),
                    ],
                )
                .set("call_block_time", call.call_block_time.as_ref().unwrap())
                .set("call_block_number", call.call_block_number)
                .set("call_success", call.call_success)
                .set(
                    "amount_to_withdraw",
                    BigDecimal::from_str(&call.amount_to_withdraw).unwrap(),
                )
                .set("oracle_signature", Hex(&call.oracle_signature).to_string());
        });
    calls.source_call_take_bids.iter().for_each(|call| {
        tables
            .create_row(
                "source_call_take_bid",
                [
                    ("call_tx_hash", call.call_tx_hash.to_string()),
                    ("call_ordinal", call.call_ordinal.to_string()),
                ],
            )
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("oracle_signature", Hex(&call.oracle_signature).to_string());
    });
    calls.source_call_take_bid_singles.iter().for_each(|call| {
        tables
            .create_row(
                "source_call_take_bid_single",
                [
                    ("call_tx_hash", call.call_tx_hash.to_string()),
                    ("call_ordinal", call.call_ordinal.to_string()),
                ],
            )
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("oracle_signature", Hex(&call.oracle_signature).to_string());
    });
    calls
        .source_call_transfer_ownerships
        .iter()
        .for_each(|call| {
            tables
                .create_row(
                    "source_call_transfer_ownership",
                    [
                        ("call_tx_hash", call.call_tx_hash.to_string()),
                        ("call_ordinal", call.call_ordinal.to_string()),
                    ],
                )
                .set("call_block_time", call.call_block_time.as_ref().unwrap())
                .set("call_block_number", call.call_block_number)
                .set("call_success", call.call_success)
                .set("new_owner", Hex(&call.new_owner).to_string());
        });
    calls.source_call_upgrade_tos.iter().for_each(|call| {
        tables
            .create_row(
                "source_call_upgrade_to",
                [
                    ("call_tx_hash", call.call_tx_hash.to_string()),
                    ("call_ordinal", call.call_ordinal.to_string()),
                ],
            )
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set(
                "new_implementation",
                Hex(&call.new_implementation).to_string(),
            );
    });
    calls
        .source_call_upgrade_to_and_calls
        .iter()
        .for_each(|call| {
            tables
                .create_row(
                    "source_call_upgrade_to_and_call",
                    [
                        ("call_tx_hash", call.call_tx_hash.to_string()),
                        ("call_ordinal", call.call_ordinal.to_string()),
                    ],
                )
                .set("call_block_time", call.call_block_time.as_ref().unwrap())
                .set("call_block_number", call.call_block_number)
                .set("call_success", call.call_success)
                .set("data", Hex(&call.data).to_string())
                .set(
                    "new_implementation",
                    Hex(&call.new_implementation).to_string(),
                );
        });
}
