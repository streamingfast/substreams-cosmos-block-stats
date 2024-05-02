// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockStats {
    #[prost(uint64, tag="1")]
    pub block_height: u64,
    #[prost(string, tag="2")]
    pub block_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag="4")]
    pub block_proposer: ::prost::alloc::string::String,
    #[prost(int64, tag="5")]
    pub parent_height: i64,
    #[prost(string, tag="6")]
    pub parent_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="7")]
    pub num_txs: u64,
    #[prost(message, optional, tag="8")]
    pub block: ::core::option::Option<super::super::sf::cosmos::r#type::v2::Block>,
}
// @@protoc_insertion_point(module)
