
use substreams::scalar::{BigDecimal, BigInt};

use crate::abi::source_contract;
use prost::Message;
use serde::Serialize;
use substreams::Hex;
use rustc_hex::FromHex;
use crate::pb::custom::v1::*;

/* #[derive(Clone, PartialEq, prost::Message)]
pub struct TakeAsk {
    #[prost(message, repeated, tag = "1")]
    orders: Vec<Order>,
    #[prost(message, repeated, tag = "2")]
    exchanges: Vec<Exchange>,
    #[prost(message, tag = "3")]
    takerFee: Option<FeeRate>,
    #[prost(bytes, tag = "4")]
    signatures: Vec<u8>,
    #[prost(message, tag = "5")]
    tokenRecipient: Option<Address>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct TakeAskSingleInputs {
    #[prost(message, tag = "1")]
    order: Option<Order>,
    #[prost(message, tag = "2")]
    exchange: Option<Exchange>,
    #[prost(message, tag = "3")]
    takerFee: Option<FeeRate>,
    #[prost(bytes, tag = "4")]
    signature: Vec<u8>,
    #[prost(message, tag = "5")]
    tokenRecipient: Option<Address>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct TakeBid {
    #[prost(message, repeated, tag = "1")]
    orders: Vec<Order>,
    #[prost(message, repeated, tag = "2")]
    exchanges: Vec<Exchange>,
    #[prost(message, tag = "3")]
    takerFee: Option<FeeRate>,
    #[prost(bytes, tag = "4")]
    signatures: Vec<u8>,
}
#[derive(Clone, PartialEq, prost::Message)]
pub struct TakeBidSingle {
    #[prost(message, tag = "1")]
    order: Option<Order>,
    #[prost(message, tag = "2")]
    exchange: Option<Exchange>,
    #[prost(message, tag = "3")]numberOfListings
    takerFee: Option<FeeRate>,
    #[prost(bytes, tag = "4")]
    signature: Vec<u8>,
}

#[derive(Clone, Debug, Copy, PartialEq, ::prost::Enumeration)]
#[repr(i32)]
pub enum AssetType {
    ERC721 = 0,
    ERC1155 = 1,
}

#[derive(Clone, Debug, Copy, PartialEq, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderType {
    ASK = 0,
    BID = 1,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct Exchange {
    #[prost(uint64, tag = "1")]
    index: u64,
    #[prost(bytes, tag = "2")]
    proof: Vec<u8>,
    #[prost(message, tag = "3")]
    listing: Option<Listing>,
    #[prost(message, tag = "4")]
    taker: Option<Taker>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct Listing {
    #[prost(uint64, tag = "1")]
    index: u64,
    #[prost(uint64, tag = "2")]
    tokenId: u64,
    #[prost(uint64, tag = "3")]
    amount: u64,
    #[prost(uint64, tag = "4")]
    price: u64,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct Taker {
    #[prost(uint64, tag = "1")]
    tokenId: u64,
    #[prost(uint64, tag = "2")]
    amount: u64,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct Order {
    #[prost(message, tag = "1")]
    trader: Option<Address>,
    #[prost(message, tag = "3")]
    collection: Option<Address>,
    #[prost(bytes, tag = "2")]
    listingsRoot: Vec<u8>,
    #[prost(uint64, tag = "4")]
    numberOfListings: u64,
    #[prost(uint64, tag = "5")]
    expirationTime: u64,
    #[prost(enumeration = "AssetType", tag = "6")]
    assetType: i32,
    #[prost(message, tag = "7")]
    makerFee: Option<FeeRate>,
    #[prost(uint64, tag = "8")]
    salt: u64,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct Transfer {
    #[prost(message, tag = "1")]
    trader: Option<Address>,
    #[prost(uint64, tag = "2")]
    id: u64,
    #[prost(uint64, tag = "3")]
    amount: u64,
    #[prost(message, tag = "4")]
    collection: Option<Address>,
    #[prost(enumeration = "AssetType", tag = "5")]
    assetType: i32,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct Fees {
    #[prost(message, tag = "1")]
    protocolFee: Option<FeeRate>,
    #[prost(message, tag = "2")]
    takerFee: Option<FeeRate>,
}source_call_take_asks

#[derive(Clone, PartialEq, prost::Message)]
pub struct FeeRate {
    #[prost(message, tag = "1")]
    recipient: Option<Address>,
    #[prost(uint64, tag = "2")]
    rate: u64,
}


#[derive(Clone, PartialEq, prost::Message)]
pub struct Address {
    #[prost(bytes, tag = "1")]
    address: ::prost::alloc::vec::Vec<u8>,
}prost::
 */

#[derive(Clone, PartialEq, Message)]
pub struct TakeAskExt {
    #[prost(message, tag = "1")]
    pub call: Option<SourceTakeAskCall>,
    #[prost(message, tag = "2")]
    pub inputs: Option<TakeAsk>,
}

#[derive(Clone, PartialEq, Message)]
pub struct TakeAskSingleExt {
    #[prost(message, tag = "1")]
    pub call: Option<SourceTakeAskSingleCall>,
    #[prost(message, tag = "2")]
    pub inputs: Option<TakeAskSingle>,
}

#[derive(Clone, PartialEq, Message)]
pub struct TakeBidExt {
    #[prost(message, tag = "1")]
    pub call: Option<SourceTakeBidCall>,
    #[prost(message, tag = "2")]
    pub inputs: Option<TakeBid>,
    #[prost(message, tag = "3")]
    pub from: Option<Address>,
}

#[derive(Clone, PartialEq, Message)]
pub struct TakeBidSingleExt {
    #[prost(message, tag = "1")]
    pub call: Option<SourceTakeBidSingleCall>,
    #[prost(message, tag = "2")]
    pub inputs: Option<TakeBidSingle>,
    #[prost(message, tag = "3")]
    pub from: Option<Address>,
}

#[derive(Clone, PartialEq, Message)]
pub struct CallsWithInputs {
    #[prost(message, repeated, tag = "1")]
    pub take_asks: Vec<TakeAskExt>,
    #[prost(message, repeated, tag = "2")]
    pub take_asks_single: Vec<TakeAskSingleExt>,
    #[prost(message, repeated, tag = "3")]
    pub take_bids: Vec<TakeBidExt>,
    #[prost(message, repeated, tag = "4")]
    pub take_bids_single: Vec<TakeBidSingleExt>,
}


#[derive(Clone, PartialEq, Message)]
pub struct Trade {
    #[prost(string, tag = "1")]
    pub tx_hash: String,
    #[prost(message, optional, tag="2")]
    pub timestamp: Option<::prost_types::Timestamp>,
    #[prost(bytes, tag = "3")]
    pub eth_value: Vec<u8>,
    #[prost(message, repeated, tag = "4")]
    pub erc_721_transfers: Vec<Transfer721>,
    // TODO: Should we consider that Blur Pool token is equal to WETH?
    //#[prost(message, repeated, tag = "5")]
    //pub erc_20_transfers: Vec<TransferERC20>,
    #[prost(enumeration="TradeType", tag="6")]
    pub trade_type: i32,
}

#[derive(Clone, PartialEq, Message)]
pub struct Trades {
    #[prost(message, repeated, tag = "1")]
    pub list: Vec<Trade>,
}

#[derive(Clone, PartialEq, Message)]
pub struct Transfer721 {
    #[prost(string, tag="1")]
    pub from: String,
    #[prost(string, tag="2")]
    pub to: String,
    #[prost(bytes, tag="3")]
    pub token_id: Vec<u8>,
    #[prost(int64, tag="4")]
    pub amount: i64,
    #[prost(string, tag="5")]
    pub collection: String,
    #[prost(enumeration="AssetType", tag="6")]
    pub asset_type: i32,
}

#[derive(Clone, PartialEq, Message)]
pub struct TransferERC20 {
    #[prost(string, tag="1")]
    pub from: String,
    #[prost(string, tag="2")]
    pub to: String,
    #[prost(int64, tag="3")]
    pub amount: i64,
    #[prost(string, tag="4")]
    pub token: String,
    #[prost(string, tag="5")]
    pub token_name: String,
}



#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TradeType {
    TakeAsk = 0,
    TakeAskSingle = 1,
    TakeAskSinglePool = 2,
    TakeBid = 3,
    TakeBidSingle = 4,
    TakeBidSinglePool = 5,
}
impl TradeType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TradeType::TakeAsk => "TakeAsk",
            TradeType::TakeAskSingle => "TakeAskSingle",
            TradeType::TakeAskSinglePool => "TakeAskSinglePool",
            TradeType::TakeBid => "TakeBid",
            TradeType::TakeBidSingle => "TakeBidSingle",
            TradeType::TakeBidSinglePool => "TakeBidSinglePool",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> Option<Self> {
        match value {
            "TakeAsk" => Some(TradeType::TakeAsk),
            "TakeAskSingle" => Some(TradeType::TakeAskSingle),
            "TakeAskSinglePool" => Some(TradeType::TakeAskSinglePool),
            "TakeBid" => Some(TradeType::TakeBid),
            "TakeBidSingle" => Some(TradeType::TakeBidSingle),
            "TakeBidSinglePool" => Some(TradeType::TakeBidSinglePool),            
            _ => None,
        }
    }
}

pub fn wei_to_eth(wei: BigInt) -> BigDecimal {
    let wei = BigDecimal::from(wei);
    let eth_conversion_factor = BigDecimal::new(BigInt::from(1), 18);
    wei / eth_conversion_factor
}


impl From<source_contract::functions::TakeAsk> for TakeAsk {
    fn from(raw: source_contract::functions::TakeAsk) -> Self {
        let orders = raw.inputs.0.into_iter().map(|input| input.into()).collect();
        let exchanges = raw.inputs.1.into_iter().map(|input| input.into()).collect();
        Self {
            orders,
            exchanges,
            taker_fee: Some(raw.inputs.2.into()),
            signatures: raw.inputs.3.into(),
            token_recipient: Some(raw.inputs.4.into()),
        }
    }
}

impl From<source_contract::functions::TakeAskSingle> for TakeAskSingle {
    fn from(raw: source_contract::functions::TakeAskSingle) -> Self {
        Self {
            order: Some(raw.inputs.0.into()),
            exchange: Some(raw.inputs.1.into()),
            taker_fee: Some(raw.inputs.2.into()),
            signature: raw.inputs.3.into(),
            token_recipient: Some(raw.inputs.4.into()),
        }
    }
}

impl From<source_contract::functions::TakeBid> for TakeBid {
    fn from(raw: source_contract::functions::TakeBid) -> Self {
        let orders = raw.inputs.0.into_iter().map(|input| input.into()).collect();

        let exchanges = raw.inputs.1.into_iter().map(|input| input.into()).collect();

        Self {
            orders,
            exchanges,
            taker_fee: Some(raw.inputs.2.into()),
            signatures: raw.inputs.3.into(),
        }
    }
}

impl From<source_contract::functions::TakeBidSingle> for TakeBidSingle {
    fn from(raw: source_contract::functions::TakeBidSingle) -> Self {
        Self {
            order: Some(raw.inputs.0.into()),
            exchange: Some(raw.inputs.1.into()),
            taker_fee: Some(raw.inputs.2.into()),
            signature: raw.inputs.3.into(),
        }
    }
}

impl From<BigInt> for AssetType {
    fn from(value: BigInt) -> Self {
        match value.to_u64() {
            0 => AssetType::Erc721,
            1 => AssetType::Erc1155,
            _ => panic!("Invalid asset type"),
        }
    }
}

impl From<BigInt> for OrderType {
    fn from(value: BigInt) -> Self {
        match value.to_u64() {
            0 => OrderType::Ask,
            1 => OrderType::Bid,
            _ => panic!("Invalid asset type"),
        }
    }
}

impl
    From<(
        BigInt,
        Vec<[u8; 32usize]>,
        (BigInt, BigInt, BigInt, BigInt),
        (BigInt, BigInt),
    )> for Exchange
{
    fn from(
        tuple: (
            BigInt,
            Vec<[u8; 32usize]>,
            (BigInt, BigInt, BigInt, BigInt),
            (BigInt, BigInt),
        ),
    ) -> Self {
        let (index, proof, listing, taker) = tuple;
        Exchange {
            index: index.to_u64() as i64,
            proof: proof.into_iter().map(|array| array.to_vec()).collect(),
            listing: Some(listing.into()),
            taker: Some(taker.into()),
        }
    }
}

impl From<(BigInt, BigInt, BigInt, BigInt)> for Listing {
    fn from(tuple: (BigInt, BigInt, BigInt, BigInt)) -> Self {
        let (index, token_id, amount, price) = tuple;
        Listing {
            index: index.to_u64() as i64,
            token_id: token_id.to_signed_bytes_be(),
            amount: amount.to_u64() as i64,
            price: price.to_signed_bytes_be(),
        }
    }
}

impl From<(BigInt, BigInt)> for Taker {
    fn from(tuple: (BigInt, BigInt)) -> Self {
        let (token_id, amount) = tuple;
        Taker {
            token_id: token_id.to_signed_bytes_be(),
            amount: amount.to_u64() as i64,
        }
    }
}

impl
    From<(
        Vec<u8>,
        Vec<u8>,
        [u8; 32usize],
        BigInt,
        BigInt,
        BigInt,
        (Vec<u8>, BigInt),
        BigInt,
    )> for Order
{
    fn from(
        tuple: (
            Vec<u8>,
            Vec<u8>,
            [u8; 32usize],
            BigInt,
            BigInt,
            BigInt,
            (Vec<u8>, BigInt),
            BigInt,
        ),
    ) -> Self {
        let (
            trader,
            collection,
            listings_root,
            number_of_listings,
            expiration_time,
            asset_type,
            maker_fee,
            _salt,
        ) = tuple;
        Order {
            trader: Some(Hex::encode(&trader).into()),
            collection: Some(Hex::encode(&collection).into()),
            listings_root: Hex::encode(listings_root.to_vec()).into(),
            number_of_listings: number_of_listings.to_u64() as i64,
            expiration_time: expiration_time.to_u64() as i64,
            asset_type: asset_type.into(),
            maker_fee: Some(maker_fee.into()),
            salt: 0, // OUPS,
        }
    }
}

impl From<(Vec<u8>, BigInt, BigInt, Vec<u8>, BigInt)> for Transfer {
    fn from(tuple: (Vec<u8>, BigInt, BigInt, Vec<u8>, BigInt)) -> Self {
        let (trader, id, amount, collection, asset_type) = tuple;
        Transfer {
            trader: Some(Hex::encode(trader).into()),
            id: id.to_u64() as i64,
            amount: amount.to_u64() as i64,
            collection: Some(Hex::encode(collection).into()),
            asset_type: asset_type.to_i32().into(),
        }
    }
}

// struct FungibleTransfers {
//     totalProtocolFee: BigInt,
//     totalSellerTransfer: BigInt,
//     totalTakerFee: BigInt,
//     feeRecipientId: BigInt,
//     makerId: BigInt,
//     feeRecipients: Vec<Address>,
//     makers: Vec<Address>,
//     makerTransfers: Vec<BigInt>,
//     feeTransfers: Vec<BigInt>,
//     executions: Vec<AtomicExecution>,
// }

impl From<(FeeRate, FeeRate)> for Fees {
    fn from(tuple: (FeeRate, FeeRate)) -> Self {
        let (protocol_fee, taker_fee) = tuple;
        Fees {
            protocol_fee: Some(protocol_fee),
            taker_fee: Some(taker_fee),
        }
    }
}

impl From<(Vec<u8>, BigInt)> for FeeRate {
    fn from(tuple: (Vec<u8>, BigInt)) -> Self {
        let (address, rate) = tuple;
        FeeRate {
            recipient: Some(Address::from(address)),
            rate: rate.to_u64(),
        }
    }
}

impl From<Vec<u8>> for Address {
    fn from(address: Vec<u8>) -> Self {
        Address { address }
    }
}


impl From<std::string::String> for Address {
    fn from(address: String) -> Self {
        Address { address: address.from_hex().unwrap() }
    }
}


impl Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let hex_address = Hex::encode(&self.address);
        serializer.serialize_str(&hex_address)
    }
}


impl From<Address> for String {
    fn from(address: Address) -> Self {
        Hex::encode(&address.address).to_string()
    }
}
