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
    #[prost(enumeration="BalanceChangeType", tag="9")]
    pub r#type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidBalanceChangeStats {
    #[prost(uint64, tag="1")]
    pub type0_count: u64,
    #[prost(uint64, tag="2")]
    pub type1_count: u64,
    #[prost(uint64, tag="3")]
    pub type2_count: u64,
    #[prost(uint64, tag="4")]
    pub type66_count: u64,
    #[prost(uint64, tag="42")]
    pub total_count: u64,
    #[prost(uint64, tag="99")]
    pub block_number: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BalanceChangeType {
    /// easy case where storage change is in the same call as the Transfer call
    Type0 = 0,
    /// storage change is in a different call than the Transfer call
    Type1 = 1,
    /// {unused}
    Type2 = 2,
    /// cannot determine balance change
    TypeUnknown = 99,
}
impl BalanceChangeType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BalanceChangeType::Type0 => "TYPE_0",
            BalanceChangeType::Type1 => "TYPE_1",
            BalanceChangeType::Type2 => "TYPE_2",
            BalanceChangeType::TypeUnknown => "TYPE_UNKNOWN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TYPE_0" => Some(Self::Type0),
            "TYPE_1" => Some(Self::Type1),
            "TYPE_2" => Some(Self::Type2),
            "TYPE_UNKNOWN" => Some(Self::TypeUnknown),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
