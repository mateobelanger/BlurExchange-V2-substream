    const INTERNAL_ERR: &'static str = "`ethabi_derive` internal error";
    /// Contract's functions.
    #[allow(dead_code, unused_imports, unused_variables)]
    pub mod functions {
        use super::INTERNAL_ERR;
        #[derive(Debug, Clone, PartialEq)]
        pub struct AcceptOwnership {}
        impl AcceptOwnership {
            const METHOD_ID: [u8; 4] = [121u8, 186u8, 80u8, 151u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Ok(Self {})
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for AcceptOwnership {
            const NAME: &'static str = "acceptOwnership";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct AmountTaken {
            pub param0: Vec<u8>,
            pub param1: [u8; 32usize],
            pub param2: substreams::scalar::BigInt,
        }
        impl AmountTaken {
            const METHOD_ID: [u8; 4] = [60u8, 42u8, 182u8, 191u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Address,
                            ethabi::ParamType::FixedBytes(32usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    param0: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    param1: {
                        let mut result = [0u8; 32];
                        let v = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                    param2: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.param0),
                        ),
                        ethabi::Token::FixedBytes(self.param1.as_ref().to_vec()),
                        ethabi::Token::Uint(
                            ethabi::Uint::from_big_endian(
                                match self.param2.clone().to_bytes_be() {
                                    (num_bigint::Sign::Plus, bytes) => bytes,
                                    (num_bigint::Sign::NoSign, bytes) => bytes,
                                    (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported")
                                    }
                                }
                                    .as_slice(),
                            ),
                        ),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<substreams::scalar::BigInt, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<substreams::scalar::BigInt, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok({
                    let mut v = [0 as u8; 32];
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_uint()
                        .expect(INTERNAL_ERR)
                        .to_big_endian(v.as_mut_slice());
                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                })
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<substreams::scalar::BigInt> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for AmountTaken {
            const NAME: &'static str = "amountTaken";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<substreams::scalar::BigInt>
        for AmountTaken {
            fn output(data: &[u8]) -> Result<substreams::scalar::BigInt, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct BlockRange {}
        impl BlockRange {
            const METHOD_ID: [u8; 4] = [164u8, 178u8, 198u8, 116u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Ok(Self {})
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<substreams::scalar::BigInt, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<substreams::scalar::BigInt, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok({
                    let mut v = [0 as u8; 32];
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_uint()
                        .expect(INTERNAL_ERR)
                        .to_big_endian(v.as_mut_slice());
                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                })
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<substreams::scalar::BigInt> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for BlockRange {
            const NAME: &'static str = "blockRange";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<substreams::scalar::BigInt>
        for BlockRange {
            fn output(data: &[u8]) -> Result<substreams::scalar::BigInt, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct CancelTrades {
            pub cancels: Vec<
                ([u8; 32usize], substreams::scalar::BigInt, substreams::scalar::BigInt),
            >,
        }
        impl CancelTrades {
            const METHOD_ID: [u8; 4] = [63u8, 143u8, 194u8, 51u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(
                                Box::new(
                                    ethabi::ParamType::Tuple(
                                        vec![
                                            ethabi::ParamType::FixedBytes(32usize),
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Uint(256usize)
                                        ],
                                    ),
                                ),
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    cancels: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            let tuple_elements = inner.into_tuple().expect(INTERNAL_ERR);
                            (
                                {
                                    let mut result = [0u8; 32];
                                    let v = tuple_elements[0usize]
                                        .clone()
                                        .into_fixed_bytes()
                                        .expect(INTERNAL_ERR);
                                    result.copy_from_slice(&v);
                                    result
                                },
                                {
                                    let mut v = [0 as u8; 32];
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_uint()
                                        .expect(INTERNAL_ERR)
                                        .to_big_endian(v.as_mut_slice());
                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                },
                                {
                                    let mut v = [0 as u8; 32];
                                    tuple_elements[2usize]
                                        .clone()
                                        .into_uint()
                                        .expect(INTERNAL_ERR)
                                        .to_big_endian(v.as_mut_slice());
                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                },
                            )
                        })
                        .collect(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        {
                            let v = self
                                .cancels
                                .iter()
                                .map(|inner| ethabi::Token::Tuple(
                                    vec![
                                        ethabi::Token::FixedBytes(inner.0.as_ref().to_vec()),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.1.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.2.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),)
                                    ],
                                ))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for CancelTrades {
            const NAME: &'static str = "cancelTrades";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Governor {}
        impl Governor {
            const METHOD_ID: [u8; 4] = [12u8, 52u8, 10u8, 36u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Ok(Self {})
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Vec<u8>, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<Vec<u8>, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<Vec<u8>> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for Governor {
            const NAME: &'static str = "governor";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<Vec<u8>> for Governor {
            fn output(data: &[u8]) -> Result<Vec<u8>, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct HashListing {
            pub listing: (
                substreams::scalar::BigInt,
                substreams::scalar::BigInt,
                substreams::scalar::BigInt,
                substreams::scalar::BigInt,
            ),
        }
        impl HashListing {
            const METHOD_ID: [u8; 4] = [135u8, 204u8, 105u8, 77u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Tuple(
                                vec![
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize)
                                ],
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    listing: {
                        let tuple_elements = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_tuple()
                            .expect(INTERNAL_ERR);
                        (
                            {
                                let mut v = [0 as u8; 32];
                                tuple_elements[0usize]
                                    .clone()
                                    .into_uint()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                            },
                            {
                                let mut v = [0 as u8; 32];
                                tuple_elements[1usize]
                                    .clone()
                                    .into_uint()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                            },
                            {
                                let mut v = [0 as u8; 32];
                                tuple_elements[2usize]
                                    .clone()
                                    .into_uint()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                            },
                            {
                                let mut v = [0 as u8; 32];
                                tuple_elements[3usize]
                                    .clone()
                                    .into_uint()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                            },
                        )
                    },
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Tuple(
                            vec![
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .listing.0.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .listing.1.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .listing.2.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .listing.3.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)
                            ],
                        ),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<[u8; 32usize], String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<[u8; 32usize], String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::FixedBytes(32usize)],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok({
                    let mut result = [0u8; 32];
                    let v = values
                        .pop()
                        .expect("one output data should have existed")
                        .into_fixed_bytes()
                        .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                })
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<[u8; 32usize]> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for HashListing {
            const NAME: &'static str = "hashListing";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<[u8; 32usize]> for HashListing {
            fn output(data: &[u8]) -> Result<[u8; 32usize], String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct HashOrder {
            pub order: (
                Vec<u8>,
                Vec<u8>,
                [u8; 32usize],
                substreams::scalar::BigInt,
                substreams::scalar::BigInt,
                substreams::scalar::BigInt,
                (Vec<u8>, substreams::scalar::BigInt),
                substreams::scalar::BigInt,
            ),
            pub order_type: substreams::scalar::BigInt,
        }
        impl HashOrder {
            const METHOD_ID: [u8; 4] = [81u8, 17u8, 79u8, 250u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Tuple(
                                vec![
                                    ethabi::ParamType::Address, ethabi::ParamType::Address,
                                    ethabi::ParamType::FixedBytes(32usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(8usize),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Address,
                                    ethabi::ParamType::Uint(16usize)]),
                                    ethabi::ParamType::Uint(256usize)
                                ],
                            ),
                            ethabi::ParamType::Uint(8usize),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    order: {
                        let tuple_elements = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_tuple()
                            .expect(INTERNAL_ERR);
                        (
                            tuple_elements[0usize]
                                .clone()
                                .into_address()
                                .expect(INTERNAL_ERR)
                                .as_bytes()
                                .to_vec(),
                            tuple_elements[1usize]
                                .clone()
                                .into_address()
                                .expect(INTERNAL_ERR)
                                .as_bytes()
                                .to_vec(),
                            {
                                let mut result = [0u8; 32];
                                let v = tuple_elements[2usize]
                                    .clone()
                                    .into_fixed_bytes()
                                    .expect(INTERNAL_ERR);
                                result.copy_from_slice(&v);
                                result
                            },
                            {
                                let mut v = [0 as u8; 32];
                                tuple_elements[3usize]
                                    .clone()
                                    .into_uint()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                            },
                            {
                                let mut v = [0 as u8; 32];
                                tuple_elements[4usize]
                                    .clone()
                                    .into_uint()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                            },
                            {
                                let mut v = [0 as u8; 32];
                                tuple_elements[5usize]
                                    .clone()
                                    .into_uint()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                            },
                            {
                                let tuple_elements = tuple_elements[6usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec(),
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                )
                            },
                            {
                                let mut v = [0 as u8; 32];
                                tuple_elements[7usize]
                                    .clone()
                                    .into_uint()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                            },
                        )
                    },
                    order_type: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Tuple(
                            vec![
                                ethabi::Token::Address(ethabi::Address::from_slice(& self
                                .order.0)),
                                ethabi::Token::Address(ethabi::Address::from_slice(& self
                                .order.1)), ethabi::Token::FixedBytes(self.order.2.as_ref()
                                .to_vec()),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .order.3.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .order.4.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .order.5.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Tuple(vec![ethabi::Token::Address(ethabi::Address::from_slice(&
                                self.order.6.0)),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .order.6.1.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)]),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .order.7.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)
                            ],
                        ),
                        ethabi::Token::Uint(
                            ethabi::Uint::from_big_endian(
                                match self.order_type.clone().to_bytes_be() {
                                    (num_bigint::Sign::Plus, bytes) => bytes,
                                    (num_bigint::Sign::NoSign, bytes) => bytes,
                                    (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported")
                                    }
                                }
                                    .as_slice(),
                            ),
                        ),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<[u8; 32usize], String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<[u8; 32usize], String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::FixedBytes(32usize)],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok({
                    let mut result = [0u8; 32];
                    let v = values
                        .pop()
                        .expect("one output data should have existed")
                        .into_fixed_bytes()
                        .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                })
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<[u8; 32usize]> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for HashOrder {
            const NAME: &'static str = "hashOrder";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<[u8; 32usize]> for HashOrder {
            fn output(data: &[u8]) -> Result<[u8; 32usize], String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct HashTakeAsk {
            pub inputs: (
                Vec<
                    (
                        Vec<u8>,
                        Vec<u8>,
                        [u8; 32usize],
                        substreams::scalar::BigInt,
                        substreams::scalar::BigInt,
                        substreams::scalar::BigInt,
                        (Vec<u8>, substreams::scalar::BigInt),
                        substreams::scalar::BigInt,
                    ),
                >,
                Vec<
                    (
                        substreams::scalar::BigInt,
                        Vec<[u8; 32usize]>,
                        (
                            substreams::scalar::BigInt,
                            substreams::scalar::BigInt,
                            substreams::scalar::BigInt,
                            substreams::scalar::BigInt,
                        ),
                        (substreams::scalar::BigInt, substreams::scalar::BigInt),
                    ),
                >,
                (Vec<u8>, substreams::scalar::BigInt),
                Vec<u8>,
                Vec<u8>,
            ),
            pub u_caller: Vec<u8>,
        }
        impl HashTakeAsk {
            const METHOD_ID: [u8; 4] = [207u8, 107u8, 15u8, 82u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Tuple(
                                vec![
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::Address,
                                    ethabi::ParamType::Address,
                                    ethabi::ParamType::FixedBytes(32usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(8usize),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Address,
                                    ethabi::ParamType::Uint(16usize)]),
                                    ethabi::ParamType::Uint(256usize)]))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::FixedBytes(32usize))),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize)]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize)])]))),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Address,
                                    ethabi::ParamType::Uint(16usize)]),
                                    ethabi::ParamType::Bytes, ethabi::ParamType::Address
                                ],
                            ),
                            ethabi::ParamType::Address,
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    inputs: {
                        let tuple_elements = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_tuple()
                            .expect(INTERNAL_ERR);
                        (
                            tuple_elements[0usize]
                                .clone()
                                .into_array()
                                .expect(INTERNAL_ERR)
                                .into_iter()
                                .map(|inner| {
                                    let tuple_elements = inner
                                        .into_tuple()
                                        .expect(INTERNAL_ERR);
                                    (
                                        tuple_elements[0usize]
                                            .clone()
                                            .into_address()
                                            .expect(INTERNAL_ERR)
                                            .as_bytes()
                                            .to_vec(),
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_address()
                                            .expect(INTERNAL_ERR)
                                            .as_bytes()
                                            .to_vec(),
                                        {
                                            let mut result = [0u8; 32];
                                            let v = tuple_elements[2usize]
                                                .clone()
                                                .into_fixed_bytes()
                                                .expect(INTERNAL_ERR);
                                            result.copy_from_slice(&v);
                                            result
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[3usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[4usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[5usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        {
                                            let tuple_elements = tuple_elements[6usize]
                                                .clone()
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_address()
                                                    .expect(INTERNAL_ERR)
                                                    .as_bytes()
                                                    .to_vec(),
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[1usize]
                                                        .clone()
                                                        .into_uint()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                },
                                            )
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[7usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                    )
                                })
                                .collect(),
                            tuple_elements[1usize]
                                .clone()
                                .into_array()
                                .expect(INTERNAL_ERR)
                                .into_iter()
                                .map(|inner| {
                                    let tuple_elements = inner
                                        .into_tuple()
                                        .expect(INTERNAL_ERR);
                                    (
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[0usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_array()
                                            .expect(INTERNAL_ERR)
                                            .into_iter()
                                            .map(|inner| {
                                                let mut result = [0u8; 32];
                                                let v = inner.into_fixed_bytes().expect(INTERNAL_ERR);
                                                result.copy_from_slice(&v);
                                                result
                                            })
                                            .collect(),
                                        {
                                            let tuple_elements = tuple_elements[2usize]
                                                .clone()
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[0usize]
                                                        .clone()
                                                        .into_uint()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                },
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[1usize]
                                                        .clone()
                                                        .into_uint()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                },
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[2usize]
                                                        .clone()
                                                        .into_uint()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                },
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[3usize]
                                                        .clone()
                                                        .into_uint()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                },
                                            )
                                        },
                                        {
                                            let tuple_elements = tuple_elements[3usize]
                                                .clone()
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[0usize]
                                                        .clone()
                                                        .into_uint()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                },
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[1usize]
                                                        .clone()
                                                        .into_uint()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                },
                                            )
                                        },
                                    )
                                })
                                .collect(),
                            {
                                let tuple_elements = tuple_elements[2usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec(),
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                )
                            },
                            tuple_elements[3usize]
                                .clone()
                                .into_bytes()
                                .expect(INTERNAL_ERR),
                            tuple_elements[4usize]
                                .clone()
                                .into_address()
                                .expect(INTERNAL_ERR)
                                .as_bytes()
                                .to_vec(),
                        )
                    },
                    u_caller: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Tuple(
                            vec![
                                { let v = self.inputs.0.iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::Address(ethabi::Address::from_slice(&
                                inner.0)),
                                ethabi::Token::Address(ethabi::Address::from_slice(& inner
                                .1)), ethabi::Token::FixedBytes(inner.2.as_ref().to_vec()),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.3.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.4.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.5.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Tuple(vec![ethabi::Token::Address(ethabi::Address::from_slice(&
                                inner.6.0)),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.6.1.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)]),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.7.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)])).collect(); ethabi::Token::Array(v) }, {
                                let v = self.inputs.1.iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.0.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),), { let v = inner.1.iter().map(| inner |
                                ethabi::Token::FixedBytes(inner.as_ref().to_vec()))
                                .collect(); ethabi::Token::Array(v) },
                                ethabi::Token::Tuple(vec![ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.2.0.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.2.1.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.2.2.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.2.3.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)]),
                                ethabi::Token::Tuple(vec![ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.3.0.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.3.1.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)])])).collect(); ethabi::Token::Array(v) },
                                ethabi::Token::Tuple(vec![ethabi::Token::Address(ethabi::Address::from_slice(&
                                self.inputs.2.0)),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.2.1.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)]), ethabi::Token::Bytes(self.inputs.3
                                .clone()),
                                ethabi::Token::Address(ethabi::Address::from_slice(& self
                                .inputs.4))
                            ],
                        ),
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.u_caller),
                        ),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<[u8; 32usize], String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<[u8; 32usize], String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::FixedBytes(32usize)],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok({
                    let mut result = [0u8; 32];
                    let v = values
                        .pop()
                        .expect("one output data should have existed")
                        .into_fixed_bytes()
                        .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                })
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<[u8; 32usize]> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for HashTakeAsk {
            const NAME: &'static str = "hashTakeAsk";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<[u8; 32usize]> for HashTakeAsk {
            fn output(data: &[u8]) -> Result<[u8; 32usize], String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct HashTakeAskSingle {
            pub inputs: (
                (
                    Vec<u8>,
                    Vec<u8>,
                    [u8; 32usize],
                    substreams::scalar::BigInt,
                    substreams::scalar::BigInt,
                    substreams::scalar::BigInt,
                    (Vec<u8>, substreams::scalar::BigInt),
                    substreams::scalar::BigInt,
                ),
                (
                    substreams::scalar::BigInt,
                    Vec<[u8; 32usize]>,
                    (
                        substreams::scalar::BigInt,
                        substreams::scalar::BigInt,
                        substreams::scalar::BigInt,
                        substreams::scalar::BigInt,
                    ),
                    (substreams::scalar::BigInt, substreams::scalar::BigInt),
                ),
                (Vec<u8>, substreams::scalar::BigInt),
                Vec<u8>,
                Vec<u8>,
            ),
            pub u_caller: Vec<u8>,
        }
        impl HashTakeAskSingle {
            const METHOD_ID: [u8; 4] = [87u8, 144u8, 119u8, 184u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Tuple(
                                vec![
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Address,
                                    ethabi::ParamType::Address,
                                    ethabi::ParamType::FixedBytes(32usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(8usize),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Address,
                                    ethabi::ParamType::Uint(16usize)]),
                                    ethabi::ParamType::Uint(256usize)]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::FixedBytes(32usize))),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize)]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize)])]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Address,
                                    ethabi::ParamType::Uint(16usize)]),
                                    ethabi::ParamType::Bytes, ethabi::ParamType::Address
                                ],
                            ),
                            ethabi::ParamType::Address,
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    inputs: {
                        let tuple_elements = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_tuple()
                            .expect(INTERNAL_ERR);
                        (
                            {
                                let tuple_elements = tuple_elements[0usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec(),
                                    {
                                        let mut result = [0u8; 32];
                                        let v = tuple_elements[2usize]
                                            .clone()
                                            .into_fixed_bytes()
                                            .expect(INTERNAL_ERR);
                                        result.copy_from_slice(&v);
                                        result
                                    },
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[3usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[4usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[5usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                    {
                                        let tuple_elements = tuple_elements[6usize]
                                            .clone()
                                            .into_tuple()
                                            .expect(INTERNAL_ERR);
                                        (
                                            tuple_elements[0usize]
                                                .clone()
                                                .into_address()
                                                .expect(INTERNAL_ERR)
                                                .as_bytes()
                                                .to_vec(),
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                            },
                                        )
                                    },
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[7usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                )
                            },
                            {
                                let tuple_elements = tuple_elements[1usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[0usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let mut result = [0u8; 32];
                                            let v = inner.into_fixed_bytes().expect(INTERNAL_ERR);
                                            result.copy_from_slice(&v);
                                            result
                                        })
                                        .collect(),
                                    {
                                        let tuple_elements = tuple_elements[2usize]
                                            .clone()
                                            .into_tuple()
                                            .expect(INTERNAL_ERR);
                                        (
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                            },
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                            },
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[2usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                            },
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[3usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                            },
                                        )
                                    },
                                    {
                                        let tuple_elements = tuple_elements[3usize]
                                            .clone()
                                            .into_tuple()
                                            .expect(INTERNAL_ERR);
                                        (
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                            },
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                            },
                                        )
                                    },
                                )
                            },
                            {
                                let tuple_elements = tuple_elements[2usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec(),
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                )
                            },
                            tuple_elements[3usize]
                                .clone()
                                .into_bytes()
                                .expect(INTERNAL_ERR),
                            tuple_elements[4usize]
                                .clone()
                                .into_address()
                                .expect(INTERNAL_ERR)
                                .as_bytes()
                                .to_vec(),
                        )
                    },
                    u_caller: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Tuple(
                            vec![
                                ethabi::Token::Tuple(vec![ethabi::Token::Address(ethabi::Address::from_slice(&
                                self.inputs.0.0)),
                                ethabi::Token::Address(ethabi::Address::from_slice(& self
                                .inputs.0.1)), ethabi::Token::FixedBytes(self.inputs.0.2
                                .as_ref().to_vec()),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.0.3.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.0.4.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.0.5.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Tuple(vec![ethabi::Token::Address(ethabi::Address::from_slice(&
                                self.inputs.0.6.0)),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.0.6.1.clone().to_bytes_be() {
                                (num_bigint::Sign::Plus, bytes) => bytes,
                                (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)]),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.0.7.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)]),
                                ethabi::Token::Tuple(vec![ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                self.inputs.1.0.clone().to_bytes_be() {
                                (num_bigint::Sign::Plus, bytes) => bytes,
                                (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),), { let v = self.inputs.1.1.iter().map(|
                                inner | ethabi::Token::FixedBytes(inner.as_ref().to_vec()))
                                .collect(); ethabi::Token::Array(v) },
                                ethabi::Token::Tuple(vec![ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                self.inputs.1.2.0.clone().to_bytes_be() {
                                (num_bigint::Sign::Plus, bytes) => bytes,
                                (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.1.2.1.clone().to_bytes_be() {
                                (num_bigint::Sign::Plus, bytes) => bytes,
                                (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.1.2.2.clone().to_bytes_be() {
                                (num_bigint::Sign::Plus, bytes) => bytes,
                                (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.1.2.3.clone().to_bytes_be() {
                                (num_bigint::Sign::Plus, bytes) => bytes,
                                (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)]),
                                ethabi::Token::Tuple(vec![ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                self.inputs.1.3.0.clone().to_bytes_be() {
                                (num_bigint::Sign::Plus, bytes) => bytes,
                                (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.1.3.1.clone().to_bytes_be() {
                                (num_bigint::Sign::Plus, bytes) => bytes,
                                (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)])]),
                                ethabi::Token::Tuple(vec![ethabi::Token::Address(ethabi::Address::from_slice(&
                                self.inputs.2.0)),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.2.1.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)]), ethabi::Token::Bytes(self.inputs.3
                                .clone()),
                                ethabi::Token::Address(ethabi::Address::from_slice(& self
                                .inputs.4))
                            ],
                        ),
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.u_caller),
                        ),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<[u8; 32usize], String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<[u8; 32usize], String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::FixedBytes(32usize)],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok({
                    let mut result = [0u8; 32];
                    let v = values
                        .pop()
                        .expect("one output data should have existed")
                        .into_fixed_bytes()
                        .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                })
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<[u8; 32usize]> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for HashTakeAskSingle {
            const NAME: &'static str = "hashTakeAskSingle";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<[u8; 32usize]>
        for HashTakeAskSingle {
            fn output(data: &[u8]) -> Result<[u8; 32usize], String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct HashTakeBid {
            pub inputs: (
                Vec<
                    (
                        Vec<u8>,
                        Vec<u8>,
                        [u8; 32usize],
                        substreams::scalar::BigInt,
                        substreams::scalar::BigInt,
                        substreams::scalar::BigInt,
                        (Vec<u8>, substreams::scalar::BigInt),
                        substreams::scalar::BigInt,
                    ),
                >,
                Vec<
                    (
                        substreams::scalar::BigInt,
                        Vec<[u8; 32usize]>,
                        (
                            substreams::scalar::BigInt,
                            substreams::scalar::BigInt,
                            substreams::scalar::BigInt,
                            substreams::scalar::BigInt,
                        ),
                        (substreams::scalar::BigInt, substreams::scalar::BigInt),
                    ),
                >,
                (Vec<u8>, substreams::scalar::BigInt),
                Vec<u8>,
            ),
            pub u_caller: Vec<u8>,
        }
        impl HashTakeBid {
            const METHOD_ID: [u8; 4] = [171u8, 227u8, 187u8, 102u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Tuple(
                                vec![
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::Address,
                                    ethabi::ParamType::Address,
                                    ethabi::ParamType::FixedBytes(32usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(8usize),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Address,
                                    ethabi::ParamType::Uint(16usize)]),
                                    ethabi::ParamType::Uint(256usize)]))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::FixedBytes(32usize))),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize)]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize)])]))),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Address,
                                    ethabi::ParamType::Uint(16usize)]), ethabi::ParamType::Bytes
                                ],
                            ),
                            ethabi::ParamType::Address,
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    inputs: {
                        let tuple_elements = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_tuple()
                            .expect(INTERNAL_ERR);
                        (
                            tuple_elements[0usize]
                                .clone()
                                .into_array()
                                .expect(INTERNAL_ERR)
                                .into_iter()
                                .map(|inner| {
                                    let tuple_elements = inner
                                        .into_tuple()
                                        .expect(INTERNAL_ERR);
                                    (
                                        tuple_elements[0usize]
                                            .clone()
                                            .into_address()
                                            .expect(INTERNAL_ERR)
                                            .as_bytes()
                                            .to_vec(),
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_address()
                                            .expect(INTERNAL_ERR)
                                            .as_bytes()
                                            .to_vec(),
                                        {
                                            let mut result = [0u8; 32];
                                            let v = tuple_elements[2usize]
                                                .clone()
                                                .into_fixed_bytes()
                                                .expect(INTERNAL_ERR);
                                            result.copy_from_slice(&v);
                                            result
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[3usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[4usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[5usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        {
                                            let tuple_elements = tuple_elements[6usize]
                                                .clone()
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_address()
                                                    .expect(INTERNAL_ERR)
                                                    .as_bytes()
                                                    .to_vec(),
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[1usize]
                                                        .clone()
                                                        .into_uint()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                },
                                            )
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[7usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                    )
                                })
                                .collect(),
                            tuple_elements[1usize]
                                .clone()
                                .into_array()
                                .expect(INTERNAL_ERR)
                                .into_iter()
                                .map(|inner| {
                                    let tuple_elements = inner
                                        .into_tuple()
                                        .expect(INTERNAL_ERR);
                                    (
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[0usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_array()
                                            .expect(INTERNAL_ERR)
                                            .into_iter()
                                            .map(|inner| {
                                                let mut result = [0u8; 32];
                                                let v = inner.into_fixed_bytes().expect(INTERNAL_ERR);
                                                result.copy_from_slice(&v);
                                                result
                                            })
                                            .collect(),
                                        {
                                            let tuple_elements = tuple_elements[2usize]
                                                .clone()
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[0usize]
                                                        .clone()
                                                        .into_uint()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                },
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[1usize]
                                                        .clone()
                                                        .into_uint()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                },
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[2usize]
                                                        .clone()
                                                        .into_uint()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                },
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[3usize]
                                                        .clone()
                                                        .into_uint()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                },
                                            )
                                        },
                                        {
                                            let tuple_elements = tuple_elements[3usize]
                                                .clone()
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[0usize]
                                                        .clone()
                                                        .into_uint()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                },
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[1usize]
                                                        .clone()
                                                        .into_uint()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                },
                                            )
                                        },
                                    )
                                })
                                .collect(),
                            {
                                let tuple_elements = tuple_elements[2usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec(),
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                )
                            },
                            tuple_elements[3usize]
                                .clone()
                                .into_bytes()
                                .expect(INTERNAL_ERR),
                        )
                    },
                    u_caller: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Tuple(
                            vec![
                                { let v = self.inputs.0.iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::Address(ethabi::Address::from_slice(&
                                inner.0)),
                                ethabi::Token::Address(ethabi::Address::from_slice(& inner
                                .1)), ethabi::Token::FixedBytes(inner.2.as_ref().to_vec()),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.3.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.4.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.5.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Tuple(vec![ethabi::Token::Address(ethabi::Address::from_slice(&
                                inner.6.0)),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.6.1.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)]),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.7.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)])).collect(); ethabi::Token::Array(v) }, {
                                let v = self.inputs.1.iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.0.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),), { let v = inner.1.iter().map(| inner |
                                ethabi::Token::FixedBytes(inner.as_ref().to_vec()))
                                .collect(); ethabi::Token::Array(v) },
                                ethabi::Token::Tuple(vec![ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.2.0.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.2.1.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.2.2.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.2.3.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)]),
                                ethabi::Token::Tuple(vec![ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.3.0.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.3.1.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)])])).collect(); ethabi::Token::Array(v) },
                                ethabi::Token::Tuple(vec![ethabi::Token::Address(ethabi::Address::from_slice(&
                                self.inputs.2.0)),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.2.1.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)]), ethabi::Token::Bytes(self.inputs.3
                                .clone())
                            ],
                        ),
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.u_caller),
                        ),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<[u8; 32usize], String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<[u8; 32usize], String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::FixedBytes(32usize)],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok({
                    let mut result = [0u8; 32];
                    let v = values
                        .pop()
                        .expect("one output data should have existed")
                        .into_fixed_bytes()
                        .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                })
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<[u8; 32usize]> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for HashTakeBid {
            const NAME: &'static str = "hashTakeBid";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<[u8; 32usize]> for HashTakeBid {
            fn output(data: &[u8]) -> Result<[u8; 32usize], String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct HashTakeBidSingle {
            pub inputs: (
                (
                    Vec<u8>,
                    Vec<u8>,
                    [u8; 32usize],
                    substreams::scalar::BigInt,
                    substreams::scalar::BigInt,
                    substreams::scalar::BigInt,
                    (Vec<u8>, substreams::scalar::BigInt),
                    substreams::scalar::BigInt,
                ),
                (
                    substreams::scalar::BigInt,
                    Vec<[u8; 32usize]>,
                    (
                        substreams::scalar::BigInt,
                        substreams::scalar::BigInt,
                        substreams::scalar::BigInt,
                        substreams::scalar::BigInt,
                    ),
                    (substreams::scalar::BigInt, substreams::scalar::BigInt),
                ),
                (Vec<u8>, substreams::scalar::BigInt),
                Vec<u8>,
            ),
            pub u_caller: Vec<u8>,
        }
        impl HashTakeBidSingle {
            const METHOD_ID: [u8; 4] = [145u8, 190u8, 168u8, 64u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Tuple(
                                vec![
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Address,
                                    ethabi::ParamType::Address,
                                    ethabi::ParamType::FixedBytes(32usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(8usize),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Address,
                                    ethabi::ParamType::Uint(16usize)]),
                                    ethabi::ParamType::Uint(256usize)]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::FixedBytes(32usize))),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize)]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize)])]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Address,
                                    ethabi::ParamType::Uint(16usize)]), ethabi::ParamType::Bytes
                                ],
                            ),
                            ethabi::ParamType::Address,
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    inputs: {
                        let tuple_elements = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_tuple()
                            .expect(INTERNAL_ERR);
                        (
                            {
                                let tuple_elements = tuple_elements[0usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec(),
                                    {
                                        let mut result = [0u8; 32];
                                        let v = tuple_elements[2usize]
                                            .clone()
                                            .into_fixed_bytes()
                                            .expect(INTERNAL_ERR);
                                        result.copy_from_slice(&v);
                                        result
                                    },
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[3usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[4usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[5usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                    {
                                        let tuple_elements = tuple_elements[6usize]
                                            .clone()
                                            .into_tuple()
                                            .expect(INTERNAL_ERR);
                                        (
                                            tuple_elements[0usize]
                                                .clone()
                                                .into_address()
                                                .expect(INTERNAL_ERR)
                                                .as_bytes()
                                                .to_vec(),
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                            },
                                        )
                                    },
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[7usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                )
                            },
                            {
                                let tuple_elements = tuple_elements[1usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[0usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let mut result = [0u8; 32];
                                            let v = inner.into_fixed_bytes().expect(INTERNAL_ERR);
                                            result.copy_from_slice(&v);
                                            result
                                        })
                                        .collect(),
                                    {
                                        let tuple_elements = tuple_elements[2usize]
                                            .clone()
                                            .into_tuple()
                                            .expect(INTERNAL_ERR);
                                        (
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                            },
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                            },
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[2usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                            },
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[3usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                            },
                                        )
                                    },
                                    {
                                        let tuple_elements = tuple_elements[3usize]
                                            .clone()
                                            .into_tuple()
                                            .expect(INTERNAL_ERR);
                                        (
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                            },
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                            },
                                        )
                                    },
                                )
                            },
                            {
                                let tuple_elements = tuple_elements[2usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec(),
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                )
                            },
                            tuple_elements[3usize]
                                .clone()
                                .into_bytes()
                                .expect(INTERNAL_ERR),
                        )
                    },
                    u_caller: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Tuple(
                            vec![
                                ethabi::Token::Tuple(vec![ethabi::Token::Address(ethabi::Address::from_slice(&
                                self.inputs.0.0)),
                                ethabi::Token::Address(ethabi::Address::from_slice(& self
                                .inputs.0.1)), ethabi::Token::FixedBytes(self.inputs.0.2
                                .as_ref().to_vec()),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.0.3.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.0.4.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.0.5.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Tuple(vec![ethabi::Token::Address(ethabi::Address::from_slice(&
                                self.inputs.0.6.0)),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.0.6.1.clone().to_bytes_be() {
                                (num_bigint::Sign::Plus, bytes) => bytes,
                                (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)]),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.0.7.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)]),
                                ethabi::Token::Tuple(vec![ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                self.inputs.1.0.clone().to_bytes_be() {
                                (num_bigint::Sign::Plus, bytes) => bytes,
                                (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),), { let v = self.inputs.1.1.iter().map(|
                                inner | ethabi::Token::FixedBytes(inner.as_ref().to_vec()))
                                .collect(); ethabi::Token::Array(v) },
                                ethabi::Token::Tuple(vec![ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                self.inputs.1.2.0.clone().to_bytes_be() {
                                (num_bigint::Sign::Plus, bytes) => bytes,
                                (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.1.2.1.clone().to_bytes_be() {
                                (num_bigint::Sign::Plus, bytes) => bytes,
                                (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.1.2.2.clone().to_bytes_be() {
                                (num_bigint::Sign::Plus, bytes) => bytes,
                                (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.1.2.3.clone().to_bytes_be() {
                                (num_bigint::Sign::Plus, bytes) => bytes,
                                (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)]),
                                ethabi::Token::Tuple(vec![ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                self.inputs.1.3.0.clone().to_bytes_be() {
                                (num_bigint::Sign::Plus, bytes) => bytes,
                                (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.1.3.1.clone().to_bytes_be() {
                                (num_bigint::Sign::Plus, bytes) => bytes,
                                (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)])]),
                                ethabi::Token::Tuple(vec![ethabi::Token::Address(ethabi::Address::from_slice(&
                                self.inputs.2.0)),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.2.1.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)]), ethabi::Token::Bytes(self.inputs.3
                                .clone())
                            ],
                        ),
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.u_caller),
                        ),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<[u8; 32usize], String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<[u8; 32usize], String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::FixedBytes(32usize)],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok({
                    let mut result = [0u8; 32];
                    let v = values
                        .pop()
                        .expect("one output data should have existed")
                        .into_fixed_bytes()
                        .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                })
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<[u8; 32usize]> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for HashTakeBidSingle {
            const NAME: &'static str = "hashTakeBidSingle";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<[u8; 32usize]>
        for HashTakeBidSingle {
            fn output(data: &[u8]) -> Result<[u8; 32usize], String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct IncrementNonce {}
        impl IncrementNonce {
            const METHOD_ID: [u8; 4] = [98u8, 124u8, 220u8, 185u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Ok(Self {})
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for IncrementNonce {
            const NAME: &'static str = "incrementNonce";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Information {}
        impl Information {
            const METHOD_ID: [u8; 4] = [244u8, 123u8, 119u8, 64u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Ok(Self {})
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<(String, [u8; 32usize]), String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<(String, [u8; 32usize]), String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::String,
                            ethabi::ParamType::FixedBytes(32usize),
                        ],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                values.reverse();
                Ok((
                    values.pop().expect(INTERNAL_ERR).into_string().expect(INTERNAL_ERR),
                    {
                        let mut result = [0u8; 32];
                        let v = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                ))
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<(String, [u8; 32usize])> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for Information {
            const NAME: &'static str = "information";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<(String, [u8; 32usize])>
        for Information {
            fn output(data: &[u8]) -> Result<(String, [u8; 32usize]), String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Initialize {}
        impl Initialize {
            const METHOD_ID: [u8; 4] = [129u8, 41u8, 252u8, 28u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Ok(Self {})
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for Initialize {
            const NAME: &'static str = "initialize";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Nonces {
            pub param0: Vec<u8>,
        }
        impl Nonces {
            const METHOD_ID: [u8; 4] = [126u8, 206u8, 190u8, 0u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    param0: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[ethabi::Token::Address(ethabi::Address::from_slice(&self.param0))],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<substreams::scalar::BigInt, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<substreams::scalar::BigInt, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok({
                    let mut v = [0 as u8; 32];
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_uint()
                        .expect(INTERNAL_ERR)
                        .to_big_endian(v.as_mut_slice());
                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                })
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<substreams::scalar::BigInt> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for Nonces {
            const NAME: &'static str = "nonces";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<substreams::scalar::BigInt>
        for Nonces {
            fn output(data: &[u8]) -> Result<substreams::scalar::BigInt, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Oracles {
            pub param0: Vec<u8>,
        }
        impl Oracles {
            const METHOD_ID: [u8; 4] = [173u8, 221u8, 80u8, 153u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    param0: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[ethabi::Token::Address(ethabi::Address::from_slice(&self.param0))],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<substreams::scalar::BigInt, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<substreams::scalar::BigInt, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok({
                    let mut v = [0 as u8; 32];
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_uint()
                        .expect(INTERNAL_ERR)
                        .to_big_endian(v.as_mut_slice());
                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                })
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<substreams::scalar::BigInt> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for Oracles {
            const NAME: &'static str = "oracles";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<substreams::scalar::BigInt>
        for Oracles {
            fn output(data: &[u8]) -> Result<substreams::scalar::BigInt, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Owner {}
        impl Owner {
            const METHOD_ID: [u8; 4] = [141u8, 165u8, 203u8, 91u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Ok(Self {})
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Vec<u8>, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<Vec<u8>, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<Vec<u8>> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for Owner {
            const NAME: &'static str = "owner";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<Vec<u8>> for Owner {
            fn output(data: &[u8]) -> Result<Vec<u8>, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct PendingOwner {}
        impl PendingOwner {
            const METHOD_ID: [u8; 4] = [227u8, 12u8, 57u8, 120u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Ok(Self {})
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Vec<u8>, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<Vec<u8>, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<Vec<u8>> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for PendingOwner {
            const NAME: &'static str = "pendingOwner";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<Vec<u8>> for PendingOwner {
            fn output(data: &[u8]) -> Result<Vec<u8>, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct ProtocolFee {}
        impl ProtocolFee {
            const METHOD_ID: [u8; 4] = [176u8, 226u8, 30u8, 138u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Ok(Self {})
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<(Vec<u8>, substreams::scalar::BigInt), String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(
                data: &[u8],
            ) -> Result<(Vec<u8>, substreams::scalar::BigInt), String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address, ethabi::ParamType::Uint(16usize)],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                values.reverse();
                Ok((
                    values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                ))
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(
                &self,
                address: Vec<u8>,
            ) -> Option<(Vec<u8>, substreams::scalar::BigInt)> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for ProtocolFee {
            const NAME: &'static str = "protocolFee";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<
            (Vec<u8>, substreams::scalar::BigInt),
        > for ProtocolFee {
            fn output(
                data: &[u8],
            ) -> Result<(Vec<u8>, substreams::scalar::BigInt), String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct ProxiableUuid {}
        impl ProxiableUuid {
            const METHOD_ID: [u8; 4] = [82u8, 209u8, 144u8, 45u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Ok(Self {})
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<[u8; 32usize], String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<[u8; 32usize], String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::FixedBytes(32usize)],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok({
                    let mut result = [0u8; 32];
                    let v = values
                        .pop()
                        .expect("one output data should have existed")
                        .into_fixed_bytes()
                        .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                })
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<[u8; 32usize]> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for ProxiableUuid {
            const NAME: &'static str = "proxiableUUID";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<[u8; 32usize]> for ProxiableUuid {
            fn output(data: &[u8]) -> Result<[u8; 32usize], String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct RenounceOwnership {}
        impl RenounceOwnership {
            const METHOD_ID: [u8; 4] = [113u8, 80u8, 24u8, 166u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Ok(Self {})
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for RenounceOwnership {
            const NAME: &'static str = "renounceOwnership";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct SetBlockRange {
            pub u_block_range: substreams::scalar::BigInt,
        }
        impl SetBlockRange {
            const METHOD_ID: [u8; 4] = [105u8, 146u8, 170u8, 54u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    u_block_range: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Uint(
                            ethabi::Uint::from_big_endian(
                                match self.u_block_range.clone().to_bytes_be() {
                                    (num_bigint::Sign::Plus, bytes) => bytes,
                                    (num_bigint::Sign::NoSign, bytes) => bytes,
                                    (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported")
                                    }
                                }
                                    .as_slice(),
                            ),
                        ),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for SetBlockRange {
            const NAME: &'static str = "setBlockRange";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct SetGovernor {
            pub u_governor: Vec<u8>,
        }
        impl SetGovernor {
            const METHOD_ID: [u8; 4] = [196u8, 44u8, 245u8, 53u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    u_governor: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.u_governor),
                        ),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for SetGovernor {
            const NAME: &'static str = "setGovernor";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct SetOracle {
            pub oracle: Vec<u8>,
            pub approved: bool,
        }
        impl SetOracle {
            const METHOD_ID: [u8; 4] = [101u8, 54u8, 8u8, 67u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address, ethabi::ParamType::Bool],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    oracle: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    approved: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bool()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.oracle),
                        ),
                        ethabi::Token::Bool(self.approved.clone()),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for SetOracle {
            const NAME: &'static str = "setOracle";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct SetProtocolFee {
            pub recipient: Vec<u8>,
            pub rate: substreams::scalar::BigInt,
        }
        impl SetProtocolFee {
            const METHOD_ID: [u8; 4] = [58u8, 22u8, 183u8, 104u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address, ethabi::ParamType::Uint(16usize)],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    recipient: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    rate: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.recipient),
                        ),
                        ethabi::Token::Uint(
                            ethabi::Uint::from_big_endian(
                                match self.rate.clone().to_bytes_be() {
                                    (num_bigint::Sign::Plus, bytes) => bytes,
                                    (num_bigint::Sign::NoSign, bytes) => bytes,
                                    (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported")
                                    }
                                }
                                    .as_slice(),
                            ),
                        ),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for SetProtocolFee {
            const NAME: &'static str = "setProtocolFee";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct TakeAsk {
            pub inputs: (
                Vec<
                    (
                        Vec<u8>,
                        Vec<u8>,
                        [u8; 32usize],
                        substreams::scalar::BigInt,
                        substreams::scalar::BigInt,
                        substreams::scalar::BigInt,
                        (Vec<u8>, substreams::scalar::BigInt),
                        substreams::scalar::BigInt,
                    ),
                >,
                Vec<
                    (
                        substreams::scalar::BigInt,
                        Vec<[u8; 32usize]>,
                        (
                            substreams::scalar::BigInt,
                            substreams::scalar::BigInt,
                            substreams::scalar::BigInt,
                            substreams::scalar::BigInt,
                        ),
                        (substreams::scalar::BigInt, substreams::scalar::BigInt),
                    ),
                >,
                (Vec<u8>, substreams::scalar::BigInt),
                Vec<u8>,
                Vec<u8>,
            ),
            pub oracle_signature: Vec<u8>,
        }
        impl TakeAsk {
            const METHOD_ID: [u8; 4] = [57u8, 37u8, 195u8, 195u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Tuple(
                                vec![
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::Address,
                                    ethabi::ParamType::Address,
                                    ethabi::ParamType::FixedBytes(32usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(8usize),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Address,
                                    ethabi::ParamType::Uint(16usize)]),
                                    ethabi::ParamType::Uint(256usize)]))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::FixedBytes(32usize))),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize)]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize)])]))),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Address,
                                    ethabi::ParamType::Uint(16usize)]),
                                    ethabi::ParamType::Bytes, ethabi::ParamType::Address
                                ],
                            ),
                            ethabi::ParamType::Bytes,
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    inputs: {
                        let tuple_elements = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_tuple()
                            .expect(INTERNAL_ERR);
                        (
                            tuple_elements[0usize]
                                .clone()
                                .into_array()
                                .expect(INTERNAL_ERR)
                                .into_iter()
                                .map(|inner| {
                                    let tuple_elements = inner
                                        .into_tuple()
                                        .expect(INTERNAL_ERR);
                                    (
                                        tuple_elements[0usize]
                                            .clone()
                                            .into_address()
                                            .expect(INTERNAL_ERR)
                                            .as_bytes()
                                            .to_vec(),
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_address()
                                            .expect(INTERNAL_ERR)
                                            .as_bytes()
                                            .to_vec(),
                                        {
                                            let mut result = [0u8; 32];
                                            let v = tuple_elements[2usize]
                                                .clone()
                                                .into_fixed_bytes()
                                                .expect(INTERNAL_ERR);
                                            result.copy_from_slice(&v);
                                            result
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[3usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[4usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[5usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        {
                                            let tuple_elements = tuple_elements[6usize]
                                                .clone()
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_address()
                                                    .expect(INTERNAL_ERR)
                                                    .as_bytes()
                                                    .to_vec(),
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[1usize]
                                                        .clone()
                                                        .into_uint()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                },
                                            )
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[7usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                    )
                                })
                                .collect(),
                            tuple_elements[1usize]
                                .clone()
                                .into_array()
                                .expect(INTERNAL_ERR)
                                .into_iter()
                                .map(|inner| {
                                    let tuple_elements = inner
                                        .into_tuple()
                                        .expect(INTERNAL_ERR);
                                    (
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[0usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_array()
                                            .expect(INTERNAL_ERR)
                                            .into_iter()
                                            .map(|inner| {
                                                let mut result = [0u8; 32];
                                                let v = inner.into_fixed_bytes().expect(INTERNAL_ERR);
                                                result.copy_from_slice(&v);
                                                result
                                            })
                                            .collect(),
                                        {
                                            let tuple_elements = tuple_elements[2usize]
                                                .clone()
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[0usize]
                                                        .clone()
                                                        .into_uint()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                },
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[1usize]
                                                        .clone()
                                                        .into_uint()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                },
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[2usize]
                                                        .clone()
                                                        .into_uint()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                },
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[3usize]
                                                        .clone()
                                                        .into_uint()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                },
                                            )
                                        },
                                        {
                                            let tuple_elements = tuple_elements[3usize]
                                                .clone()
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[0usize]
                                                        .clone()
                                                        .into_uint()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                },
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[1usize]
                                                        .clone()
                                                        .into_uint()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                },
                                            )
                                        },
                                    )
                                })
                                .collect(),
                            {
                                let tuple_elements = tuple_elements[2usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec(),
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                )
                            },
                            tuple_elements[3usize]
                                .clone()
                                .into_bytes()
                                .expect(INTERNAL_ERR),
                            tuple_elements[4usize]
                                .clone()
                                .into_address()
                                .expect(INTERNAL_ERR)
                                .as_bytes()
                                .to_vec(),
                        )
                    },
                    oracle_signature: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bytes()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Tuple(
                            vec![
                                { let v = self.inputs.0.iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::Address(ethabi::Address::from_slice(&
                                inner.0)),
                                ethabi::Token::Address(ethabi::Address::from_slice(& inner
                                .1)), ethabi::Token::FixedBytes(inner.2.as_ref().to_vec()),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.3.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.4.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.5.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Tuple(vec![ethabi::Token::Address(ethabi::Address::from_slice(&
                                inner.6.0)),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.6.1.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)]),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.7.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)])).collect(); ethabi::Token::Array(v) }, {
                                let v = self.inputs.1.iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.0.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),), { let v = inner.1.iter().map(| inner |
                                ethabi::Token::FixedBytes(inner.as_ref().to_vec()))
                                .collect(); ethabi::Token::Array(v) },
                                ethabi::Token::Tuple(vec![ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.2.0.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.2.1.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.2.2.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.2.3.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)]),
                                ethabi::Token::Tuple(vec![ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.3.0.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.3.1.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)])])).collect(); ethabi::Token::Array(v) },
                                ethabi::Token::Tuple(vec![ethabi::Token::Address(ethabi::Address::from_slice(&
                                self.inputs.2.0)),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.2.1.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)]), ethabi::Token::Bytes(self.inputs.3
                                .clone()),
                                ethabi::Token::Address(ethabi::Address::from_slice(& self
                                .inputs.4))
                            ],
                        ),
                        ethabi::Token::Bytes(self.oracle_signature.clone()),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for TakeAsk {
            const NAME: &'static str = "takeAsk";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct TakeAskPool {
            pub inputs: (
                Vec<
                    (
                        Vec<u8>,
                        Vec<u8>,
                        [u8; 32usize],
                        substreams::scalar::BigInt,
                        substreams::scalar::BigInt,
                        substreams::scalar::BigInt,
                        (Vec<u8>, substreams::scalar::BigInt),
                        substreams::scalar::BigInt,
                    ),
                >,
                Vec<
                    (
                        substreams::scalar::BigInt,
                        Vec<[u8; 32usize]>,
                        (
                            substreams::scalar::BigInt,
                            substreams::scalar::BigInt,
                            substreams::scalar::BigInt,
                            substreams::scalar::BigInt,
                        ),
                        (substreams::scalar::BigInt, substreams::scalar::BigInt),
                    ),
                >,
                (Vec<u8>, substreams::scalar::BigInt),
                Vec<u8>,
                Vec<u8>,
            ),
            pub oracle_signature: Vec<u8>,
            pub amount_to_withdraw: substreams::scalar::BigInt,
        }
        impl TakeAskPool {
            const METHOD_ID: [u8; 4] = [19u8, 59u8, 169u8, 166u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Tuple(
                                vec![
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::Address,
                                    ethabi::ParamType::Address,
                                    ethabi::ParamType::FixedBytes(32usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(8usize),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Address,
                                    ethabi::ParamType::Uint(16usize)]),
                                    ethabi::ParamType::Uint(256usize)]))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::FixedBytes(32usize))),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize)]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize)])]))),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Address,
                                    ethabi::ParamType::Uint(16usize)]),
                                    ethabi::ParamType::Bytes, ethabi::ParamType::Address
                                ],
                            ),
                            ethabi::ParamType::Bytes,
                            ethabi::ParamType::Uint(256usize),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    inputs: {
                        let tuple_elements = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_tuple()
                            .expect(INTERNAL_ERR);
                        (
                            tuple_elements[0usize]
                                .clone()
                                .into_array()
                                .expect(INTERNAL_ERR)
                                .into_iter()
                                .map(|inner| {
                                    let tuple_elements = inner
                                        .into_tuple()
                                        .expect(INTERNAL_ERR);
                                    (
                                        tuple_elements[0usize]
                                            .clone()
                                            .into_address()
                                            .expect(INTERNAL_ERR)
                                            .as_bytes()
                                            .to_vec(),
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_address()
                                            .expect(INTERNAL_ERR)
                                            .as_bytes()
                                            .to_vec(),
                                        {
                                            let mut result = [0u8; 32];
                                            let v = tuple_elements[2usize]
                                                .clone()
                                                .into_fixed_bytes()
                                                .expect(INTERNAL_ERR);
                                            result.copy_from_slice(&v);
                                            result
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[3usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[4usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[5usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        {
                                            let tuple_elements = tuple_elements[6usize]
                                                .clone()
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_address()
                                                    .expect(INTERNAL_ERR)
                                                    .as_bytes()
                                                    .to_vec(),
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[1usize]
                                                        .clone()
                                                        .into_uint()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                },
                                            )
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[7usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                    )
                                })
                                .collect(),
                            tuple_elements[1usize]
                                .clone()
                                .into_array()
                                .expect(INTERNAL_ERR)
                                .into_iter()
                                .map(|inner| {
                                    let tuple_elements = inner
                                        .into_tuple()
                                        .expect(INTERNAL_ERR);
                                    (
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[0usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_array()
                                            .expect(INTERNAL_ERR)
                                            .into_iter()
                                            .map(|inner| {
                                                let mut result = [0u8; 32];
                                                let v = inner.into_fixed_bytes().expect(INTERNAL_ERR);
                                                result.copy_from_slice(&v);
                                                result
                                            })
                                            .collect(),
                                        {
                                            let tuple_elements = tuple_elements[2usize]
                                                .clone()
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[0usize]
                                                        .clone()
                                                        .into_uint()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                },
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[1usize]
                                                        .clone()
                                                        .into_uint()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                },
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[2usize]
                                                        .clone()
                                                        .into_uint()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                },
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[3usize]
                                                        .clone()
                                                        .into_uint()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                },
                                            )
                                        },
                                        {
                                            let tuple_elements = tuple_elements[3usize]
                                                .clone()
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[0usize]
                                                        .clone()
                                                        .into_uint()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                },
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[1usize]
                                                        .clone()
                                                        .into_uint()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                },
                                            )
                                        },
                                    )
                                })
                                .collect(),
                            {
                                let tuple_elements = tuple_elements[2usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec(),
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                )
                            },
                            tuple_elements[3usize]
                                .clone()
                                .into_bytes()
                                .expect(INTERNAL_ERR),
                            tuple_elements[4usize]
                                .clone()
                                .into_address()
                                .expect(INTERNAL_ERR)
                                .as_bytes()
                                .to_vec(),
                        )
                    },
                    oracle_signature: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bytes()
                        .expect(INTERNAL_ERR),
                    amount_to_withdraw: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Tuple(
                            vec![
                                { let v = self.inputs.0.iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::Address(ethabi::Address::from_slice(&
                                inner.0)),
                                ethabi::Token::Address(ethabi::Address::from_slice(& inner
                                .1)), ethabi::Token::FixedBytes(inner.2.as_ref().to_vec()),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.3.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.4.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.5.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Tuple(vec![ethabi::Token::Address(ethabi::Address::from_slice(&
                                inner.6.0)),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.6.1.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)]),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.7.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)])).collect(); ethabi::Token::Array(v) }, {
                                let v = self.inputs.1.iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.0.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),), { let v = inner.1.iter().map(| inner |
                                ethabi::Token::FixedBytes(inner.as_ref().to_vec()))
                                .collect(); ethabi::Token::Array(v) },
                                ethabi::Token::Tuple(vec![ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.2.0.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.2.1.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.2.2.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.2.3.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)]),
                                ethabi::Token::Tuple(vec![ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.3.0.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.3.1.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)])])).collect(); ethabi::Token::Array(v) },
                                ethabi::Token::Tuple(vec![ethabi::Token::Address(ethabi::Address::from_slice(&
                                self.inputs.2.0)),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.2.1.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)]), ethabi::Token::Bytes(self.inputs.3
                                .clone()),
                                ethabi::Token::Address(ethabi::Address::from_slice(& self
                                .inputs.4))
                            ],
                        ),
                        ethabi::Token::Bytes(self.oracle_signature.clone()),
                        ethabi::Token::Uint(
                            ethabi::Uint::from_big_endian(
                                match self.amount_to_withdraw.clone().to_bytes_be() {
                                    (num_bigint::Sign::Plus, bytes) => bytes,
                                    (num_bigint::Sign::NoSign, bytes) => bytes,
                                    (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported")
                                    }
                                }
                                    .as_slice(),
                            ),
                        ),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for TakeAskPool {
            const NAME: &'static str = "takeAskPool";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct TakeAskSingle {
            pub inputs: (
                (
                    Vec<u8>,
                    Vec<u8>,
                    [u8; 32usize],
                    substreams::scalar::BigInt,
                    substreams::scalar::BigInt,
                    substreams::scalar::BigInt,
                    (Vec<u8>, substreams::scalar::BigInt),
                    substreams::scalar::BigInt,
                ),
                (
                    substreams::scalar::BigInt,
                    Vec<[u8; 32usize]>,
                    (
                        substreams::scalar::BigInt,
                        substreams::scalar::BigInt,
                        substreams::scalar::BigInt,
                        substreams::scalar::BigInt,
                    ),
                    (substreams::scalar::BigInt, substreams::scalar::BigInt),
                ),
                (Vec<u8>, substreams::scalar::BigInt),
                Vec<u8>,
                Vec<u8>,
            ),
            pub oracle_signature: Vec<u8>,
        }
        impl TakeAskSingle {
            const METHOD_ID: [u8; 4] = [112u8, 188u8, 226u8, 214u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Tuple(
                                vec![
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Address,
                                    ethabi::ParamType::Address,
                                    ethabi::ParamType::FixedBytes(32usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(8usize),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Address,
                                    ethabi::ParamType::Uint(16usize)]),
                                    ethabi::ParamType::Uint(256usize)]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::FixedBytes(32usize))),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize)]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize)])]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Address,
                                    ethabi::ParamType::Uint(16usize)]),
                                    ethabi::ParamType::Bytes, ethabi::ParamType::Address
                                ],
                            ),
                            ethabi::ParamType::Bytes,
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    inputs: {
                        let tuple_elements = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_tuple()
                            .expect(INTERNAL_ERR);
                        (
                            {
                                let tuple_elements = tuple_elements[0usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec(),
                                    {
                                        let mut result = [0u8; 32];
                                        let v = tuple_elements[2usize]
                                            .clone()
                                            .into_fixed_bytes()
                                            .expect(INTERNAL_ERR);
                                        result.copy_from_slice(&v);
                                        result
                                    },
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[3usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[4usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[5usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                    {
                                        let tuple_elements = tuple_elements[6usize]
                                            .clone()
                                            .into_tuple()
                                            .expect(INTERNAL_ERR);
                                        (
                                            tuple_elements[0usize]
                                                .clone()
                                                .into_address()
                                                .expect(INTERNAL_ERR)
                                                .as_bytes()
                                                .to_vec(),
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                            },
                                        )
                                    },
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[7usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                )
                            },
                            {
                                let tuple_elements = tuple_elements[1usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[0usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let mut result = [0u8; 32];
                                            let v = inner.into_fixed_bytes().expect(INTERNAL_ERR);
                                            result.copy_from_slice(&v);
                                            result
                                        })
                                        .collect(),
                                    {
                                        let tuple_elements = tuple_elements[2usize]
                                            .clone()
                                            .into_tuple()
                                            .expect(INTERNAL_ERR);
                                        (
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                            },
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                            },
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[2usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                            },
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[3usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                            },
                                        )
                                    },
                                    {
                                        let tuple_elements = tuple_elements[3usize]
                                            .clone()
                                            .into_tuple()
                                            .expect(INTERNAL_ERR);
                                        (
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                            },
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                            },
                                        )
                                    },
                                )
                            },
                            {
                                let tuple_elements = tuple_elements[2usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec(),
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                )
                            },
                            tuple_elements[3usize]
                                .clone()
                                .into_bytes()
                                .expect(INTERNAL_ERR),
                            tuple_elements[4usize]
                                .clone()
                                .into_address()
                                .expect(INTERNAL_ERR)
                                .as_bytes()
                                .to_vec(),
                        )
                    },
                    oracle_signature: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bytes()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Tuple(
                            vec![
                                ethabi::Token::Tuple(vec![ethabi::Token::Address(ethabi::Address::from_slice(&
                                self.inputs.0.0)),
                                ethabi::Token::Address(ethabi::Address::from_slice(& self
                                .inputs.0.1)), ethabi::Token::FixedBytes(self.inputs.0.2
                                .as_ref().to_vec()),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.0.3.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.0.4.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.0.5.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Tuple(vec![ethabi::Token::Address(ethabi::Address::from_slice(&
                                self.inputs.0.6.0)),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.0.6.1.clone().to_bytes_be() {
                                (num_bigint::Sign::Plus, bytes) => bytes,
                                (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)]),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.0.7.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)]),
                                ethabi::Token::Tuple(vec![ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                self.inputs.1.0.clone().to_bytes_be() {
                                (num_bigint::Sign::Plus, bytes) => bytes,
                                (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),), { let v = self.inputs.1.1.iter().map(|
                                inner | ethabi::Token::FixedBytes(inner.as_ref().to_vec()))
                                .collect(); ethabi::Token::Array(v) },
                                ethabi::Token::Tuple(vec![ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                self.inputs.1.2.0.clone().to_bytes_be() {
                                (num_bigint::Sign::Plus, bytes) => bytes,
                                (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.1.2.1.clone().to_bytes_be() {
                                (num_bigint::Sign::Plus, bytes) => bytes,
                                (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.1.2.2.clone().to_bytes_be() {
                                (num_bigint::Sign::Plus, bytes) => bytes,
                                (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.1.2.3.clone().to_bytes_be() {
                                (num_bigint::Sign::Plus, bytes) => bytes,
                                (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)]),
                                ethabi::Token::Tuple(vec![ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                self.inputs.1.3.0.clone().to_bytes_be() {
                                (num_bigint::Sign::Plus, bytes) => bytes,
                                (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.1.3.1.clone().to_bytes_be() {
                                (num_bigint::Sign::Plus, bytes) => bytes,
                                (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)])]),
                                ethabi::Token::Tuple(vec![ethabi::Token::Address(ethabi::Address::from_slice(&
                                self.inputs.2.0)),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.2.1.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)]), ethabi::Token::Bytes(self.inputs.3
                                .clone()),
                                ethabi::Token::Address(ethabi::Address::from_slice(& self
                                .inputs.4))
                            ],
                        ),
                        ethabi::Token::Bytes(self.oracle_signature.clone()),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for TakeAskSingle {
            const NAME: &'static str = "takeAskSingle";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct TakeAskSinglePool {
            pub inputs: (
                (
                    Vec<u8>,
                    Vec<u8>,
                    [u8; 32usize],
                    substreams::scalar::BigInt,
                    substreams::scalar::BigInt,
                    substreams::scalar::BigInt,
                    (Vec<u8>, substreams::scalar::BigInt),
                    substreams::scalar::BigInt,
                ),
                (
                    substreams::scalar::BigInt,
                    Vec<[u8; 32usize]>,
                    (
                        substreams::scalar::BigInt,
                        substreams::scalar::BigInt,
                        substreams::scalar::BigInt,
                        substreams::scalar::BigInt,
                    ),
                    (substreams::scalar::BigInt, substreams::scalar::BigInt),
                ),
                (Vec<u8>, substreams::scalar::BigInt),
                Vec<u8>,
                Vec<u8>,
            ),
            pub oracle_signature: Vec<u8>,
            pub amount_to_withdraw: substreams::scalar::BigInt,
        }
        impl TakeAskSinglePool {
            const METHOD_ID: [u8; 4] = [51u8, 109u8, 130u8, 6u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Tuple(
                                vec![
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Address,
                                    ethabi::ParamType::Address,
                                    ethabi::ParamType::FixedBytes(32usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(8usize),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Address,
                                    ethabi::ParamType::Uint(16usize)]),
                                    ethabi::ParamType::Uint(256usize)]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::FixedBytes(32usize))),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize)]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize)])]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Address,
                                    ethabi::ParamType::Uint(16usize)]),
                                    ethabi::ParamType::Bytes, ethabi::ParamType::Address
                                ],
                            ),
                            ethabi::ParamType::Bytes,
                            ethabi::ParamType::Uint(256usize),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    inputs: {
                        let tuple_elements = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_tuple()
                            .expect(INTERNAL_ERR);
                        (
                            {
                                let tuple_elements = tuple_elements[0usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec(),
                                    {
                                        let mut result = [0u8; 32];
                                        let v = tuple_elements[2usize]
                                            .clone()
                                            .into_fixed_bytes()
                                            .expect(INTERNAL_ERR);
                                        result.copy_from_slice(&v);
                                        result
                                    },
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[3usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[4usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[5usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                    {
                                        let tuple_elements = tuple_elements[6usize]
                                            .clone()
                                            .into_tuple()
                                            .expect(INTERNAL_ERR);
                                        (
                                            tuple_elements[0usize]
                                                .clone()
                                                .into_address()
                                                .expect(INTERNAL_ERR)
                                                .as_bytes()
                                                .to_vec(),
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                            },
                                        )
                                    },
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[7usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                )
                            },
                            {
                                let tuple_elements = tuple_elements[1usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[0usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let mut result = [0u8; 32];
                                            let v = inner.into_fixed_bytes().expect(INTERNAL_ERR);
                                            result.copy_from_slice(&v);
                                            result
                                        })
                                        .collect(),
                                    {
                                        let tuple_elements = tuple_elements[2usize]
                                            .clone()
                                            .into_tuple()
                                            .expect(INTERNAL_ERR);
                                        (
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                            },
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                            },
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[2usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                            },
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[3usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                            },
                                        )
                                    },
                                    {
                                        let tuple_elements = tuple_elements[3usize]
                                            .clone()
                                            .into_tuple()
                                            .expect(INTERNAL_ERR);
                                        (
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                            },
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                            },
                                        )
                                    },
                                )
                            },
                            {
                                let tuple_elements = tuple_elements[2usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec(),
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                )
                            },
                            tuple_elements[3usize]
                                .clone()
                                .into_bytes()
                                .expect(INTERNAL_ERR),
                            tuple_elements[4usize]
                                .clone()
                                .into_address()
                                .expect(INTERNAL_ERR)
                                .as_bytes()
                                .to_vec(),
                        )
                    },
                    oracle_signature: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bytes()
                        .expect(INTERNAL_ERR),
                    amount_to_withdraw: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Tuple(
                            vec![
                                ethabi::Token::Tuple(vec![ethabi::Token::Address(ethabi::Address::from_slice(&
                                self.inputs.0.0)),
                                ethabi::Token::Address(ethabi::Address::from_slice(& self
                                .inputs.0.1)), ethabi::Token::FixedBytes(self.inputs.0.2
                                .as_ref().to_vec()),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.0.3.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.0.4.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.0.5.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Tuple(vec![ethabi::Token::Address(ethabi::Address::from_slice(&
                                self.inputs.0.6.0)),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.0.6.1.clone().to_bytes_be() {
                                (num_bigint::Sign::Plus, bytes) => bytes,
                                (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)]),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.0.7.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)]),
                                ethabi::Token::Tuple(vec![ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                self.inputs.1.0.clone().to_bytes_be() {
                                (num_bigint::Sign::Plus, bytes) => bytes,
                                (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),), { let v = self.inputs.1.1.iter().map(|
                                inner | ethabi::Token::FixedBytes(inner.as_ref().to_vec()))
                                .collect(); ethabi::Token::Array(v) },
                                ethabi::Token::Tuple(vec![ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                self.inputs.1.2.0.clone().to_bytes_be() {
                                (num_bigint::Sign::Plus, bytes) => bytes,
                                (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.1.2.1.clone().to_bytes_be() {
                                (num_bigint::Sign::Plus, bytes) => bytes,
                                (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.1.2.2.clone().to_bytes_be() {
                                (num_bigint::Sign::Plus, bytes) => bytes,
                                (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.1.2.3.clone().to_bytes_be() {
                                (num_bigint::Sign::Plus, bytes) => bytes,
                                (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)]),
                                ethabi::Token::Tuple(vec![ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                self.inputs.1.3.0.clone().to_bytes_be() {
                                (num_bigint::Sign::Plus, bytes) => bytes,
                                (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.1.3.1.clone().to_bytes_be() {
                                (num_bigint::Sign::Plus, bytes) => bytes,
                                (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)])]),
                                ethabi::Token::Tuple(vec![ethabi::Token::Address(ethabi::Address::from_slice(&
                                self.inputs.2.0)),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.2.1.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)]), ethabi::Token::Bytes(self.inputs.3
                                .clone()),
                                ethabi::Token::Address(ethabi::Address::from_slice(& self
                                .inputs.4))
                            ],
                        ),
                        ethabi::Token::Bytes(self.oracle_signature.clone()),
                        ethabi::Token::Uint(
                            ethabi::Uint::from_big_endian(
                                match self.amount_to_withdraw.clone().to_bytes_be() {
                                    (num_bigint::Sign::Plus, bytes) => bytes,
                                    (num_bigint::Sign::NoSign, bytes) => bytes,
                                    (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported")
                                    }
                                }
                                    .as_slice(),
                            ),
                        ),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for TakeAskSinglePool {
            const NAME: &'static str = "takeAskSinglePool";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct TakeBid {
            pub inputs: (
                Vec<
                    (
                        Vec<u8>,
                        Vec<u8>,
                        [u8; 32usize],
                        substreams::scalar::BigInt,
                        substreams::scalar::BigInt,
                        substreams::scalar::BigInt,
                        (Vec<u8>, substreams::scalar::BigInt),
                        substreams::scalar::BigInt,
                    ),
                >,
                Vec<
                    (
                        substreams::scalar::BigInt,
                        Vec<[u8; 32usize]>,
                        (
                            substreams::scalar::BigInt,
                            substreams::scalar::BigInt,
                            substreams::scalar::BigInt,
                            substreams::scalar::BigInt,
                        ),
                        (substreams::scalar::BigInt, substreams::scalar::BigInt),
                    ),
                >,
                (Vec<u8>, substreams::scalar::BigInt),
                Vec<u8>,
            ),
            pub oracle_signature: Vec<u8>,
        }
        impl TakeBid {
            const METHOD_ID: [u8; 4] = [112u8, 52u8, 209u8, 32u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Tuple(
                                vec![
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::Address,
                                    ethabi::ParamType::Address,
                                    ethabi::ParamType::FixedBytes(32usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(8usize),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Address,
                                    ethabi::ParamType::Uint(16usize)]),
                                    ethabi::ParamType::Uint(256usize)]))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::FixedBytes(32usize))),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize)]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize)])]))),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Address,
                                    ethabi::ParamType::Uint(16usize)]), ethabi::ParamType::Bytes
                                ],
                            ),
                            ethabi::ParamType::Bytes,
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    inputs: {
                        let tuple_elements = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_tuple()
                            .expect(INTERNAL_ERR);
                        (
                            tuple_elements[0usize]
                                .clone()
                                .into_array()
                                .expect(INTERNAL_ERR)
                                .into_iter()
                                .map(|inner| {
                                    let tuple_elements = inner
                                        .into_tuple()
                                        .expect(INTERNAL_ERR);
                                    (
                                        tuple_elements[0usize]
                                            .clone()
                                            .into_address()
                                            .expect(INTERNAL_ERR)
                                            .as_bytes()
                                            .to_vec(),
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_address()
                                            .expect(INTERNAL_ERR)
                                            .as_bytes()
                                            .to_vec(),
                                        {
                                            let mut result = [0u8; 32];
                                            let v = tuple_elements[2usize]
                                                .clone()
                                                .into_fixed_bytes()
                                                .expect(INTERNAL_ERR);
                                            result.copy_from_slice(&v);
                                            result
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[3usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[4usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[5usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        {
                                            let tuple_elements = tuple_elements[6usize]
                                                .clone()
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_address()
                                                    .expect(INTERNAL_ERR)
                                                    .as_bytes()
                                                    .to_vec(),
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[1usize]
                                                        .clone()
                                                        .into_uint()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                },
                                            )
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[7usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                    )
                                })
                                .collect(),
                            tuple_elements[1usize]
                                .clone()
                                .into_array()
                                .expect(INTERNAL_ERR)
                                .into_iter()
                                .map(|inner| {
                                    let tuple_elements = inner
                                        .into_tuple()
                                        .expect(INTERNAL_ERR);
                                    (
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[0usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_array()
                                            .expect(INTERNAL_ERR)
                                            .into_iter()
                                            .map(|inner| {
                                                let mut result = [0u8; 32];
                                                let v = inner.into_fixed_bytes().expect(INTERNAL_ERR);
                                                result.copy_from_slice(&v);
                                                result
                                            })
                                            .collect(),
                                        {
                                            let tuple_elements = tuple_elements[2usize]
                                                .clone()
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[0usize]
                                                        .clone()
                                                        .into_uint()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                },
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[1usize]
                                                        .clone()
                                                        .into_uint()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                },
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[2usize]
                                                        .clone()
                                                        .into_uint()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                },
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[3usize]
                                                        .clone()
                                                        .into_uint()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                },
                                            )
                                        },
                                        {
                                            let tuple_elements = tuple_elements[3usize]
                                                .clone()
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[0usize]
                                                        .clone()
                                                        .into_uint()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                },
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[1usize]
                                                        .clone()
                                                        .into_uint()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                },
                                            )
                                        },
                                    )
                                })
                                .collect(),
                            {
                                let tuple_elements = tuple_elements[2usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec(),
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                )
                            },
                            tuple_elements[3usize]
                                .clone()
                                .into_bytes()
                                .expect(INTERNAL_ERR),
                        )
                    },
                    oracle_signature: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bytes()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Tuple(
                            vec![
                                { let v = self.inputs.0.iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::Address(ethabi::Address::from_slice(&
                                inner.0)),
                                ethabi::Token::Address(ethabi::Address::from_slice(& inner
                                .1)), ethabi::Token::FixedBytes(inner.2.as_ref().to_vec()),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.3.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.4.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.5.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Tuple(vec![ethabi::Token::Address(ethabi::Address::from_slice(&
                                inner.6.0)),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.6.1.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)]),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.7.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)])).collect(); ethabi::Token::Array(v) }, {
                                let v = self.inputs.1.iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.0.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),), { let v = inner.1.iter().map(| inner |
                                ethabi::Token::FixedBytes(inner.as_ref().to_vec()))
                                .collect(); ethabi::Token::Array(v) },
                                ethabi::Token::Tuple(vec![ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.2.0.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.2.1.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.2.2.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.2.3.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)]),
                                ethabi::Token::Tuple(vec![ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.3.0.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.3.1.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)])])).collect(); ethabi::Token::Array(v) },
                                ethabi::Token::Tuple(vec![ethabi::Token::Address(ethabi::Address::from_slice(&
                                self.inputs.2.0)),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.2.1.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)]), ethabi::Token::Bytes(self.inputs.3
                                .clone())
                            ],
                        ),
                        ethabi::Token::Bytes(self.oracle_signature.clone()),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for TakeBid {
            const NAME: &'static str = "takeBid";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct TakeBidSingle {
            pub inputs: (
                (
                    Vec<u8>,
                    Vec<u8>,
                    [u8; 32usize],
                    substreams::scalar::BigInt,
                    substreams::scalar::BigInt,
                    substreams::scalar::BigInt,
                    (Vec<u8>, substreams::scalar::BigInt),
                    substreams::scalar::BigInt,
                ),
                (
                    substreams::scalar::BigInt,
                    Vec<[u8; 32usize]>,
                    (
                        substreams::scalar::BigInt,
                        substreams::scalar::BigInt,
                        substreams::scalar::BigInt,
                        substreams::scalar::BigInt,
                    ),
                    (substreams::scalar::BigInt, substreams::scalar::BigInt),
                ),
                (Vec<u8>, substreams::scalar::BigInt),
                Vec<u8>,
            ),
            pub oracle_signature: Vec<u8>,
        }
        impl TakeBidSingle {
            const METHOD_ID: [u8; 4] = [218u8, 129u8, 92u8, 181u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Tuple(
                                vec![
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Address,
                                    ethabi::ParamType::Address,
                                    ethabi::ParamType::FixedBytes(32usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(8usize),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Address,
                                    ethabi::ParamType::Uint(16usize)]),
                                    ethabi::ParamType::Uint(256usize)]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::FixedBytes(32usize))),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize)]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize)])]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Address,
                                    ethabi::ParamType::Uint(16usize)]), ethabi::ParamType::Bytes
                                ],
                            ),
                            ethabi::ParamType::Bytes,
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    inputs: {
                        let tuple_elements = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_tuple()
                            .expect(INTERNAL_ERR);
                        (
                            {
                                let tuple_elements = tuple_elements[0usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec(),
                                    {
                                        let mut result = [0u8; 32];
                                        let v = tuple_elements[2usize]
                                            .clone()
                                            .into_fixed_bytes()
                                            .expect(INTERNAL_ERR);
                                        result.copy_from_slice(&v);
                                        result
                                    },
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[3usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[4usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[5usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                    {
                                        let tuple_elements = tuple_elements[6usize]
                                            .clone()
                                            .into_tuple()
                                            .expect(INTERNAL_ERR);
                                        (
                                            tuple_elements[0usize]
                                                .clone()
                                                .into_address()
                                                .expect(INTERNAL_ERR)
                                                .as_bytes()
                                                .to_vec(),
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                            },
                                        )
                                    },
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[7usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                )
                            },
                            {
                                let tuple_elements = tuple_elements[1usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[0usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let mut result = [0u8; 32];
                                            let v = inner.into_fixed_bytes().expect(INTERNAL_ERR);
                                            result.copy_from_slice(&v);
                                            result
                                        })
                                        .collect(),
                                    {
                                        let tuple_elements = tuple_elements[2usize]
                                            .clone()
                                            .into_tuple()
                                            .expect(INTERNAL_ERR);
                                        (
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                            },
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                            },
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[2usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                            },
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[3usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                            },
                                        )
                                    },
                                    {
                                        let tuple_elements = tuple_elements[3usize]
                                            .clone()
                                            .into_tuple()
                                            .expect(INTERNAL_ERR);
                                        (
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                            },
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                            },
                                        )
                                    },
                                )
                            },
                            {
                                let tuple_elements = tuple_elements[2usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec(),
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                )
                            },
                            tuple_elements[3usize]
                                .clone()
                                .into_bytes()
                                .expect(INTERNAL_ERR),
                        )
                    },
                    oracle_signature: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bytes()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Tuple(
                            vec![
                                ethabi::Token::Tuple(vec![ethabi::Token::Address(ethabi::Address::from_slice(&
                                self.inputs.0.0)),
                                ethabi::Token::Address(ethabi::Address::from_slice(& self
                                .inputs.0.1)), ethabi::Token::FixedBytes(self.inputs.0.2
                                .as_ref().to_vec()),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.0.3.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.0.4.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.0.5.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Tuple(vec![ethabi::Token::Address(ethabi::Address::from_slice(&
                                self.inputs.0.6.0)),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.0.6.1.clone().to_bytes_be() {
                                (num_bigint::Sign::Plus, bytes) => bytes,
                                (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)]),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.0.7.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)]),
                                ethabi::Token::Tuple(vec![ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                self.inputs.1.0.clone().to_bytes_be() {
                                (num_bigint::Sign::Plus, bytes) => bytes,
                                (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),), { let v = self.inputs.1.1.iter().map(|
                                inner | ethabi::Token::FixedBytes(inner.as_ref().to_vec()))
                                .collect(); ethabi::Token::Array(v) },
                                ethabi::Token::Tuple(vec![ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                self.inputs.1.2.0.clone().to_bytes_be() {
                                (num_bigint::Sign::Plus, bytes) => bytes,
                                (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.1.2.1.clone().to_bytes_be() {
                                (num_bigint::Sign::Plus, bytes) => bytes,
                                (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.1.2.2.clone().to_bytes_be() {
                                (num_bigint::Sign::Plus, bytes) => bytes,
                                (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.1.2.3.clone().to_bytes_be() {
                                (num_bigint::Sign::Plus, bytes) => bytes,
                                (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)]),
                                ethabi::Token::Tuple(vec![ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                self.inputs.1.3.0.clone().to_bytes_be() {
                                (num_bigint::Sign::Plus, bytes) => bytes,
                                (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.1.3.1.clone().to_bytes_be() {
                                (num_bigint::Sign::Plus, bytes) => bytes,
                                (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)])]),
                                ethabi::Token::Tuple(vec![ethabi::Token::Address(ethabi::Address::from_slice(&
                                self.inputs.2.0)),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .inputs.2.1.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)]), ethabi::Token::Bytes(self.inputs.3
                                .clone())
                            ],
                        ),
                        ethabi::Token::Bytes(self.oracle_signature.clone()),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for TakeBidSingle {
            const NAME: &'static str = "takeBidSingle";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct TransferOwnership {
            pub new_owner: Vec<u8>,
        }
        impl TransferOwnership {
            const METHOD_ID: [u8; 4] = [242u8, 253u8, 227u8, 139u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    new_owner: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.new_owner),
                        ),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for TransferOwnership {
            const NAME: &'static str = "transferOwnership";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct UpgradeTo {
            pub new_implementation: Vec<u8>,
        }
        impl UpgradeTo {
            const METHOD_ID: [u8; 4] = [54u8, 89u8, 207u8, 230u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    new_implementation: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.new_implementation),
                        ),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for UpgradeTo {
            const NAME: &'static str = "upgradeTo";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct UpgradeToAndCall {
            pub new_implementation: Vec<u8>,
            pub data: Vec<u8>,
        }
        impl UpgradeToAndCall {
            const METHOD_ID: [u8; 4] = [79u8, 30u8, 242u8, 134u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address, ethabi::ParamType::Bytes],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    new_implementation: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    data: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bytes()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.new_implementation),
                        ),
                        ethabi::Token::Bytes(self.data.clone()),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for UpgradeToAndCall {
            const NAME: &'static str = "upgradeToAndCall";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct VerifyDomain {}
        impl VerifyDomain {
            const METHOD_ID: [u8; 4] = [112u8, 142u8, 249u8, 170u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Ok(Self {})
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for VerifyDomain {
            const NAME: &'static str = "verifyDomain";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
    }
    /// Contract's events.
    #[allow(dead_code, unused_imports, unused_variables)]
    pub mod events {
        use super::INTERNAL_ERR;
        #[derive(Debug, Clone, PartialEq)]
        pub struct AdminChanged1 {
            pub previous_admin: Vec<u8>,
            pub new_admin: Vec<u8>,
        }
        impl AdminChanged1 {
            const TOPIC_ID: [u8; 32] = [
                126u8,
                100u8,
                77u8,
                121u8,
                66u8,
                47u8,
                23u8,
                192u8,
                30u8,
                72u8,
                148u8,
                181u8,
                244u8,
                245u8,
                136u8,
                211u8,
                49u8,
                235u8,
                250u8,
                40u8,
                101u8,
                61u8,
                66u8,
                174u8,
                131u8,
                45u8,
                197u8,
                158u8,
                56u8,
                201u8,
                121u8,
                143u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 1usize {
                    return false;
                }
                if log.data.len() != 64usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address, ethabi::ParamType::Address],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    previous_admin: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    new_admin: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
        }
        impl substreams_ethereum::Event for AdminChanged1 {
            const NAME: &'static str = "AdminChanged1";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct AdminChanged2 {
            pub previous_admin: Vec<u8>,
            pub new_admin: Vec<u8>,
        }
        impl AdminChanged2 {
            const TOPIC_ID: [u8; 32] = [
                126u8,
                100u8,
                77u8,
                121u8,
                66u8,
                47u8,
                23u8,
                192u8,
                30u8,
                72u8,
                148u8,
                181u8,
                244u8,
                245u8,
                136u8,
                211u8,
                49u8,
                235u8,
                250u8,
                40u8,
                101u8,
                61u8,
                66u8,
                174u8,
                131u8,
                45u8,
                197u8,
                158u8,
                56u8,
                201u8,
                121u8,
                143u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 1usize {
                    return false;
                }
                if log.data.len() != 64usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address, ethabi::ParamType::Address],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    previous_admin: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    new_admin: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
        }
        impl substreams_ethereum::Event for AdminChanged2 {
            const NAME: &'static str = "AdminChanged2";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct BeaconUpgraded1 {
            pub beacon: Vec<u8>,
        }
        impl BeaconUpgraded1 {
            const TOPIC_ID: [u8; 32] = [
                28u8,
                243u8,
                176u8,
                58u8,
                108u8,
                241u8,
                159u8,
                162u8,
                186u8,
                186u8,
                77u8,
                241u8,
                72u8,
                233u8,
                220u8,
                171u8,
                237u8,
                234u8,
                127u8,
                138u8,
                92u8,
                7u8,
                132u8,
                14u8,
                32u8,
                126u8,
                92u8,
                8u8,
                155u8,
                233u8,
                93u8,
                62u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() != 0usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Ok(Self {
                    beacon: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'beacon' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
        }
        impl substreams_ethereum::Event for BeaconUpgraded1 {
            const NAME: &'static str = "BeaconUpgraded1";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct BeaconUpgraded2 {
            pub beacon: Vec<u8>,
        }
        impl BeaconUpgraded2 {
            const TOPIC_ID: [u8; 32] = [
                28u8,
                243u8,
                176u8,
                58u8,
                108u8,
                241u8,
                159u8,
                162u8,
                186u8,
                186u8,
                77u8,
                241u8,
                72u8,
                233u8,
                220u8,
                171u8,
                237u8,
                234u8,
                127u8,
                138u8,
                92u8,
                7u8,
                132u8,
                14u8,
                32u8,
                126u8,
                92u8,
                8u8,
                155u8,
                233u8,
                93u8,
                62u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() != 0usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Ok(Self {
                    beacon: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'beacon' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
        }
        impl substreams_ethereum::Event for BeaconUpgraded2 {
            const NAME: &'static str = "BeaconUpgraded2";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct CancelTrade {
            pub user: Vec<u8>,
            pub hash: [u8; 32usize],
            pub index: substreams::scalar::BigInt,
            pub amount: substreams::scalar::BigInt,
        }
        impl CancelTrade {
            const TOPIC_ID: [u8; 32] = [
                244u8,
                9u8,
                42u8,
                124u8,
                84u8,
                225u8,
                53u8,
                220u8,
                95u8,
                39u8,
                61u8,
                102u8,
                117u8,
                50u8,
                123u8,
                123u8,
                120u8,
                56u8,
                57u8,
                37u8,
                55u8,
                210u8,
                247u8,
                182u8,
                63u8,
                122u8,
                203u8,
                236u8,
                140u8,
                124u8,
                210u8,
                150u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() != 96usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::FixedBytes(32usize),
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    user: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'user' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    hash: {
                        let mut result = [0u8; 32];
                        let v = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                    index: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    amount: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
        }
        impl substreams_ethereum::Event for CancelTrade {
            const NAME: &'static str = "CancelTrade";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Execution {
            pub transfer: (
                Vec<u8>,
                substreams::scalar::BigInt,
                substreams::scalar::BigInt,
                Vec<u8>,
                substreams::scalar::BigInt,
            ),
            pub order_hash: [u8; 32usize],
            pub listing_index: substreams::scalar::BigInt,
            pub price: substreams::scalar::BigInt,
            pub maker_fee: (Vec<u8>, substreams::scalar::BigInt),
            pub fees: (
                (Vec<u8>, substreams::scalar::BigInt),
                (Vec<u8>, substreams::scalar::BigInt),
            ),
            pub order_type: substreams::scalar::BigInt,
        }
        impl Execution {
            const TOPIC_ID: [u8; 32] = [
                242u8,
                246u8,
                98u8,
                148u8,
                223u8,
                111u8,
                174u8,
                122u8,
                198u8,
                129u8,
                203u8,
                226u8,
                246u8,
                217u8,
                28u8,
                105u8,
                4u8,
                72u8,
                89u8,
                41u8,
                103u8,
                157u8,
                206u8,
                38u8,
                62u8,
                143u8,
                101u8,
                57u8,
                183u8,
                213u8,
                197u8,
                89u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 1usize {
                    return false;
                }
                if log.data.len() != 480usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Tuple(
                                vec![
                                    ethabi::ParamType::Address,
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Address, ethabi::ParamType::Uint(8usize)
                                ],
                            ),
                            ethabi::ParamType::FixedBytes(32usize),
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Tuple(
                                vec![
                                    ethabi::ParamType::Address, ethabi::ParamType::Uint(16usize)
                                ],
                            ),
                            ethabi::ParamType::Tuple(
                                vec![
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Address,
                                    ethabi::ParamType::Uint(16usize)]),
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Address,
                                    ethabi::ParamType::Uint(16usize)])
                                ],
                            ),
                            ethabi::ParamType::Uint(8usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    transfer: {
                        let tuple_elements = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_tuple()
                            .expect(INTERNAL_ERR);
                        (
                            tuple_elements[0usize]
                                .clone()
                                .into_address()
                                .expect(INTERNAL_ERR)
                                .as_bytes()
                                .to_vec(),
                            {
                                let mut v = [0 as u8; 32];
                                tuple_elements[1usize]
                                    .clone()
                                    .into_uint()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                            },
                            {
                                let mut v = [0 as u8; 32];
                                tuple_elements[2usize]
                                    .clone()
                                    .into_uint()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                            },
                            tuple_elements[3usize]
                                .clone()
                                .into_address()
                                .expect(INTERNAL_ERR)
                                .as_bytes()
                                .to_vec(),
                            {
                                let mut v = [0 as u8; 32];
                                tuple_elements[4usize]
                                    .clone()
                                    .into_uint()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                            },
                        )
                    },
                    order_hash: {
                        let mut result = [0u8; 32];
                        let v = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                    listing_index: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    price: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    maker_fee: {
                        let tuple_elements = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_tuple()
                            .expect(INTERNAL_ERR);
                        (
                            tuple_elements[0usize]
                                .clone()
                                .into_address()
                                .expect(INTERNAL_ERR)
                                .as_bytes()
                                .to_vec(),
                            {
                                let mut v = [0 as u8; 32];
                                tuple_elements[1usize]
                                    .clone()
                                    .into_uint()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                            },
                        )
                    },
                    fees: {
                        let tuple_elements = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_tuple()
                            .expect(INTERNAL_ERR);
                        (
                            {
                                let tuple_elements = tuple_elements[0usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec(),
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                )
                            },
                            {
                                let tuple_elements = tuple_elements[1usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec(),
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                )
                            },
                        )
                    },
                    order_type: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
        }
        impl substreams_ethereum::Event for Execution {
            const NAME: &'static str = "Execution";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Execution721MakerFeePacked {
            pub order_hash: [u8; 32usize],
            pub token_id_listing_index_trader: substreams::scalar::BigInt,
            pub collection_price_side: substreams::scalar::BigInt,
            pub maker_fee_recipient_rate: substreams::scalar::BigInt,
        }
        impl Execution721MakerFeePacked {
            const TOPIC_ID: [u8; 32] = [
                125u8,
                197u8,
                192u8,
                105u8,
                154u8,
                200u8,
                221u8,
                82u8,
                80u8,
                203u8,
                227u8,
                104u8,
                162u8,
                252u8,
                59u8,
                74u8,
                45u8,
                170u8,
                219u8,
                18u8,
                10u8,
                208u8,
                127u8,
                108u8,
                204u8,
                234u8,
                41u8,
                248u8,
                52u8,
                130u8,
                104u8,
                110u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 1usize {
                    return false;
                }
                if log.data.len() != 128usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::FixedBytes(32usize),
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    order_hash: {
                        let mut result = [0u8; 32];
                        let v = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                    token_id_listing_index_trader: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    collection_price_side: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    maker_fee_recipient_rate: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
        }
        impl substreams_ethereum::Event for Execution721MakerFeePacked {
            const NAME: &'static str = "Execution721MakerFeePacked";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Execution721Packed {
            pub order_hash: [u8; 32usize],
            pub token_id_listing_index_trader: substreams::scalar::BigInt,
            pub collection_price_side: substreams::scalar::BigInt,
        }
        impl Execution721Packed {
            const TOPIC_ID: [u8; 32] = [
                29u8,
                94u8,
                18u8,
                181u8,
                29u8,
                238u8,
                94u8,
                77u8,
                52u8,
                67u8,
                69u8,
                118u8,
                195u8,
                251u8,
                153u8,
                113u8,
                74u8,
                133u8,
                245u8,
                123u8,
                15u8,
                213u8,
                70u8,
                173u8,
                164u8,
                176u8,
                189u8,
                221u8,
                115u8,
                109u8,
                18u8,
                178u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 1usize {
                    return false;
                }
                if log.data.len() != 96usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::FixedBytes(32usize),
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    order_hash: {
                        let mut result = [0u8; 32];
                        let v = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                    token_id_listing_index_trader: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    collection_price_side: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
        }
        impl substreams_ethereum::Event for Execution721Packed {
            const NAME: &'static str = "Execution721Packed";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Execution721TakerFeePacked {
            pub order_hash: [u8; 32usize],
            pub token_id_listing_index_trader: substreams::scalar::BigInt,
            pub collection_price_side: substreams::scalar::BigInt,
            pub taker_fee_recipient_rate: substreams::scalar::BigInt,
        }
        impl Execution721TakerFeePacked {
            const TOPIC_ID: [u8; 32] = [
                15u8,
                207u8,
                23u8,
                250u8,
                193u8,
                20u8,
                19u8,
                27u8,
                16u8,
                243u8,
                123u8,
                24u8,
                60u8,
                106u8,
                96u8,
                249u8,
                5u8,
                145u8,
                30u8,
                82u8,
                128u8,
                44u8,
                174u8,
                235u8,
                62u8,
                110u8,
                162u8,
                16u8,
                57u8,
                139u8,
                129u8,
                171u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 1usize {
                    return false;
                }
                if log.data.len() != 128usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::FixedBytes(32usize),
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    order_hash: {
                        let mut result = [0u8; 32];
                        let v = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                    token_id_listing_index_trader: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    collection_price_side: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    taker_fee_recipient_rate: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
        }
        impl substreams_ethereum::Event for Execution721TakerFeePacked {
            const NAME: &'static str = "Execution721TakerFeePacked";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Initialized {
            pub version: substreams::scalar::BigInt,
        }
        impl Initialized {
            const TOPIC_ID: [u8; 32] = [
                127u8,
                38u8,
                184u8,
                63u8,
                249u8,
                110u8,
                31u8,
                43u8,
                106u8,
                104u8,
                47u8,
                19u8,
                56u8,
                82u8,
                246u8,
                121u8,
                138u8,
                9u8,
                196u8,
                101u8,
                218u8,
                149u8,
                146u8,
                20u8,
                96u8,
                206u8,
                251u8,
                56u8,
                71u8,
                64u8,
                36u8,
                152u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 1usize {
                    return false;
                }
                if log.data.len() != 32usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(8usize)],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    version: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
        }
        impl substreams_ethereum::Event for Initialized {
            const NAME: &'static str = "Initialized";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct NewBlockRange {
            pub block_range: substreams::scalar::BigInt,
        }
        impl NewBlockRange {
            const TOPIC_ID: [u8; 32] = [
                119u8,
                6u8,
                23u8,
                124u8,
                84u8,
                27u8,
                161u8,
                184u8,
                88u8,
                55u8,
                27u8,
                252u8,
                86u8,
                138u8,
                167u8,
                116u8,
                80u8,
                180u8,
                113u8,
                59u8,
                189u8,
                187u8,
                166u8,
                44u8,
                115u8,
                13u8,
                68u8,
                132u8,
                171u8,
                108u8,
                18u8,
                81u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 1usize {
                    return false;
                }
                if log.data.len() != 32usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    block_range: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
        }
        impl substreams_ethereum::Event for NewBlockRange {
            const NAME: &'static str = "NewBlockRange";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct NewGovernor {
            pub governor: Vec<u8>,
        }
        impl NewGovernor {
            const TOPIC_ID: [u8; 32] = [
                84u8,
                37u8,
                54u8,
                58u8,
                3u8,
                241u8,
                130u8,
                40u8,
                17u8,
                32u8,
                245u8,
                145u8,
                145u8,
                7u8,
                196u8,
                156u8,
                122u8,
                26u8,
                98u8,
                58u8,
                204u8,
                28u8,
                188u8,
                109u8,
                244u8,
                104u8,
                182u8,
                240u8,
                193u8,
                31u8,
                207u8,
                140u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() != 0usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Ok(Self {
                    governor: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'governor' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
        }
        impl substreams_ethereum::Event for NewGovernor {
            const NAME: &'static str = "NewGovernor";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct NewProtocolFee {
            pub recipient: Vec<u8>,
            pub rate: substreams::scalar::BigInt,
        }
        impl NewProtocolFee {
            const TOPIC_ID: [u8; 32] = [
                29u8,
                158u8,
                57u8,
                10u8,
                15u8,
                85u8,
                164u8,
                227u8,
                37u8,
                26u8,
                29u8,
                229u8,
                65u8,
                179u8,
                218u8,
                28u8,
                176u8,
                18u8,
                253u8,
                133u8,
                217u8,
                180u8,
                240u8,
                9u8,
                139u8,
                207u8,
                251u8,
                112u8,
                196u8,
                244u8,
                3u8,
                45u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 3usize {
                    return false;
                }
                if log.data.len() != 0usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Ok(Self {
                    recipient: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'recipient' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    rate: {
                        let mut v = [0 as u8; 32];
                        ethabi::decode(
                                &[ethabi::ParamType::Uint(16usize)],
                                log.topics[2usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'rate' from topic of type 'uint16': {:?}",
                                    e
                                )
                            })?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
        }
        impl substreams_ethereum::Event for NewProtocolFee {
            const NAME: &'static str = "NewProtocolFee";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct NonceIncremented {
            pub user: Vec<u8>,
            pub new_nonce: substreams::scalar::BigInt,
        }
        impl NonceIncremented {
            const TOPIC_ID: [u8; 32] = [
                168u8,
                42u8,
                100u8,
                155u8,
                189u8,
                6u8,
                12u8,
                144u8,
                153u8,
                205u8,
                123u8,
                115u8,
                38u8,
                226u8,
                176u8,
                220u8,
                158u8,
                154u8,
                240u8,
                131u8,
                100u8,
                128u8,
                224u8,
                248u8,
                73u8,
                220u8,
                158u8,
                170u8,
                121u8,
                113u8,
                11u8,
                59u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() != 32usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    user: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'user' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    new_nonce: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
        }
        impl substreams_ethereum::Event for NonceIncremented {
            const NAME: &'static str = "NonceIncremented";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct OwnershipTransferStarted {
            pub previous_owner: Vec<u8>,
            pub new_owner: Vec<u8>,
        }
        impl OwnershipTransferStarted {
            const TOPIC_ID: [u8; 32] = [
                56u8,
                209u8,
                107u8,
                140u8,
                172u8,
                34u8,
                217u8,
                159u8,
                199u8,
                193u8,
                36u8,
                185u8,
                205u8,
                13u8,
                226u8,
                211u8,
                250u8,
                31u8,
                174u8,
                244u8,
                32u8,
                191u8,
                231u8,
                145u8,
                216u8,
                195u8,
                98u8,
                215u8,
                101u8,
                226u8,
                39u8,
                0u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 3usize {
                    return false;
                }
                if log.data.len() != 0usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Ok(Self {
                    previous_owner: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'previous_owner' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    new_owner: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'new_owner' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
        }
        impl substreams_ethereum::Event for OwnershipTransferStarted {
            const NAME: &'static str = "OwnershipTransferStarted";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct OwnershipTransferred {
            pub previous_owner: Vec<u8>,
            pub new_owner: Vec<u8>,
        }
        impl OwnershipTransferred {
            const TOPIC_ID: [u8; 32] = [
                139u8,
                224u8,
                7u8,
                156u8,
                83u8,
                22u8,
                89u8,
                20u8,
                19u8,
                68u8,
                205u8,
                31u8,
                208u8,
                164u8,
                242u8,
                132u8,
                25u8,
                73u8,
                127u8,
                151u8,
                34u8,
                163u8,
                218u8,
                175u8,
                227u8,
                180u8,
                24u8,
                111u8,
                107u8,
                100u8,
                87u8,
                224u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 3usize {
                    return false;
                }
                if log.data.len() != 0usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Ok(Self {
                    previous_owner: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'previous_owner' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    new_owner: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'new_owner' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
        }
        impl substreams_ethereum::Event for OwnershipTransferred {
            const NAME: &'static str = "OwnershipTransferred";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct SetOracle {
            pub user: Vec<u8>,
            pub approved: bool,
        }
        impl SetOracle {
            const TOPIC_ID: [u8; 32] = [
                204u8,
                133u8,
                39u8,
                146u8,
                183u8,
                175u8,
                174u8,
                19u8,
                201u8,
                144u8,
                55u8,
                104u8,
                92u8,
                144u8,
                221u8,
                59u8,
                228u8,
                64u8,
                115u8,
                212u8,
                188u8,
                50u8,
                170u8,
                140u8,
                27u8,
                114u8,
                253u8,
                7u8,
                162u8,
                172u8,
                83u8,
                124u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() != 32usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Bool],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    user: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'user' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    approved: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bool()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for SetOracle {
            const NAME: &'static str = "SetOracle";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Upgraded1 {
            pub implementation: Vec<u8>,
        }
        impl Upgraded1 {
            const TOPIC_ID: [u8; 32] = [
                188u8,
                124u8,
                215u8,
                90u8,
                32u8,
                238u8,
                39u8,
                253u8,
                154u8,
                222u8,
                186u8,
                179u8,
                32u8,
                65u8,
                247u8,
                85u8,
                33u8,
                77u8,
                188u8,
                107u8,
                255u8,
                169u8,
                12u8,
                192u8,
                34u8,
                91u8,
                57u8,
                218u8,
                46u8,
                92u8,
                45u8,
                59u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() != 0usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Ok(Self {
                    implementation: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'implementation' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
        }
        impl substreams_ethereum::Event for Upgraded1 {
            const NAME: &'static str = "Upgraded1";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Upgraded2 {
            pub implementation: Vec<u8>,
        }
        impl Upgraded2 {
            const TOPIC_ID: [u8; 32] = [
                188u8,
                124u8,
                215u8,
                90u8,
                32u8,
                238u8,
                39u8,
                253u8,
                154u8,
                222u8,
                186u8,
                179u8,
                32u8,
                65u8,
                247u8,
                85u8,
                33u8,
                77u8,
                188u8,
                107u8,
                255u8,
                169u8,
                12u8,
                192u8,
                34u8,
                91u8,
                57u8,
                218u8,
                46u8,
                92u8,
                45u8,
                59u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() != 0usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Ok(Self {
                    implementation: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'implementation' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
        }
        impl substreams_ethereum::Event for Upgraded2 {
            const NAME: &'static str = "Upgraded2";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
    }