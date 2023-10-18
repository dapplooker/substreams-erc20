// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Block {
    #[prost(message, repeated, tag="1")]
    pub storage_changes: ::prost::alloc::vec::Vec<BalanceOfStorageChange>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalanceOfStorageChange {
    /// contract address
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    /// string name = 2;
    /// string decimals = 3;
    #[prost(string, tag="4")]
    pub method: ::prost::alloc::string::String,
    /// storage changes
    #[prost(string, tag="5")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub balance: ::prost::alloc::string::String,
    /// trace information
    #[prost(string, tag="7")]
    pub transaction: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Erc20Token {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub decimals: u64,
}
// @@protoc_insertion_point(module)
