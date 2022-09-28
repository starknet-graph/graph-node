// This file only contains the bare minimum types for the POC. It's far from a complete
// representation of a StarkNet network's history as required by the Firehose protocol. As a result,
// any future changes to this schema would require a full re-sync of the StarkNet node.

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Block {
    #[prost(uint64, tag="1")]
    pub height: u64,
    #[prost(bytes="vec", tag="2")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub prev_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="4")]
    pub timestamp: u64,
    #[prost(message, repeated, tag="5")]
    pub transactions: ::prost::alloc::vec::Vec<Transaction>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    #[prost(enumeration="TransactionType", tag="1")]
    pub r#type: i32,
    #[prost(bytes="vec", tag="2")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag="3")]
    pub events: ::prost::alloc::vec::Vec<Event>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    #[prost(bytes="vec", tag="1")]
    pub from_addr: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", repeated, tag="2")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", repeated, tag="3")]
    pub data: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TransactionType {
    Deploy = 0,
    InvokeFunction = 1,
    Declare = 2,
    L1Handler = 3,
}
