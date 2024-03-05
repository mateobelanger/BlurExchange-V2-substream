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
use substreams_ethereum::pb::eth::v2::Block;

substreams_ethereum::init!();

use crate::pb::contract::v1::{Calls, Transfer};
use abi::structs_mapping::*;

const PROXY_TRACKED_CONTRACT: [u8; 20] = hex!("b2ecfe4e4d61f8790bbb9de2d1259b9e2410cea5");
const SOURCE_TRACKED_CONTRACT: [u8; 20] = hex!("5fa60726e62c50af45ff2f6280c468da438a7837");

fn map_proxy_events(blk: &eth::Block, events: &mut contract::Events) {
    events.proxy_executions.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == PROXY_TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::proxy_contract::events::Execution::match_and_decode(log)
                        {
                            return Some(contract::ProxyExecution {
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
            .collect(),
    );
    events.proxy_execution721_maker_fee_packeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PROXY_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::proxy_contract::events::Execution721MakerFeePacked::match_and_decode(log) {
                        return Some(contract::ProxyExecution721MakerFeePacked {
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
    events.proxy_execution721_packeds.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == PROXY_TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::proxy_contract::events::Execution721Packed::match_and_decode(log)
                        {
                            return Some(contract::ProxyExecution721Packed {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                collection_price_side: event.collection_price_side.to_string(),
                                order_hash: Vec::from(event.order_hash),
                                token_id_listing_index_trader: event
                                    .token_id_listing_index_trader
                                    .to_string(),
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
    events.proxy_execution721_taker_fee_packeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PROXY_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::proxy_contract::events::Execution721TakerFeePacked::match_and_decode(log) {
                        return Some(contract::ProxyExecution721TakerFeePacked {
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
    events.proxy_new_protocol_fees.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == PROXY_TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::proxy_contract::events::NewProtocolFee::match_and_decode(log)
                        {
                            return Some(contract::ProxyNewProtocolFee {
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
            .collect(),
    );
    events.proxy_ownership_transfer_starteds.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == PROXY_TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::proxy_contract::events::OwnershipTransferStarted::match_and_decode(
                                log,
                            )
                        {
                            return Some(contract::ProxyOwnershipTransferStarted {
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
            .collect(),
    );
    events.proxy_ownership_transferreds.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == PROXY_TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::proxy_contract::events::OwnershipTransferred::match_and_decode(log)
                        {
                            return Some(contract::ProxyOwnershipTransferred {
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
            .collect(),
    );
}

fn map_source_events(blk: &eth::Block, events: &mut contract::Events) {
    events.source_executions.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == SOURCE_TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::source_contract::events::Execution::match_and_decode(log)
                        {
                            return Some(contract::SourceExecution {
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
            .collect(),
    );
    events.source_execution721_maker_fee_packeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == SOURCE_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::source_contract::events::Execution721MakerFeePacked::match_and_decode(log) {
                        return Some(contract::SourceExecution721MakerFeePacked {
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
    events.source_execution721_packeds.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == SOURCE_TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::source_contract::events::Execution721Packed::match_and_decode(log)
                        {
                            return Some(contract::SourceExecution721Packed {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                collection_price_side: event.collection_price_side.to_string(),
                                order_hash: Vec::from(event.order_hash),
                                token_id_listing_index_trader: event
                                    .token_id_listing_index_trader
                                    .to_string(),
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
    events.source_execution721_taker_fee_packeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == SOURCE_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::source_contract::events::Execution721TakerFeePacked::match_and_decode(log) {
                        return Some(contract::SourceExecution721TakerFeePacked {
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
    events.source_new_protocol_fees.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == SOURCE_TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::source_contract::events::NewProtocolFee::match_and_decode(log)
                        {
                            return Some(contract::SourceNewProtocolFee {
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
            .collect(),
    );
    events.source_ownership_transfer_starteds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == SOURCE_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::source_contract::events::OwnershipTransferStarted::match_and_decode(log) {
                        return Some(contract::SourceOwnershipTransferStarted {
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
    events.source_ownership_transferreds.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == SOURCE_TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::source_contract::events::OwnershipTransferred::match_and_decode(
                                log,
                            )
                        {
                            return Some(contract::SourceOwnershipTransferred {
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
            .collect(),
    );
}

fn map_proxy_calls(blk: &eth::Block, calls: &mut contract::Calls) {
    /*     calls.source_call_take_bids.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PROXY_TRACKED_CONTRACT && abi::source_contract::functions::TakeBid::match_call(call))
                .filter_map(|call| {
                    match abi::proxy_contract::functions::TakeBid::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::SourceTakeBidCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                oracle_signature: decoded_call.oracle_signature,
                                inputs: Some(decoded_call.into()),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.source_call_take_bid_singles.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == PROXY_TRACKED_CONTRACT && abi::source_contract::functions::TakeBidSingle::match_call(call))
                .filter_map(|call| {
                    match abi::source_contract::functions::TakeBidSingle::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::SourceTakeBidSingleCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                oracle_signature: decoded_call.oracle_signature,
                                inputs: Some(decoded_call.into()),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect()); */
}

fn map_source_calls(blk: &eth::Block, calls: &mut contract::Calls) {
    calls.source_call_accept_ownerships.append(
        &mut blk
            .transactions()
            .flat_map(|tx| {
                tx.calls
                    .iter()
                    .filter(|call| {
                        call.address == SOURCE_TRACKED_CONTRACT
                            && abi::source_contract::functions::AcceptOwnership::match_call(call)
                    })
                    .filter_map(|call| {
                        match abi::source_contract::functions::AcceptOwnership::decode(call) {
                            Ok(decoded_call) => Some(contract::SourceAcceptOwnershipCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                            }),
                            Err(_) => None,
                        }
                    })
            })
            .collect(),
    );
    calls.source_call_cancel_trades.append(
        &mut blk
            .transactions()
            .flat_map(|tx| {
                tx.calls
                    .iter()
                    .filter(|call| {
                        call.address == SOURCE_TRACKED_CONTRACT
                            && abi::source_contract::functions::CancelTrades::match_call(call)
                    })
                    .filter_map(
                        |call| match abi::source_contract::functions::CancelTrades::decode(call) {
                            Ok(decoded_call) => Some(contract::SourceCancelTradesCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                            }),
                            Err(_) => None,
                        },
                    )
            })
            .collect(),
    );
    calls.source_call_initializes.append(
        &mut blk
            .transactions()
            .flat_map(|tx| {
                tx.calls
                    .iter()
                    .filter(|call| {
                        call.address == SOURCE_TRACKED_CONTRACT
                            && abi::source_contract::functions::Initialize::match_call(call)
                    })
                    .filter_map(|call| {
                        match abi::source_contract::functions::Initialize::decode(call) {
                            Ok(decoded_call) => Some(contract::SourceInitializeCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                            }),
                            Err(_) => None,
                        }
                    })
            })
            .collect(),
    );
    calls.source_call_renounce_ownerships.append(
        &mut blk
            .transactions()
            .flat_map(|tx| {
                tx.calls
                    .iter()
                    .filter(|call| {
                        call.address == SOURCE_TRACKED_CONTRACT
                            && abi::source_contract::functions::RenounceOwnership::match_call(call)
                    })
                    .filter_map(|call| {
                        match abi::source_contract::functions::RenounceOwnership::decode(call) {
                            Ok(decoded_call) => Some(contract::SourceRenounceOwnershipCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                            }),
                            Err(_) => None,
                        }
                    })
            })
            .collect(),
    );
    calls.source_call_set_protocol_fees.append(
        &mut blk
            .transactions()
            .flat_map(|tx| {
                tx.calls
                    .iter()
                    .filter(|call| {
                        call.address == SOURCE_TRACKED_CONTRACT
                            && abi::source_contract::functions::SetProtocolFee::match_call(call)
                    })
                    .filter_map(
                        |call| match abi::source_contract::functions::SetProtocolFee::decode(call) {
                            Ok(decoded_call) => Some(contract::SourceSetProtocolFeeCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                rate: decoded_call.rate.to_u64(),
                                recipient: decoded_call.recipient,
                            }),
                            Err(_) => None,
                        },
                    )
            })
            .collect(),
    );
    calls.source_call_take_asks.append(
        &mut blk
            .transactions()
            .flat_map(|tx| {
                tx.calls
                    .iter()
                    .filter(|call| {
                        call.address == SOURCE_TRACKED_CONTRACT
                            && abi::source_contract::functions::TakeAsk::match_call(call)
                    })
                    .filter_map(|call| {
                        match abi::source_contract::functions::TakeAsk::decode(call) {
                            Ok(decoded_call) => Some(contract::SourceTakeAskCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                oracle_signature: decoded_call.oracle_signature.clone(),
                            }),
                            Err(_) => None,
                        }
                    })
            })
            .collect(),
    );
    calls.source_call_take_ask_pools.append(
        &mut blk
            .transactions()
            .flat_map(|tx| {
                tx.calls
                    .iter()
                    .filter(|call| {
                        call.address == SOURCE_TRACKED_CONTRACT
                            && abi::source_contract::functions::TakeAskPool::match_call(call)
                    })
                    .filter_map(|call| {
                        match abi::source_contract::functions::TakeAskPool::decode(call) {
                            Ok(decoded_call) => Some(contract::SourceTakeAskPoolCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                amount_to_withdraw: decoded_call.amount_to_withdraw.to_string(),
                                oracle_signature: decoded_call.oracle_signature,
                            }),
                            Err(_) => None,
                        }
                    })
            })
            .collect(),
    );
    calls.source_call_take_ask_singles.append(
        &mut blk
            .transactions()
            .flat_map(|tx| {
                tx.calls
                    .iter()
                    .filter(|call| {
                        call.address == SOURCE_TRACKED_CONTRACT
                            && abi::source_contract::functions::TakeAskSingle::match_call(call)
                    })
                    .filter_map(|call| {
                        substreams::log::info!("LOGGING LOGS");
                        call.logs
                            .iter()
                            .for_each(|log| substreams::log::info!("log: {:?}", log));
                        match abi::source_contract::functions::TakeAskSingle::decode(call) {
                            Ok(decoded_call) => Some(contract::SourceTakeAskSingleCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                oracle_signature: decoded_call.oracle_signature.clone(),
                            }),
                            Err(_) => None,
                        }
                    })
            })
            .collect(),
    );
    calls.source_call_take_ask_single_pools.append(
        &mut blk
            .transactions()
            .flat_map(|tx| {
                tx.calls
                    .iter()
                    .filter(|call| {
                        call.address == SOURCE_TRACKED_CONTRACT
                            && abi::source_contract::functions::TakeAskSinglePool::match_call(call)
                    })
                    .filter_map(|call| {
                        match abi::source_contract::functions::TakeAskSinglePool::decode(call) {
                            Ok(decoded_call) => Some(contract::SourceTakeAskSinglePoolCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                amount_to_withdraw: decoded_call.amount_to_withdraw.to_string(),
                                oracle_signature: decoded_call.oracle_signature,
                            }),
                            Err(_) => None,
                        }
                    })
            })
            .collect(),
    );
    calls.source_call_take_bids.append(
        &mut blk
            .transactions()
            .flat_map(|tx| {
                tx.calls
                    .iter()
                    .filter(|call| {
                        call.address == SOURCE_TRACKED_CONTRACT
                            && abi::source_contract::functions::TakeBid::match_call(call)
                    })
                    .filter_map(|call| {
                        match abi::source_contract::functions::TakeBid::decode(call) {
                            Ok(decoded_call) => Some(contract::SourceTakeBidCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                oracle_signature: decoded_call.oracle_signature.clone(),
                            }),
                            Err(_) => None,
                        }
                    })
            })
            .collect(),
    );
    calls.source_call_take_bid_singles.append(
        &mut blk
            .transactions()
            .flat_map(|tx| {
                tx.calls
                    .iter()
                    .filter(|call| {
                        call.address == SOURCE_TRACKED_CONTRACT
                            && abi::source_contract::functions::TakeBidSingle::match_call(call)
                    })
                    .filter_map(|call| {
                        substreams::log::info!("LOGGING LOGS");
                        call.logs
                            .iter()
                            .for_each(|log| substreams::log::info!("log: {:#?}", log));
                        match abi::source_contract::functions::TakeBidSingle::decode(call) {
                            Ok(decoded_call) => Some(contract::SourceTakeBidSingleCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                oracle_signature: decoded_call.oracle_signature.clone(),
                            }),
                            Err(_) => None,
                        }
                    })
            })
            .collect(),
    );
    calls.source_call_transfer_ownerships.append(
        &mut blk
            .transactions()
            .flat_map(|tx| {
                tx.calls
                    .iter()
                    .filter(|call| {
                        call.address == SOURCE_TRACKED_CONTRACT
                            && abi::source_contract::functions::TransferOwnership::match_call(call)
                    })
                    .filter_map(|call| {
                        match abi::source_contract::functions::TransferOwnership::decode(call) {
                            Ok(decoded_call) => Some(contract::SourceTransferOwnershipCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                new_owner: decoded_call.new_owner,
                            }),
                            Err(_) => None,
                        }
                    })
            })
            .collect(),
    );
    calls.source_call_upgrade_tos.append(
        &mut blk
            .transactions()
            .flat_map(|tx| {
                tx.calls
                    .iter()
                    .filter(|call| {
                        call.address == SOURCE_TRACKED_CONTRACT
                            && abi::source_contract::functions::UpgradeTo::match_call(call)
                    })
                    .filter_map(|call| {
                        match abi::source_contract::functions::UpgradeTo::decode(call) {
                            Ok(decoded_call) => Some(contract::SourceUpgradeToCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                new_implementation: decoded_call.new_implementation,
                            }),
                            Err(_) => None,
                        }
                    })
            })
            .collect(),
    );
    calls.source_call_upgrade_to_and_calls.append(
        &mut blk
            .transactions()
            .flat_map(|tx| {
                tx.calls
                    .iter()
                    .filter(|call| {
                        call.address == SOURCE_TRACKED_CONTRACT
                            && abi::source_contract::functions::UpgradeToAndCall::match_call(call)
                    })
                    .filter_map(|call| {
                        match abi::source_contract::functions::UpgradeToAndCall::decode(call) {
                            Ok(decoded_call) => Some(contract::SourceUpgradeToAndCallCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                data: decoded_call.data,
                                new_implementation: decoded_call.new_implementation,
                            }),
                            Err(_) => None,
                        }
                    })
            })
            .collect(),
    );
}

fn map_source_calls_inputs(blk: &eth::Block, calls: &mut CallsWithInputs) {
    calls.take_asks.append(
        &mut blk
            .transactions()
            .flat_map(|tx| {
                tx.calls
                    .iter()
                    .filter(|call| {
                        call.address == SOURCE_TRACKED_CONTRACT
                            && abi::source_contract::functions::TakeAsk::match_call(call)
                    })
                    .filter_map(|call| {
                        match abi::source_contract::functions::TakeAsk::decode(call) {
                            Ok(decoded_call) => Some(TakeAskExt {
                                call: Some(contract::SourceTakeAskCall {
                                    call_tx_hash: Hex(&tx.hash).to_string(),
                                    call_block_time: Some(blk.timestamp().to_owned()),
                                    call_block_number: blk.number,
                                    call_ordinal: call.begin_ordinal,
                                    call_success: !call.state_reverted,
                                    oracle_signature: decoded_call.oracle_signature.clone(),
                                }),
                                inputs: Some(decoded_call.into()),
                            }),
                            Err(_) => None,
                        }
                    })
            })
            .collect(),
    );
    calls.take_asks_single.append(
        &mut blk
            .transactions()
            .flat_map(|tx| {
                tx.calls
                    .iter()
                    .filter(|call| {
                        call.address == SOURCE_TRACKED_CONTRACT
                            && abi::source_contract::functions::TakeAskSingle::match_call(call)
                    })
                    .filter_map(
                        |call| match abi::source_contract::functions::TakeAskSingle::decode(call) {
                            Ok(decoded_call) => Some(TakeAskSingleExt {
                                call: Some(contract::SourceTakeAskSingleCall {
                                    call_tx_hash: Hex(&tx.hash).to_string(),
                                    call_block_time: Some(blk.timestamp().to_owned()),
                                    call_block_number: blk.number,
                                    call_ordinal: call.begin_ordinal,
                                    call_success: !call.state_reverted,
                                    oracle_signature: decoded_call.oracle_signature.clone(),
                                }),
                                inputs: Some(decoded_call.into()),
                            }),
                            Err(_) => None,
                        },
                    )
            })
            .collect(),
    );

    calls.take_bids.append(
        &mut blk
            .transactions()
            .flat_map(|tx| {
                tx.calls
                    .iter()
                    .filter(|call| {
                        call.address == SOURCE_TRACKED_CONTRACT
                            && abi::source_contract::functions::TakeBid::match_call(call)
                    })
                    .filter_map(|call| {
                        match abi::source_contract::functions::TakeBid::decode(call) {
                            Ok(decoded_call) => Some(TakeBidExt {
                                call: Some(contract::SourceTakeBidCall {
                                    call_tx_hash: Hex(&tx.hash).to_string(),
                                    call_block_time: Some(blk.timestamp().to_owned()),
                                    call_block_number: blk.number,
                                    call_ordinal: call.begin_ordinal,
                                    call_success: !call.state_reverted,
                                    oracle_signature: decoded_call.oracle_signature.clone(),
                                }),
                                inputs: Some(decoded_call.into()),
                            }),
                            Err(_) => None,
                        }
                    })
            })
            .collect(),
    );
    calls.take_bids_single.append(
        &mut blk
            .transactions()
            .flat_map(|tx| {
                tx.calls
                    .iter()
                    .filter(|call| {
                        call.address == SOURCE_TRACKED_CONTRACT
                            && abi::source_contract::functions::TakeBidSingle::match_call(call)
                    })
                    .filter_map(
                        |call| match abi::source_contract::functions::TakeBidSingle::decode(call) {
                            Ok(decoded_call) => Some(TakeBidSingleExt {
                                call: Some(contract::SourceTakeBidSingleCall {
                                    call_tx_hash: Hex(&tx.hash).to_string(),
                                    call_block_time: Some(blk.timestamp().to_owned()),
                                    call_block_number: blk.number,
                                    call_ordinal: call.begin_ordinal,
                                    call_success: !call.state_reverted,
                                    oracle_signature: decoded_call.oracle_signature.clone(),
                                }),
                                inputs: Some(decoded_call.into()),
                            }),
                            Err(_) => None,
                        },
                    )
            })
            .collect(),
    );
}

fn db_proxy_out(events: &contract::Events, tables: &mut DatabaseChangeTables) {
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
fn db_source_out(events: &contract::Events, tables: &mut DatabaseChangeTables) {
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
fn db_source_calls_out(calls: &contract::Calls, tables: &mut DatabaseChangeTables) {
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

fn graph_proxy_out(events: &contract::Events, tables: &mut EntityChangesTables) {
    // Loop over all the abis events to create table changes
    events.proxy_admin_changed_1s.iter().for_each(|evt| {
        tables
            .create_row(
                "proxy_admin_changed1",
                format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
            )
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("new_admin", Hex(&evt.new_admin).to_string())
            .set("previous_admin", Hex(&evt.previous_admin).to_string());
    });
    events.proxy_admin_changed_2s.iter().for_each(|evt| {
        tables
            .create_row(
                "proxy_admin_changed2",
                format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
            )
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("new_admin", Hex(&evt.new_admin).to_string())
            .set("previous_admin", Hex(&evt.previous_admin).to_string());
    });
    events.proxy_beacon_upgraded_1s.iter().for_each(|evt| {
        tables
            .create_row(
                "proxy_beacon_upgraded1",
                format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
            )
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("beacon", Hex(&evt.beacon).to_string());
    });
    events.proxy_beacon_upgraded_2s.iter().for_each(|evt| {
        tables
            .create_row(
                "proxy_beacon_upgraded2",
                format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
            )
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("beacon", Hex(&evt.beacon).to_string());
    });
    events.proxy_cancel_trades.iter().for_each(|evt| {
        tables
            .create_row(
                "proxy_cancel_trade",
                format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
            )
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
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
                format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
            )
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
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
                    format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
                )
                .set("evt_tx_hash", &evt.evt_tx_hash)
                .set("evt_index", evt.evt_index)
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
                format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
            )
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
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
                    format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
                )
                .set("evt_tx_hash", &evt.evt_tx_hash)
                .set("evt_index", evt.evt_index)
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
                format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
            )
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("version", evt.version);
    });
    events.proxy_new_block_ranges.iter().for_each(|evt| {
        tables
            .create_row(
                "proxy_new_block_range",
                format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
            )
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
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
                format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
            )
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("governor", Hex(&evt.governor).to_string());
    });
    events.proxy_new_protocol_fees.iter().for_each(|evt| {
        tables
            .create_row(
                "proxy_new_protocol_fee",
                format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
            )
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("rate", evt.rate)
            .set("recipient", Hex(&evt.recipient).to_string());
    });
    events.proxy_nonce_incrementeds.iter().for_each(|evt| {
        tables
            .create_row(
                "proxy_nonce_incremented",
                format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
            )
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
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
                    format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
                )
                .set("evt_tx_hash", &evt.evt_tx_hash)
                .set("evt_index", evt.evt_index)
                .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
                .set("evt_block_number", evt.evt_block_number)
                .set("new_owner", Hex(&evt.new_owner).to_string())
                .set("previous_owner", Hex(&evt.previous_owner).to_string());
        });
    events.proxy_ownership_transferreds.iter().for_each(|evt| {
        tables
            .create_row(
                "proxy_ownership_transferred",
                format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
            )
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("new_owner", Hex(&evt.new_owner).to_string())
            .set("previous_owner", Hex(&evt.previous_owner).to_string());
    });
    events.proxy_set_oracles.iter().for_each(|evt| {
        tables
            .create_row(
                "proxy_set_oracle",
                format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
            )
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("approved", evt.approved)
            .set("user", Hex(&evt.user).to_string());
    });
    events.proxy_upgraded_1s.iter().for_each(|evt| {
        tables
            .create_row(
                "proxy_upgraded1",
                format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
            )
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("implementation", Hex(&evt.implementation).to_string());
    });
    events.proxy_upgraded_2s.iter().for_each(|evt| {
        tables
            .create_row(
                "proxy_upgraded2",
                format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
            )
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("implementation", Hex(&evt.implementation).to_string());
    });
}
fn graph_source_out(events: &contract::Events, tables: &mut EntityChangesTables) {
    // Loop over all the abis events to create table changes
    events.source_admin_changeds.iter().for_each(|evt| {
        tables
            .create_row(
                "source_admin_changed",
                format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
            )
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("new_admin", Hex(&evt.new_admin).to_string())
            .set("previous_admin", Hex(&evt.previous_admin).to_string());
    });
    events.source_beacon_upgradeds.iter().for_each(|evt| {
        tables
            .create_row(
                "source_beacon_upgraded",
                format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
            )
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("beacon", Hex(&evt.beacon).to_string());
    });
    events.source_cancel_trades.iter().for_each(|evt| {
        tables
            .create_row(
                "source_cancel_trade",
                format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
            )
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
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
                format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
            )
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
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
                    format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
                )
                .set("evt_tx_hash", &evt.evt_tx_hash)
                .set("evt_index", evt.evt_index)
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
                format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
            )
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
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
                    format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
                )
                .set("evt_tx_hash", &evt.evt_tx_hash)
                .set("evt_index", evt.evt_index)
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
                format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
            )
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("version", evt.version);
    });
    events.source_new_block_ranges.iter().for_each(|evt| {
        tables
            .create_row(
                "source_new_block_range",
                format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
            )
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
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
                format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
            )
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("governor", Hex(&evt.governor).to_string());
    });
    events.source_new_protocol_fees.iter().for_each(|evt| {
        tables
            .create_row(
                "source_new_protocol_fee",
                format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
            )
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("rate", evt.rate)
            .set("recipient", Hex(&evt.recipient).to_string());
    });
    events.source_nonce_incrementeds.iter().for_each(|evt| {
        tables
            .create_row(
                "source_nonce_incremented",
                format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
            )
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
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
                    format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
                )
                .set("evt_tx_hash", &evt.evt_tx_hash)
                .set("evt_index", evt.evt_index)
                .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
                .set("evt_block_number", evt.evt_block_number)
                .set("new_owner", Hex(&evt.new_owner).to_string())
                .set("previous_owner", Hex(&evt.previous_owner).to_string());
        });
    events.source_ownership_transferreds.iter().for_each(|evt| {
        tables
            .create_row(
                "source_ownership_transferred",
                format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
            )
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("new_owner", Hex(&evt.new_owner).to_string())
            .set("previous_owner", Hex(&evt.previous_owner).to_string());
    });
    events.source_set_oracles.iter().for_each(|evt| {
        tables
            .create_row(
                "source_set_oracle",
                format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
            )
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("approved", evt.approved)
            .set("user", Hex(&evt.user).to_string());
    });
    events.source_upgradeds.iter().for_each(|evt| {
        tables
            .create_row(
                "source_upgraded",
                format!("{}-{}", evt.evt_tx_hash, evt.evt_index),
            )
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("implementation", Hex(&evt.implementation).to_string());
    });
}
fn graph_source_calls_out(calls: &contract::Calls, tables: &mut EntityChangesTables) {
    // Loop over all the abis calls to create table changes
    calls.source_call_accept_ownerships.iter().for_each(|call| {
        tables
            .create_row(
                "source_call_accept_ownership",
                format!("{}-{}", call.call_tx_hash, call.call_ordinal),
            )
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success);
    });
    calls.source_call_cancel_trades.iter().for_each(|call| {
        tables
            .create_row(
                "source_call_cancel_trades",
                format!("{}-{}", call.call_tx_hash, call.call_ordinal),
            )
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success);
    });
    calls.source_call_increment_nonces.iter().for_each(|call| {
        tables
            .create_row(
                "source_call_increment_nonce",
                format!("{}-{}", call.call_tx_hash, call.call_ordinal),
            )
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success);
    });
    calls.source_call_initializes.iter().for_each(|call| {
        tables
            .create_row(
                "source_call_initialize",
                format!("{}-{}", call.call_tx_hash, call.call_ordinal),
            )
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
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
                    format!("{}-{}", call.call_tx_hash, call.call_ordinal),
                )
                .set("call_tx_hash", &call.call_tx_hash)
                .set("call_ordinal", call.call_ordinal)
                .set("call_block_time", call.call_block_time.as_ref().unwrap())
                .set("call_block_number", call.call_block_number)
                .set("call_success", call.call_success);
        });
    calls.source_call_set_block_ranges.iter().for_each(|call| {
        tables
            .create_row(
                "source_call_set_block_range",
                format!("{}-{}", call.call_tx_hash, call.call_ordinal),
            )
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
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
                format!("{}-{}", call.call_tx_hash, call.call_ordinal),
            )
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("u_governor", Hex(&call.u_governor).to_string());
    });
    calls.source_call_set_oracles.iter().for_each(|call| {
        tables
            .create_row(
                "source_call_set_oracle",
                format!("{}-{}", call.call_tx_hash, call.call_ordinal),
            )
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
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
                format!("{}-{}", call.call_tx_hash, call.call_ordinal),
            )
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
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
                format!("{}-{}", call.call_tx_hash, call.call_ordinal),
            )
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("oracle_signature", Hex(&call.oracle_signature).to_string());
    });
    calls.source_call_take_ask_pools.iter().for_each(|call| {
        tables
            .create_row(
                "source_call_take_ask_pool",
                format!("{}-{}", call.call_tx_hash, call.call_ordinal),
            )
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
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
                format!("{}-{}", call.call_tx_hash, call.call_ordinal),
            )
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
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
                    format!("{}-{}", call.call_tx_hash, call.call_ordinal),
                )
                .set("call_tx_hash", &call.call_tx_hash)
                .set("call_ordinal", call.call_ordinal)
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
                format!("{}-{}", call.call_tx_hash, call.call_ordinal),
            )
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
            .set("call_block_time", call.call_block_time.as_ref().unwrap())
            .set("call_block_number", call.call_block_number)
            .set("call_success", call.call_success)
            .set("oracle_signature", Hex(&call.oracle_signature).to_string());
    });
    calls.source_call_take_bid_singles.iter().for_each(|call| {
        tables
            .create_row(
                "source_call_take_bid_single",
                format!("{}-{}", call.call_tx_hash, call.call_ordinal),
            )
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
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
                    format!("{}-{}", call.call_tx_hash, call.call_ordinal),
                )
                .set("call_tx_hash", &call.call_tx_hash)
                .set("call_ordinal", call.call_ordinal)
                .set("call_block_time", call.call_block_time.as_ref().unwrap())
                .set("call_block_number", call.call_block_number)
                .set("call_success", call.call_success)
                .set("new_owner", Hex(&call.new_owner).to_string());
        });
    calls.source_call_upgrade_tos.iter().for_each(|call| {
        tables
            .create_row(
                "source_call_upgrade_to",
                format!("{}-{}", call.call_tx_hash, call.call_ordinal),
            )
            .set("call_tx_hash", &call.call_tx_hash)
            .set("call_ordinal", call.call_ordinal)
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
                    format!("{}-{}", call.call_tx_hash, call.call_ordinal),
                )
                .set("call_tx_hash", &call.call_tx_hash)
                .set("call_ordinal", call.call_ordinal)
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

#[substreams::handlers::map]
fn map_events(blk: eth::Block) -> Result<contract::Events, substreams::errors::Error> {
    let mut events = contract::Events::default();
    map_proxy_events(&blk, &mut events);
    map_source_events(&blk, &mut events);
    Ok(events)
}
#[substreams::handlers::map]
fn map_calls(blk: eth::Block) -> Result<contract::Calls, substreams::errors::Error> {
    let mut calls = contract::Calls::default();
    let mut calls_inputs: CallsWithInputs = CallsWithInputs::default();
    map_source_calls(&blk, &mut calls);
    map_proxy_calls(&blk, &mut calls);

    map_source_calls_inputs(&blk, &mut calls_inputs);

    calls_inputs.take_asks.iter().for_each(|call| {
        substreams::log::info!("Take_ask decoded inputs: {:#?}", call);
    });
    calls_inputs.take_asks_single.iter().for_each(|call| {
        substreams::log::info!("Take_ask_single decoded inputs: {:#?}", call);
    });
    calls_inputs.take_bids_single.iter().for_each(|call| {
        substreams::log::info!("Take_bid_single decoded inputs: {:#?}", call);
    });
    calls_inputs.take_bids.iter().for_each(|call| {
        substreams::log::info!("Take_bid decoded inputs: {:#?}", call);
    });

    Ok(calls)
}

fn build_trades_log(txs: CallsWithInputs) -> Vec<Trades> {
    let mut trades: Vec<Trades> = Vec::new();
    trades.append(&mut txs.take_asks_single.iter().flat_map(|&tx| {
        Trades {
            tx_hash: tx.call.unwrap().call_tx_hash.into(),
            volume: tx.inputs.unwrap().exchange.unwrap().listing.unwrap().price,
            erc_721_transfers: vec![Transfer {
                trader: None,
                id: 0,
                amount: 0,
                collection: None,
                from: tx.inputs.unwrap().exchange.unwrap().listing.unwrap().seller,
                to: tx.inputs.unwrap().exchange.unwrap().listing.unwrap().buyer,
                token_id: tx.inputs.unwrap().exchange.unwrap().listing.unwrap().token_id,
                asset_type: 0,
            }],
        }
    }).collect());
}


#[substreams::handlers::map]
fn db_out(
    events: contract::Events,
    calls: contract::Calls,
) -> Result<DatabaseChanges, substreams::errors::Error> {
    // Initialize Database Changes container
    let mut tables = DatabaseChangeTables::new();
    db_proxy_out(&events, &mut tables);
    db_source_out(&events, &mut tables);
    db_source_calls_out(&calls, &mut tables);
    Ok(tables.to_database_changes())
}

#[substreams::handlers::map]
fn graph_out(
    events: contract::Events,
    calls: contract::Calls,
) -> Result<EntityChanges, substreams::errors::Error> {
    // Initialize Database Changes container
    let mut tables = EntityChangesTables::new();
    graph_proxy_out(&events, &mut tables);
    graph_source_out(&events, &mut tables);
    graph_source_calls_out(&calls, &mut tables);
    Ok(tables.to_entity_changes())
}
