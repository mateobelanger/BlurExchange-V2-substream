mod abi;
mod pb;
use hex_literal::hex;
use pb::contract::v1 as contract;
use substreams::Hex;
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_database_change::tables::Tables as DatabaseChangeTables;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables as EntityChangesTables;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;
use std::str::FromStr;
use substreams::scalar::BigDecimal;

substreams_ethereum::init!();

const EXCHANGEV2_TRACKED_CONTRACT: [u8; 20] = hex!("b2ecfe4e4d61f8790bbb9de2d1259b9e2410cea5");

fn map_exchangev2_events(blk: &eth::Block, events: &mut contract::Events) {
    events.exchangev2_admin_changed_1s.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == EXCHANGEV2_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::exchangev2_contract::events::AdminChanged1::match_and_decode(log) {
                        return Some(contract::Exchangev2AdminChanged1 {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_admin: event.new_admin,
                            previous_admin: event.previous_admin,
                        });
                    }

                    None
                })
        })
        .collect());
    events.exchangev2_admin_changed_2s.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == EXCHANGEV2_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::exchangev2_contract::events::AdminChanged2::match_and_decode(log) {
                        return Some(contract::Exchangev2AdminChanged2 {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_admin: event.new_admin,
                            previous_admin: event.previous_admin,
                        });
                    }

                    None
                })
        })
        .collect());
    events.exchangev2_beacon_upgraded_1s.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == EXCHANGEV2_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::exchangev2_contract::events::BeaconUpgraded1::match_and_decode(log) {
                        return Some(contract::Exchangev2BeaconUpgraded1 {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            beacon: event.beacon,
                        });
                    }

                    None
                })
        })
        .collect());
    events.exchangev2_beacon_upgraded_2s.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == EXCHANGEV2_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::exchangev2_contract::events::BeaconUpgraded2::match_and_decode(log) {
                        return Some(contract::Exchangev2BeaconUpgraded2 {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            beacon: event.beacon,
                        });
                    }

                    None
                })
        })
        .collect());
    events.exchangev2_cancel_trades.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == EXCHANGEV2_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::exchangev2_contract::events::CancelTrade::match_and_decode(log) {
                        return Some(contract::Exchangev2CancelTrade {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            amount: event.amount.to_string(),
                            hash: Vec::from(event.hash),
                            index: event.index.to_string(),
                            user: event.user,
                        });
                    }

                    None
                })
        })
        .collect());
    events.exchangev2_executions.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == EXCHANGEV2_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::exchangev2_contract::events::Execution::match_and_decode(log) {
                        return Some(contract::Exchangev2Execution {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            listing_index: event.listing_index.to_string(),
                            order_hash: Vec::from(event.order_hash),
                            order_type: event.order_type.to_u64(),
                            price: event.price.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.exchangev2_execution721_maker_fee_packeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == EXCHANGEV2_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::exchangev2_contract::events::Execution721MakerFeePacked::match_and_decode(log) {
                        return Some(contract::Exchangev2Execution721MakerFeePacked {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            collection_price_side: event.collection_price_side.to_string(),
                            maker_fee_recipient_rate: event.maker_fee_recipient_rate.to_string(),
                            order_hash: Vec::from(event.order_hash),
                            token_id_listing_index_trader: event.token_id_listing_index_trader.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.exchangev2_execution721_packeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == EXCHANGEV2_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::exchangev2_contract::events::Execution721Packed::match_and_decode(log) {
                        return Some(contract::Exchangev2Execution721Packed {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            collection_price_side: event.collection_price_side.to_string(),
                            order_hash: Vec::from(event.order_hash),
                            token_id_listing_index_trader: event.token_id_listing_index_trader.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.exchangev2_execution721_taker_fee_packeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == EXCHANGEV2_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::exchangev2_contract::events::Execution721TakerFeePacked::match_and_decode(log) {
                        return Some(contract::Exchangev2Execution721TakerFeePacked {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            collection_price_side: event.collection_price_side.to_string(),
                            order_hash: Vec::from(event.order_hash),
                            taker_fee_recipient_rate: event.taker_fee_recipient_rate.to_string(),
                            token_id_listing_index_trader: event.token_id_listing_index_trader.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.exchangev2_initializeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == EXCHANGEV2_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::exchangev2_contract::events::Initialized::match_and_decode(log) {
                        return Some(contract::Exchangev2Initialized {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            version: event.version.to_u64(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.exchangev2_new_governors.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == EXCHANGEV2_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::exchangev2_contract::events::NewGovernor::match_and_decode(log) {
                        return Some(contract::Exchangev2NewGovernor {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            governor: event.governor,
                        });
                    }

                    None
                })
        })
        .collect());
    events.exchangev2_new_protocol_fees.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == EXCHANGEV2_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::exchangev2_contract::events::NewProtocolFee::match_and_decode(log) {
                        return Some(contract::Exchangev2NewProtocolFee {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            rate: event.rate.to_u64(),
                            recipient: event.recipient,
                        });
                    }

                    None
                })
        })
        .collect());
    events.exchangev2_nonce_incrementeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == EXCHANGEV2_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::exchangev2_contract::events::NonceIncremented::match_and_decode(log) {
                        return Some(contract::Exchangev2NonceIncremented {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_nonce: event.new_nonce.to_string(),
                            user: event.user,
                        });
                    }

                    None
                })
        })
        .collect());
    events.exchangev2_ownership_transfer_starteds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == EXCHANGEV2_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::exchangev2_contract::events::OwnershipTransferStarted::match_and_decode(log) {
                        return Some(contract::Exchangev2OwnershipTransferStarted {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_owner: event.new_owner,
                            previous_owner: event.previous_owner,
                        });
                    }

                    None
                })
        })
        .collect());
    events.exchangev2_ownership_transferreds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == EXCHANGEV2_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::exchangev2_contract::events::OwnershipTransferred::match_and_decode(log) {
                        return Some(contract::Exchangev2OwnershipTransferred {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_owner: event.new_owner,
                            previous_owner: event.previous_owner,
                        });
                    }

                    None
                })
        })
        .collect());
    events.exchangev2_set_oracles.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == EXCHANGEV2_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::exchangev2_contract::events::SetOracle::match_and_decode(log) {
                        return Some(contract::Exchangev2SetOracle {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            approved: event.approved,
                            user: event.user,
                        });
                    }

                    None
                })
        })
        .collect());
    events.exchangev2_upgraded_1s.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == EXCHANGEV2_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::exchangev2_contract::events::Upgraded1::match_and_decode(log) {
                        return Some(contract::Exchangev2Upgraded1 {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            implementation: event.implementation,
                        });
                    }

                    None
                })
        })
        .collect());
    events.exchangev2_upgraded_2s.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == EXCHANGEV2_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::exchangev2_contract::events::Upgraded2::match_and_decode(log) {
                        return Some(contract::Exchangev2Upgraded2 {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            implementation: event.implementation,
                        });
                    }

                    None
                })
        })
        .collect());
}

fn db_exchangev2_out(events: &contract::Events, tables: &mut DatabaseChangeTables) {
    // Loop over all the abis events to create table changes
    events.exchangev2_admin_changed_1s.iter().for_each(|evt| {
        tables
            .create_row("exchangev2_admin_changed1", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("new_admin", Hex(&evt.new_admin).to_string())
            .set("previous_admin", Hex(&evt.previous_admin).to_string());
    });
    events.exchangev2_admin_changed_2s.iter().for_each(|evt| {
        tables
            .create_row("exchangev2_admin_changed2", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("new_admin", Hex(&evt.new_admin).to_string())
            .set("previous_admin", Hex(&evt.previous_admin).to_string());
    });
    events.exchangev2_beacon_upgraded_1s.iter().for_each(|evt| {
        tables
            .create_row("exchangev2_beacon_upgraded1", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("beacon", Hex(&evt.beacon).to_string());
    });
    events.exchangev2_beacon_upgraded_2s.iter().for_each(|evt| {
        tables
            .create_row("exchangev2_beacon_upgraded2", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("beacon", Hex(&evt.beacon).to_string());
    });
    events.exchangev2_cancel_trades.iter().for_each(|evt| {
        tables
            .create_row("exchangev2_cancel_trade", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("amount", BigDecimal::from_str(&evt.amount).unwrap())
            .set("hash", Hex(&evt.hash).to_string())
            .set("index", BigDecimal::from_str(&evt.index).unwrap())
            .set("user", Hex(&evt.user).to_string());
    });
    events.exchangev2_executions.iter().for_each(|evt| {
        tables
            .create_row("exchangev2_execution", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("listing_index", BigDecimal::from_str(&evt.listing_index).unwrap())
            .set("order_hash", Hex(&evt.order_hash).to_string())
            .set("order_type", evt.order_type)
            .set("price", BigDecimal::from_str(&evt.price).unwrap());
    });
    events.exchangev2_execution721_maker_fee_packeds.iter().for_each(|evt| {
        tables
            .create_row("exchangev2_execution721_maker_fee_packed", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("collection_price_side", BigDecimal::from_str(&evt.collection_price_side).unwrap())
            .set("maker_fee_recipient_rate", BigDecimal::from_str(&evt.maker_fee_recipient_rate).unwrap())
            .set("order_hash", Hex(&evt.order_hash).to_string())
            .set("token_id_listing_index_trader", BigDecimal::from_str(&evt.token_id_listing_index_trader).unwrap());
    });
    events.exchangev2_execution721_packeds.iter().for_each(|evt| {
        tables
            .create_row("exchangev2_execution721_packed", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("collection_price_side", BigDecimal::from_str(&evt.collection_price_side).unwrap())
            .set("order_hash", Hex(&evt.order_hash).to_string())
            .set("token_id_listing_index_trader", BigDecimal::from_str(&evt.token_id_listing_index_trader).unwrap());
    });
    events.exchangev2_execution721_taker_fee_packeds.iter().for_each(|evt| {
        tables
            .create_row("exchangev2_execution721_taker_fee_packed", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("collection_price_side", BigDecimal::from_str(&evt.collection_price_side).unwrap())
            .set("order_hash", Hex(&evt.order_hash).to_string())
            .set("taker_fee_recipient_rate", BigDecimal::from_str(&evt.taker_fee_recipient_rate).unwrap())
            .set("token_id_listing_index_trader", BigDecimal::from_str(&evt.token_id_listing_index_trader).unwrap());
    });
    events.exchangev2_initializeds.iter().for_each(|evt| {
        tables
            .create_row("exchangev2_initialized", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("version", evt.version);
    });

    events.exchangev2_new_governors.iter().for_each(|evt| {
        tables
            .create_row("exchangev2_new_governor", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("governor", Hex(&evt.governor).to_string());
    });
    events.exchangev2_new_protocol_fees.iter().for_each(|evt| {
        tables
            .create_row("exchangev2_new_protocol_fee", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("rate", evt.rate)
            .set("recipient", Hex(&evt.recipient).to_string());
    });
    events.exchangev2_nonce_incrementeds.iter().for_each(|evt| {
        tables
            .create_row("exchangev2_nonce_incremented", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("new_nonce", BigDecimal::from_str(&evt.new_nonce).unwrap())
            .set("user", Hex(&evt.user).to_string());
    });
    events.exchangev2_ownership_transfer_starteds.iter().for_each(|evt| {
        tables
            .create_row("exchangev2_ownership_transfer_started", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("new_owner", Hex(&evt.new_owner).to_string())
            .set("previous_owner", Hex(&evt.previous_owner).to_string());
    });
    events.exchangev2_ownership_transferreds.iter().for_each(|evt| {
        tables
            .create_row("exchangev2_ownership_transferred", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("new_owner", Hex(&evt.new_owner).to_string())
            .set("previous_owner", Hex(&evt.previous_owner).to_string());
    });
    events.exchangev2_set_oracles.iter().for_each(|evt| {
        tables
            .create_row("exchangev2_set_oracle", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("approved", evt.approved)
            .set("user", Hex(&evt.user).to_string());
    });
    events.exchangev2_upgraded_1s.iter().for_each(|evt| {
        tables
            .create_row("exchangev2_upgraded1", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("implementation", Hex(&evt.implementation).to_string());
    });
    events.exchangev2_upgraded_2s.iter().for_each(|evt| {
        tables
            .create_row("exchangev2_upgraded2", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("implementation", Hex(&evt.implementation).to_string());
    });
}


fn graph_exchangev2_out(events: &contract::Events, tables: &mut EntityChangesTables) {
    // Loop over all the abis events to create table changes
    events.exchangev2_admin_changed_1s.iter().for_each(|evt| {
        tables
            .create_row("exchangev2_admin_changed1", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("new_admin", Hex(&evt.new_admin).to_string())
            .set("previous_admin", Hex(&evt.previous_admin).to_string());
    });
    events.exchangev2_admin_changed_2s.iter().for_each(|evt| {
        tables
            .create_row("exchangev2_admin_changed2", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("new_admin", Hex(&evt.new_admin).to_string())
            .set("previous_admin", Hex(&evt.previous_admin).to_string());
    });
    events.exchangev2_beacon_upgraded_1s.iter().for_each(|evt| {
        tables
            .create_row("exchangev2_beacon_upgraded1", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("beacon", Hex(&evt.beacon).to_string());
    });
    events.exchangev2_beacon_upgraded_2s.iter().for_each(|evt| {
        tables
            .create_row("exchangev2_beacon_upgraded2", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("beacon", Hex(&evt.beacon).to_string());
    });
    events.exchangev2_cancel_trades.iter().for_each(|evt| {
        tables
            .create_row("exchangev2_cancel_trade", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("amount", BigDecimal::from_str(&evt.amount).unwrap())
            .set("hash", Hex(&evt.hash).to_string())
            .set("index", BigDecimal::from_str(&evt.index).unwrap())
            .set("user", Hex(&evt.user).to_string());
    });
    events.exchangev2_executions.iter().for_each(|evt| {
        tables
            .create_row("exchangev2_execution", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("listing_index", BigDecimal::from_str(&evt.listing_index).unwrap())
            .set("order_hash", Hex(&evt.order_hash).to_string())
            .set("order_type", evt.order_type)
            .set("price", BigDecimal::from_str(&evt.price).unwrap());
    });
    events.exchangev2_execution721_maker_fee_packeds.iter().for_each(|evt| {
        tables
            .create_row("exchangev2_execution721_maker_fee_packed", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("collection_price_side", BigDecimal::from_str(&evt.collection_price_side).unwrap())
            .set("maker_fee_recipient_rate", BigDecimal::from_str(&evt.maker_fee_recipient_rate).unwrap())
            .set("order_hash", Hex(&evt.order_hash).to_string())
            .set("token_id_listing_index_trader", BigDecimal::from_str(&evt.token_id_listing_index_trader).unwrap());
    });
    events.exchangev2_execution721_packeds.iter().for_each(|evt| {
        tables
            .create_row("exchangev2_execution721_packed", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("collection_price_side", BigDecimal::from_str(&evt.collection_price_side).unwrap())
            .set("order_hash", Hex(&evt.order_hash).to_string())
            .set("token_id_listing_index_trader", BigDecimal::from_str(&evt.token_id_listing_index_trader).unwrap());
    });
    events.exchangev2_execution721_taker_fee_packeds.iter().for_each(|evt| {
        tables
            .create_row("exchangev2_execution721_taker_fee_packed", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("collection_price_side", BigDecimal::from_str(&evt.collection_price_side).unwrap())
            .set("order_hash", Hex(&evt.order_hash).to_string())
            .set("taker_fee_recipient_rate", BigDecimal::from_str(&evt.taker_fee_recipient_rate).unwrap())
            .set("token_id_listing_index_trader", BigDecimal::from_str(&evt.token_id_listing_index_trader).unwrap());
    });
    events.exchangev2_initializeds.iter().for_each(|evt| {
        tables
            .create_row("exchangev2_initialized", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("version", evt.version);
    });

    events.exchangev2_new_governors.iter().for_each(|evt| {
        tables
            .create_row("exchangev2_new_governor", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("governor", Hex(&evt.governor).to_string());
    });
    events.exchangev2_new_protocol_fees.iter().for_each(|evt| {
        tables
            .create_row("exchangev2_new_protocol_fee", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("rate", evt.rate)
            .set("recipient", Hex(&evt.recipient).to_string());
    });
    events.exchangev2_nonce_incrementeds.iter().for_each(|evt| {
        tables
            .create_row("exchangev2_nonce_incremented", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("new_nonce", BigDecimal::from_str(&evt.new_nonce).unwrap())
            .set("user", Hex(&evt.user).to_string());
    });
    events.exchangev2_ownership_transfer_starteds.iter().for_each(|evt| {
        tables
            .create_row("exchangev2_ownership_transfer_started", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("new_owner", Hex(&evt.new_owner).to_string())
            .set("previous_owner", Hex(&evt.previous_owner).to_string());
    });
    events.exchangev2_ownership_transferreds.iter().for_each(|evt| {
        tables
            .create_row("exchangev2_ownership_transferred", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("new_owner", Hex(&evt.new_owner).to_string())
            .set("previous_owner", Hex(&evt.previous_owner).to_string());
    });
    events.exchangev2_set_oracles.iter().for_each(|evt| {
        tables
            .create_row("exchangev2_set_oracle", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("approved", evt.approved)
            .set("user", Hex(&evt.user).to_string());
    });
    events.exchangev2_upgraded_1s.iter().for_each(|evt| {
        tables
            .create_row("exchangev2_upgraded1", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("implementation", Hex(&evt.implementation).to_string());
    });
    events.exchangev2_upgraded_2s.iter().for_each(|evt| {
        tables
            .create_row("exchangev2_upgraded2", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("implementation", Hex(&evt.implementation).to_string());
    });
}

#[substreams::handlers::map]
fn map_events(blk: eth::Block) -> Result<contract::Events, substreams::errors::Error> {
    let mut events = contract::Events::default();
    map_exchangev2_events(&blk, &mut events);
    Ok(events)
}
#[substreams::handlers::map]
fn map_calls(blk: eth::Block) -> Result<contract::Calls, substreams::errors::Error> {
    let mut calls = contract::Calls::default();
    Ok(calls)
}

#[substreams::handlers::map]
fn db_out(events: contract::Events, calls: contract::Calls) -> Result<DatabaseChanges, substreams::errors::Error> {
    // Initialize Database Changes container
    let mut tables = DatabaseChangeTables::new();
    db_exchangev2_out(&events, &mut tables);
    Ok(tables.to_database_changes())
}

#[substreams::handlers::map]
fn graph_out(events: contract::Events, calls: contract::Calls) -> Result<EntityChanges, substreams::errors::Error> {
    // Initialize Database Changes container
    let mut tables = EntityChangesTables::new();
    graph_exchangev2_out(&events, &mut tables);
    Ok(tables.to_entity_changes())
}
