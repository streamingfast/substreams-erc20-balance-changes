// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalanceChanges {
    #[prost(message, repeated, tag="1")]
    pub balance_changes: ::prost::alloc::vec::Vec<BalanceChange>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalanceChange {
    /// 0xmytokencontract
    #[prost(string, tag="1")]
    pub contract: ::prost::alloc::string::String,
    /// 0xmyaddress
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
    /// 0x1234
    #[prost(string, tag="3")]
    pub old_balance: ::prost::alloc::string::String,
    /// 0x1234
    #[prost(string, tag="4")]
    pub new_balance: ::prost::alloc::string::String,
    /// 0xmytransaction
    #[prost(string, tag="5")]
    pub transaction: ::prost::alloc::string::String,
    /// 0xmykey
    #[prost(string, tag="6")]
    pub storage_key: ::prost::alloc::string::String,
    /// 0xmycallindex
    #[prost(string, tag="7")]
    pub call_index: ::prost::alloc::string::String,
    /// 0x1234
    #[prost(string, tag="8")]
    pub transfer_value: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
