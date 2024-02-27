// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Events {
    #[prost(message, repeated, tag="1")]
    pub exchangev2_admin_changed_1s: ::prost::alloc::vec::Vec<Exchangev2AdminChanged1>,
    #[prost(message, repeated, tag="2")]
    pub exchangev2_admin_changed_2s: ::prost::alloc::vec::Vec<Exchangev2AdminChanged2>,
    #[prost(message, repeated, tag="3")]
    pub exchangev2_beacon_upgraded_1s: ::prost::alloc::vec::Vec<Exchangev2BeaconUpgraded1>,
    #[prost(message, repeated, tag="4")]
    pub exchangev2_beacon_upgraded_2s: ::prost::alloc::vec::Vec<Exchangev2BeaconUpgraded2>,
    #[prost(message, repeated, tag="5")]
    pub exchangev2_cancel_trades: ::prost::alloc::vec::Vec<Exchangev2CancelTrade>,
    #[prost(message, repeated, tag="6")]
    pub exchangev2_executions: ::prost::alloc::vec::Vec<Exchangev2Execution>,
    #[prost(message, repeated, tag="7")]
    pub exchangev2_execution721_maker_fee_packeds: ::prost::alloc::vec::Vec<Exchangev2Execution721MakerFeePacked>,
    #[prost(message, repeated, tag="8")]
    pub exchangev2_execution721_packeds: ::prost::alloc::vec::Vec<Exchangev2Execution721Packed>,
    #[prost(message, repeated, tag="9")]
    pub exchangev2_execution721_taker_fee_packeds: ::prost::alloc::vec::Vec<Exchangev2Execution721TakerFeePacked>,
    #[prost(message, repeated, tag="10")]
    pub exchangev2_initializeds: ::prost::alloc::vec::Vec<Exchangev2Initialized>,
    #[prost(message, repeated, tag="11")]
    pub exchangev2_new_block_ranges: ::prost::alloc::vec::Vec<Exchangev2NewBlockRange>,
    #[prost(message, repeated, tag="12")]
    pub exchangev2_new_governors: ::prost::alloc::vec::Vec<Exchangev2NewGovernor>,
    #[prost(message, repeated, tag="13")]
    pub exchangev2_new_protocol_fees: ::prost::alloc::vec::Vec<Exchangev2NewProtocolFee>,
    #[prost(message, repeated, tag="14")]
    pub exchangev2_nonce_incrementeds: ::prost::alloc::vec::Vec<Exchangev2NonceIncremented>,
    #[prost(message, repeated, tag="15")]
    pub exchangev2_ownership_transfer_starteds: ::prost::alloc::vec::Vec<Exchangev2OwnershipTransferStarted>,
    #[prost(message, repeated, tag="16")]
    pub exchangev2_ownership_transferreds: ::prost::alloc::vec::Vec<Exchangev2OwnershipTransferred>,
    #[prost(message, repeated, tag="17")]
    pub exchangev2_set_oracles: ::prost::alloc::vec::Vec<Exchangev2SetOracle>,
    #[prost(message, repeated, tag="18")]
    pub exchangev2_upgraded_1s: ::prost::alloc::vec::Vec<Exchangev2Upgraded1>,
    #[prost(message, repeated, tag="19")]
    pub exchangev2_upgraded_2s: ::prost::alloc::vec::Vec<Exchangev2Upgraded2>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Calls {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Exchangev2AdminChanged1 {
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
pub struct Exchangev2AdminChanged2 {
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
pub struct Exchangev2BeaconUpgraded1 {
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
pub struct Exchangev2BeaconUpgraded2 {
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
pub struct Exchangev2CancelTrade {
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
pub struct Exchangev2Execution {
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
pub struct Exchangev2Execution721MakerFeePacked {
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
pub struct Exchangev2Execution721Packed {
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
pub struct Exchangev2Execution721TakerFeePacked {
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
pub struct Exchangev2Initialized {
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
pub struct Exchangev2NewBlockRange {
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
pub struct Exchangev2NewGovernor {
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
pub struct Exchangev2NewProtocolFee {
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
pub struct Exchangev2NonceIncremented {
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
pub struct Exchangev2OwnershipTransferStarted {
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
pub struct Exchangev2OwnershipTransferred {
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
pub struct Exchangev2SetOracle {
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
pub struct Exchangev2Upgraded1 {
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
pub struct Exchangev2Upgraded2 {
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
// @@protoc_insertion_point(module)
