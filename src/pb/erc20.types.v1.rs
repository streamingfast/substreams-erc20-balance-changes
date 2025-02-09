// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Events {
    #[prost(message, repeated, tag="1")]
    pub transfers: ::prost::alloc::vec::Vec<Transfer>,
    #[prost(message, repeated, tag="2")]
    pub balance_changes: ::prost::alloc::vec::Vec<BalanceChange>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalanceChange {
    /// -- block --
    #[prost(uint64, tag="1")]
    pub block_num: u64,
    #[prost(string, tag="2")]
    pub block_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag="4")]
    pub date: ::prost::alloc::string::String,
    /// -- transaction --
    #[prost(string, tag="5")]
    pub transaction_id: ::prost::alloc::string::String,
    /// -- call --
    #[prost(uint32, tag="6")]
    pub call_index: u32,
    /// -- log --
    ///
    /// Index is the index of the log relative to the transaction. This index is always populated regardless of the state revertion of the the call that emitted this log.
    #[prost(uint32, tag="7")]
    pub log_index: u32,
    /// BlockIndex represents the index of the log relative to the Block.
    #[prost(uint32, tag="8")]
    pub log_block_index: u32,
    /// the block's global ordinal when the transfer was recorded.
    #[prost(uint64, tag="9")]
    pub log_ordinal: u64,
    /// -- storage change --
    #[prost(string, tag="10")]
    pub storage_key: ::prost::alloc::string::String,
    #[prost(uint64, tag="11")]
    pub storage_ordinal: u64,
    /// -- indexing --
    ///
    /// incrementing index for ordering
    #[prost(uint32, tag="12")]
    pub index: u32,
    /// version used as “ordering” argument (block_num << 32 + index)
    #[prost(uint64, tag="13")]
    pub version: u64,
    /// -- balance change --
    #[prost(string, tag="14")]
    pub contract: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag="16")]
    pub old_balance: ::prost::alloc::string::String,
    #[prost(string, tag="17")]
    pub new_balance: ::prost::alloc::string::String,
    /// difference between old_balance and new_balance
    #[prost(string, tag="18")]
    pub amount: ::prost::alloc::string::String,
    #[prost(enumeration="BalanceChangeType", tag="19")]
    pub change_type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transfer {
    /// -- block --
    #[prost(uint64, tag="1")]
    pub block_num: u64,
    #[prost(string, tag="2")]
    pub block_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag="4")]
    pub date: ::prost::alloc::string::String,
    /// -- transaction --
    #[prost(string, tag="5")]
    pub transaction_id: ::prost::alloc::string::String,
    /// -- call --
    #[prost(uint32, tag="6")]
    pub call_index: u32,
    /// -- log --
    ///
    /// Index is the index of the log relative to the transaction. This index is always populated regardless of the state revertion of the the call that emitted this log.
    #[prost(uint32, tag="7")]
    pub log_index: u32,
    /// BlockIndex represents the index of the log relative to the Block.
    #[prost(uint32, tag="8")]
    pub log_block_index: u32,
    /// the block's global ordinal when the transfer was recorded.
    #[prost(uint64, tag="9")]
    pub log_ordinal: u64,
    /// -- transfer --
    #[prost(string, tag="10")]
    pub contract: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub value: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalanceChangeStats {
    #[prost(uint64, tag="1")]
    pub type0_count: u64,
    #[prost(uint64, tag="2")]
    pub type1_count: u64,
    #[prost(uint64, tag="3")]
    pub type2_count: u64,
    #[prost(uint64, tag="42")]
    pub total_count: u64,
    #[prost(string, tag="43")]
    pub valid_rate: ::prost::alloc::string::String,
    #[prost(uint64, tag="99")]
    pub block_number: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BalanceChangeType {
    /// cannot determine balance change
    Unspecified = 0,
    /// easy case where storage change is in the same call as the Transfer call
    BalanceChangeType1 = 1,
    /// storage change is in a different call than the Transfer call
    BalanceChangeType2 = 2,
}
impl BalanceChangeType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BalanceChangeType::Unspecified => "BALANCE_CHANGE_TYPE_UNSPECIFIED",
            BalanceChangeType::BalanceChangeType1 => "BALANCE_CHANGE_TYPE_1",
            BalanceChangeType::BalanceChangeType2 => "BALANCE_CHANGE_TYPE_2",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BALANCE_CHANGE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "BALANCE_CHANGE_TYPE_1" => Some(Self::BalanceChangeType1),
            "BALANCE_CHANGE_TYPE_2" => Some(Self::BalanceChangeType2),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
