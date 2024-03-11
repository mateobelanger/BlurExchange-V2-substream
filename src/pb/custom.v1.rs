// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallsWithInputs {
    #[prost(message, repeated, tag="1")]
    pub take_asks: ::prost::alloc::vec::Vec<TakeAskExt>,
    #[prost(message, repeated, tag="2")]
    pub take_asks_single: ::prost::alloc::vec::Vec<TakeAskSingleExt>,
    #[prost(message, repeated, tag="3")]
    pub take_bids: ::prost::alloc::vec::Vec<TakeBidExt>,
    #[prost(message, repeated, tag="4")]
    pub take_bids_single: ::prost::alloc::vec::Vec<TakeBidSingleExt>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TakeAskExt {
    #[prost(message, optional, tag="1")]
    pub call: ::core::option::Option<SourceTakeAskCall>,
    #[prost(message, optional, tag="2")]
    pub inputs: ::core::option::Option<TakeAsk>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TakeAskSingleExt {
    #[prost(message, optional, tag="1")]
    pub call: ::core::option::Option<SourceTakeAskSingleCall>,
    #[prost(message, optional, tag="2")]
    pub inputs: ::core::option::Option<TakeAskSingle>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TakeBidExt {
    #[prost(message, optional, tag="1")]
    pub call: ::core::option::Option<SourceTakeBidCall>,
    #[prost(message, optional, tag="2")]
    pub inputs: ::core::option::Option<TakeBid>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TakeBidSingleExt {
    #[prost(message, optional, tag="1")]
    pub call: ::core::option::Option<SourceTakeBidSingleCall>,
    #[prost(message, optional, tag="2")]
    pub inputs: ::core::option::Option<TakeBidSingle>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Trade {
    #[prost(string, tag="1")]
    pub tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(bytes="vec", tag="3")]
    pub total_wei_value: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="4")]
    pub erc_721_transfer: ::core::option::Option<Transfer721>,
    #[prost(enumeration="TradeType", tag="5")]
    pub trade_type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Trades {
    #[prost(message, repeated, tag="1")]
    pub trades: ::prost::alloc::vec::Vec<Trade>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transfer721 {
    #[prost(string, tag="1")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub to: ::prost::alloc::string::String,
    #[prost(int64, tag="3")]
    pub id: i64,
    #[prost(int64, tag="4")]
    pub amount: i64,
    #[prost(string, tag="5")]
    pub collection: ::prost::alloc::string::String,
    #[prost(enumeration="AssetType", tag="6")]
    pub asset_type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Events {
    #[prost(message, repeated, tag="1")]
    pub proxy_admin_changed_1s: ::prost::alloc::vec::Vec<ProxyAdminChanged1>,
    #[prost(message, repeated, tag="2")]
    pub proxy_admin_changed_2s: ::prost::alloc::vec::Vec<ProxyAdminChanged2>,
    #[prost(message, repeated, tag="3")]
    pub proxy_beacon_upgraded_1s: ::prost::alloc::vec::Vec<ProxyBeaconUpgraded1>,
    #[prost(message, repeated, tag="4")]
    pub proxy_beacon_upgraded_2s: ::prost::alloc::vec::Vec<ProxyBeaconUpgraded2>,
    #[prost(message, repeated, tag="5")]
    pub proxy_cancel_trades: ::prost::alloc::vec::Vec<ProxyCancelTrade>,
    #[prost(message, repeated, tag="6")]
    pub proxy_executions: ::prost::alloc::vec::Vec<ProxyExecution>,
    #[prost(message, repeated, tag="7")]
    pub proxy_execution721_maker_fee_packeds: ::prost::alloc::vec::Vec<ProxyExecution721MakerFeePacked>,
    #[prost(message, repeated, tag="8")]
    pub proxy_execution721_packeds: ::prost::alloc::vec::Vec<ProxyExecution721Packed>,
    #[prost(message, repeated, tag="9")]
    pub proxy_execution721_taker_fee_packeds: ::prost::alloc::vec::Vec<ProxyExecution721TakerFeePacked>,
    #[prost(message, repeated, tag="10")]
    pub proxy_initializeds: ::prost::alloc::vec::Vec<ProxyInitialized>,
    #[prost(message, repeated, tag="11")]
    pub proxy_new_block_ranges: ::prost::alloc::vec::Vec<ProxyNewBlockRange>,
    #[prost(message, repeated, tag="12")]
    pub proxy_new_governors: ::prost::alloc::vec::Vec<ProxyNewGovernor>,
    #[prost(message, repeated, tag="13")]
    pub proxy_new_protocol_fees: ::prost::alloc::vec::Vec<ProxyNewProtocolFee>,
    #[prost(message, repeated, tag="14")]
    pub proxy_nonce_incrementeds: ::prost::alloc::vec::Vec<ProxyNonceIncremented>,
    #[prost(message, repeated, tag="15")]
    pub proxy_ownership_transfer_starteds: ::prost::alloc::vec::Vec<ProxyOwnershipTransferStarted>,
    #[prost(message, repeated, tag="16")]
    pub proxy_ownership_transferreds: ::prost::alloc::vec::Vec<ProxyOwnershipTransferred>,
    #[prost(message, repeated, tag="17")]
    pub proxy_set_oracles: ::prost::alloc::vec::Vec<ProxySetOracle>,
    #[prost(message, repeated, tag="18")]
    pub proxy_upgraded_1s: ::prost::alloc::vec::Vec<ProxyUpgraded1>,
    #[prost(message, repeated, tag="19")]
    pub proxy_upgraded_2s: ::prost::alloc::vec::Vec<ProxyUpgraded2>,
    #[prost(message, repeated, tag="20")]
    pub source_admin_changeds: ::prost::alloc::vec::Vec<SourceAdminChanged>,
    #[prost(message, repeated, tag="21")]
    pub source_beacon_upgradeds: ::prost::alloc::vec::Vec<SourceBeaconUpgraded>,
    #[prost(message, repeated, tag="22")]
    pub source_cancel_trades: ::prost::alloc::vec::Vec<SourceCancelTrade>,
    #[prost(message, repeated, tag="23")]
    pub source_executions: ::prost::alloc::vec::Vec<SourceExecution>,
    #[prost(message, repeated, tag="24")]
    pub source_execution721_maker_fee_packeds: ::prost::alloc::vec::Vec<SourceExecution721MakerFeePacked>,
    #[prost(message, repeated, tag="25")]
    pub source_execution721_packeds: ::prost::alloc::vec::Vec<SourceExecution721Packed>,
    #[prost(message, repeated, tag="26")]
    pub source_execution721_taker_fee_packeds: ::prost::alloc::vec::Vec<SourceExecution721TakerFeePacked>,
    #[prost(message, repeated, tag="27")]
    pub source_initializeds: ::prost::alloc::vec::Vec<SourceInitialized>,
    #[prost(message, repeated, tag="28")]
    pub source_new_block_ranges: ::prost::alloc::vec::Vec<SourceNewBlockRange>,
    #[prost(message, repeated, tag="29")]
    pub source_new_governors: ::prost::alloc::vec::Vec<SourceNewGovernor>,
    #[prost(message, repeated, tag="30")]
    pub source_new_protocol_fees: ::prost::alloc::vec::Vec<SourceNewProtocolFee>,
    #[prost(message, repeated, tag="31")]
    pub source_nonce_incrementeds: ::prost::alloc::vec::Vec<SourceNonceIncremented>,
    #[prost(message, repeated, tag="32")]
    pub source_ownership_transfer_starteds: ::prost::alloc::vec::Vec<SourceOwnershipTransferStarted>,
    #[prost(message, repeated, tag="33")]
    pub source_ownership_transferreds: ::prost::alloc::vec::Vec<SourceOwnershipTransferred>,
    #[prost(message, repeated, tag="34")]
    pub source_set_oracles: ::prost::alloc::vec::Vec<SourceSetOracle>,
    #[prost(message, repeated, tag="35")]
    pub source_upgradeds: ::prost::alloc::vec::Vec<SourceUpgraded>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Calls {
    #[prost(message, repeated, tag="1")]
    pub source_call_accept_ownerships: ::prost::alloc::vec::Vec<SourceAcceptOwnershipCall>,
    #[prost(message, repeated, tag="2")]
    pub source_call_cancel_trades: ::prost::alloc::vec::Vec<SourceCancelTradesCall>,
    #[prost(message, repeated, tag="3")]
    pub source_call_increment_nonces: ::prost::alloc::vec::Vec<SourceIncrementNonceCall>,
    #[prost(message, repeated, tag="4")]
    pub source_call_initializes: ::prost::alloc::vec::Vec<SourceInitializeCall>,
    #[prost(message, repeated, tag="5")]
    pub source_call_renounce_ownerships: ::prost::alloc::vec::Vec<SourceRenounceOwnershipCall>,
    #[prost(message, repeated, tag="6")]
    pub source_call_set_block_ranges: ::prost::alloc::vec::Vec<SourceSetBlockRangeCall>,
    #[prost(message, repeated, tag="7")]
    pub source_call_set_governors: ::prost::alloc::vec::Vec<SourceSetGovernorCall>,
    #[prost(message, repeated, tag="8")]
    pub source_call_set_oracles: ::prost::alloc::vec::Vec<SourceSetOracleCall>,
    #[prost(message, repeated, tag="9")]
    pub source_call_set_protocol_fees: ::prost::alloc::vec::Vec<SourceSetProtocolFeeCall>,
    #[prost(message, repeated, tag="10")]
    pub source_call_take_asks: ::prost::alloc::vec::Vec<SourceTakeAskCall>,
    #[prost(message, repeated, tag="11")]
    pub source_call_take_ask_pools: ::prost::alloc::vec::Vec<SourceTakeAskPoolCall>,
    #[prost(message, repeated, tag="12")]
    pub source_call_take_ask_singles: ::prost::alloc::vec::Vec<SourceTakeAskSingleCall>,
    #[prost(message, repeated, tag="13")]
    pub source_call_take_ask_single_pools: ::prost::alloc::vec::Vec<SourceTakeAskSinglePoolCall>,
    #[prost(message, repeated, tag="14")]
    pub source_call_take_bids: ::prost::alloc::vec::Vec<SourceTakeBidCall>,
    #[prost(message, repeated, tag="15")]
    pub source_call_take_bid_singles: ::prost::alloc::vec::Vec<SourceTakeBidSingleCall>,
    #[prost(message, repeated, tag="16")]
    pub source_call_transfer_ownerships: ::prost::alloc::vec::Vec<SourceTransferOwnershipCall>,
    #[prost(message, repeated, tag="17")]
    pub source_call_upgrade_tos: ::prost::alloc::vec::Vec<SourceUpgradeToCall>,
    #[prost(message, repeated, tag="18")]
    pub source_call_upgrade_to_and_calls: ::prost::alloc::vec::Vec<SourceUpgradeToAndCallCall>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProxyAdminChanged1 {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub previous_admin: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub new_admin: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProxyAdminChanged2 {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub previous_admin: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub new_admin: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProxyBeaconUpgraded1 {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub beacon: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProxyBeaconUpgraded2 {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub beacon: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProxyCancelTrade {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub user: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="7")]
    pub index: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub amount: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProxyExecution {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub order_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="6")]
    pub listing_index: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub price: ::prost::alloc::string::String,
    #[prost(uint64, tag="8")]
    pub order_type: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProxyExecution721MakerFeePacked {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub order_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="6")]
    pub token_id_listing_index_trader: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub collection_price_side: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub maker_fee_recipient_rate: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProxyExecution721Packed {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub order_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="6")]
    pub token_id_listing_index_trader: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub collection_price_side: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProxyExecution721TakerFeePacked {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub order_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="6")]
    pub token_id_listing_index_trader: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub collection_price_side: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub taker_fee_recipient_rate: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProxyInitialized {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(uint64, tag="5")]
    pub version: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProxyNewBlockRange {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub block_range: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProxyNewGovernor {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub governor: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProxyNewProtocolFee {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="6")]
    pub rate: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProxyNonceIncremented {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub user: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="6")]
    pub new_nonce: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProxyOwnershipTransferStarted {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub previous_owner: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub new_owner: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProxyOwnershipTransferred {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub previous_owner: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub new_owner: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProxySetOracle {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub user: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag="6")]
    pub approved: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProxyUpgraded1 {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub implementation: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProxyUpgraded2 {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub implementation: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceAdminChanged {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub previous_admin: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub new_admin: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceBeaconUpgraded {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub beacon: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceCancelTrade {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub user: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="7")]
    pub index: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub amount: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceExecution {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub order_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="6")]
    pub listing_index: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub price: ::prost::alloc::string::String,
    #[prost(uint64, tag="8")]
    pub order_type: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceExecution721MakerFeePacked {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub order_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="6")]
    pub token_id_listing_index_trader: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub collection_price_side: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub maker_fee_recipient_rate: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceExecution721Packed {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub order_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="6")]
    pub token_id_listing_index_trader: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub collection_price_side: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceExecution721TakerFeePacked {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub order_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="6")]
    pub token_id_listing_index_trader: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub collection_price_side: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub taker_fee_recipient_rate: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceInitialized {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(uint64, tag="5")]
    pub version: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceNewBlockRange {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub block_range: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceNewGovernor {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub governor: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceNewProtocolFee {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="6")]
    pub rate: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceNonceIncremented {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub user: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="6")]
    pub new_nonce: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceOwnershipTransferStarted {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub previous_owner: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub new_owner: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceOwnershipTransferred {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub previous_owner: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub new_owner: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceSetOracle {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub user: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag="6")]
    pub approved: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceUpgraded {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub implementation: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceAcceptOwnershipCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceCancelTradesCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceIncrementNonceCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceInitializeCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceRenounceOwnershipCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceSetBlockRangeCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(string, tag="6")]
    pub u_block_range: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceSetGovernorCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(bytes="vec", tag="6")]
    pub u_governor: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceSetOracleCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(bytes="vec", tag="6")]
    pub oracle: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag="7")]
    pub approved: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceSetProtocolFeeCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(bytes="vec", tag="6")]
    pub recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="7")]
    pub rate: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceTakeAskCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(bytes="vec", tag="6")]
    pub oracle_signature: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceTakeAskPoolCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(bytes="vec", tag="6")]
    pub oracle_signature: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="7")]
    pub amount_to_withdraw: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceTakeAskSingleCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(bytes="vec", tag="6")]
    pub oracle_signature: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceTakeAskSinglePoolCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(bytes="vec", tag="6")]
    pub oracle_signature: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="7")]
    pub amount_to_withdraw: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceTakeBidCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(bytes="vec", tag="6")]
    pub oracle_signature: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceTakeBidSingleCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(bytes="vec", tag="6")]
    pub oracle_signature: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceTransferOwnershipCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(bytes="vec", tag="6")]
    pub new_owner: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceUpgradeToCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(bytes="vec", tag="6")]
    pub new_implementation: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceUpgradeToAndCallCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(bytes="vec", tag="6")]
    pub new_implementation: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Address {
    #[prost(bytes="vec", tag="1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeRate {
    #[prost(message, optional, tag="1")]
    pub recipient: ::core::option::Option<Address>,
    #[prost(uint64, tag="2")]
    pub rate: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Fees {
    #[prost(message, optional, tag="1")]
    pub protocol_fee: ::core::option::Option<FeeRate>,
    #[prost(message, optional, tag="2")]
    pub taker_fee: ::core::option::Option<FeeRate>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transfer {
    #[prost(message, optional, tag="1")]
    pub trader: ::core::option::Option<Address>,
    #[prost(int64, tag="2")]
    pub id: i64,
    #[prost(int64, tag="3")]
    pub amount: i64,
    #[prost(message, optional, tag="4")]
    pub collection: ::core::option::Option<Address>,
    #[prost(enumeration="AssetType", tag="5")]
    pub asset_type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Order {
    #[prost(message, optional, tag="1")]
    pub trader: ::core::option::Option<Address>,
    #[prost(message, optional, tag="2")]
    pub collection: ::core::option::Option<Address>,
    #[prost(bytes="vec", tag="3")]
    pub listings_root: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="4")]
    pub number_of_listings: i64,
    #[prost(int64, tag="5")]
    pub expiration_time: i64,
    #[prost(enumeration="AssetType", tag="6")]
    pub asset_type: i32,
    #[prost(message, optional, tag="7")]
    pub maker_fee: ::core::option::Option<FeeRate>,
    #[prost(int64, tag="8")]
    pub salt: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Exchange {
    #[prost(int64, tag="1")]
    pub index: i64,
    #[prost(bytes="vec", repeated, tag="2")]
    pub proof: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag="3")]
    pub listing: ::core::option::Option<Listing>,
    #[prost(message, optional, tag="4")]
    pub taker: ::core::option::Option<Taker>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Taker {
    #[prost(bytes="vec", tag="1")]
    pub token_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="2")]
    pub amount: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Listing {
    #[prost(int64, tag="1")]
    pub index: i64,
    #[prost(bytes="vec", tag="2")]
    pub token_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="3")]
    pub amount: i64,
    #[prost(bytes="vec", tag="4")]
    pub price: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TakeBidSingle {
    #[prost(message, optional, tag="1")]
    pub order: ::core::option::Option<Order>,
    #[prost(message, optional, tag="2")]
    pub exchange: ::core::option::Option<Exchange>,
    #[prost(message, optional, tag="3")]
    pub taker_fee: ::core::option::Option<FeeRate>,
    #[prost(bytes="vec", tag="4")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TakeBid {
    #[prost(message, repeated, tag="1")]
    pub orders: ::prost::alloc::vec::Vec<Order>,
    #[prost(message, repeated, tag="2")]
    pub exchanges: ::prost::alloc::vec::Vec<Exchange>,
    #[prost(message, optional, tag="3")]
    pub taker_fee: ::core::option::Option<FeeRate>,
    #[prost(bytes="vec", tag="4")]
    pub signatures: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TakeAskSingle {
    #[prost(message, optional, tag="1")]
    pub order: ::core::option::Option<Order>,
    #[prost(message, optional, tag="2")]
    pub exchange: ::core::option::Option<Exchange>,
    #[prost(message, optional, tag="3")]
    pub taker_fee: ::core::option::Option<FeeRate>,
    #[prost(bytes="vec", tag="4")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="5")]
    pub token_recipient: ::core::option::Option<Address>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TakeAsk {
    #[prost(message, repeated, tag="1")]
    pub orders: ::prost::alloc::vec::Vec<Order>,
    #[prost(message, repeated, tag="2")]
    pub exchanges: ::prost::alloc::vec::Vec<Exchange>,
    #[prost(message, optional, tag="3")]
    pub taker_fee: ::core::option::Option<FeeRate>,
    #[prost(bytes="vec", tag="4")]
    pub signatures: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="5")]
    pub token_recipient: ::core::option::Option<Address>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TradeType {
    Takeask = 0,
    Takeasksingle = 1,
    Takeasksinglepool = 2,
    Takebid = 3,
    Takebidsingle = 4,
    Takebidsinglepool = 5,
}
impl TradeType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TradeType::Takeask => "TAKEASK",
            TradeType::Takeasksingle => "TAKEASKSINGLE",
            TradeType::Takeasksinglepool => "TAKEASKSINGLEPOOL",
            TradeType::Takebid => "TAKEBID",
            TradeType::Takebidsingle => "TAKEBIDSINGLE",
            TradeType::Takebidsinglepool => "TAKEBIDSINGLEPOOL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TAKEASK" => Some(Self::Takeask),
            "TAKEASKSINGLE" => Some(Self::Takeasksingle),
            "TAKEASKSINGLEPOOL" => Some(Self::Takeasksinglepool),
            "TAKEBID" => Some(Self::Takebid),
            "TAKEBIDSINGLE" => Some(Self::Takebidsingle),
            "TAKEBIDSINGLEPOOL" => Some(Self::Takebidsinglepool),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderType {
    Ask = 0,
    Bid = 1,
}
impl OrderType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrderType::Ask => "ASK",
            OrderType::Bid => "BID",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ASK" => Some(Self::Ask),
            "BID" => Some(Self::Bid),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AssetType {
    Erc721 = 0,
    Erc1155 = 1,
}
impl AssetType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AssetType::Erc721 => "ERC721",
            AssetType::Erc1155 => "ERC1155",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ERC721" => Some(Self::Erc721),
            "ERC1155" => Some(Self::Erc1155),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
