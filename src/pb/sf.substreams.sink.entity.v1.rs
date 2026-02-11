// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityChanges {
    #[prost(message, repeated, tag="1")]
    pub entity_changes: ::prost::alloc::vec::Vec<EntityChange>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityChange {
    #[prost(string, tag="1")]
    pub entity: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub ordinal: u64,
    #[prost(enumeration="Operation", tag="4")]
    pub operation: i32,
    #[prost(message, repeated, tag="5")]
    pub fields: ::prost::alloc::vec::Vec<Field>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Field {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(oneof="field::TypedValue", tags="2, 3, 4, 5, 6, 7, 8, 9")]
    pub typed_value: ::core::option::Option<field::TypedValue>,
}
/// Nested message and enum types in `Field`.
pub mod field {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TypedValue {
        #[prost(string, tag="2")]
        StringValue(::prost::alloc::string::String),
        #[prost(int64, tag="3")]
        Int64Value(i64),
        #[prost(double, tag="4")]
        Float64Value(f64),
        #[prost(bytes, tag="5")]
        BytesValue(::prost::alloc::vec::Vec<u8>),
        #[prost(bool, tag="6")]
        BoolValue(bool),
        #[prost(message, tag="7")]
        BigintValue(super::BigInt),
        #[prost(message, tag="8")]
        BigdecimalValue(super::BigDecimal),
        #[prost(message, tag="9")]
        ArrayValue(super::Array),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigInt {
    #[prost(bytes="vec", tag="1")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigDecimal {
    #[prost(bytes="vec", tag="1")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Array {
    #[prost(message, repeated, tag="1")]
    pub values: ::prost::alloc::vec::Vec<Value>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Value {
    #[prost(oneof="value::Typed", tags="1, 2, 3, 4, 5, 6, 7")]
    pub typed: ::core::option::Option<value::Typed>,
}
/// Nested message and enum types in `Value`.
pub mod value {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Typed {
        #[prost(string, tag="1")]
        String(::prost::alloc::string::String),
        #[prost(int64, tag="2")]
        Int64(i64),
        #[prost(double, tag="3")]
        Float64(f64),
        #[prost(bytes, tag="4")]
        Bytes(::prost::alloc::vec::Vec<u8>),
        #[prost(bool, tag="5")]
        Bool(bool),
        #[prost(message, tag="6")]
        Bigint(super::BigInt),
        #[prost(message, tag="7")]
        Bigdecimal(super::BigDecimal),
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Operation {
    Unspecified = 0,
    Create = 1,
    Update = 2,
    Delete = 3,
}
impl Operation {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Operation::Unspecified => "OPERATION_UNSPECIFIED",
            Operation::Create => "OPERATION_CREATE",
            Operation::Update => "OPERATION_UPDATE",
            Operation::Delete => "OPERATION_DELETE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OPERATION_UNSPECIFIED" => Some(Self::Unspecified),
            "OPERATION_CREATE" => Some(Self::Create),
            "OPERATION_UPDATE" => Some(Self::Update),
            "OPERATION_DELETE" => Some(Self::Delete),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
