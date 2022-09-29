use graph::{
    prelude::BigInt,
    runtime::{asc_new, gas::GasCounter, AscHeap, DeterministicHostError, ToAscObj},
};
use graph_runtime_wasm::asc_abi::class::{Array, AscEnum, EnumPayload};

use crate::codec;

pub(crate) use super::generated::*;

impl ToAscObj<AscBlock> for codec::Block {
    fn to_asc_obj<H: AscHeap + ?Sized>(
        &self,
        heap: &mut H,
        gas: &GasCounter,
    ) -> Result<AscBlock, DeterministicHostError> {
        Ok(AscBlock {
            height: asc_new(heap, &BigInt::from(self.height), gas)?,
            hash: asc_new(heap, self.hash.as_slice(), gas)?,
            prev_hash: asc_new(heap, self.prev_hash.as_slice(), gas)?,
            timestamp: asc_new(heap, &BigInt::from(self.timestamp), gas)?,
            transactions: asc_new(heap, &self.transactions, gas)?,
        })
    }
}

impl ToAscObj<AscTransaction> for codec::Transaction {
    fn to_asc_obj<H: AscHeap + ?Sized>(
        &self,
        heap: &mut H,
        gas: &GasCounter,
    ) -> Result<AscTransaction, DeterministicHostError> {
        Ok(AscTransaction {
            r#type: asc_new(
                heap,
                &codec::TransactionType::from_i32(self.r#type)
                    .expect("invalid TransactionType value"),
                gas,
            )?,
            hash: asc_new(heap, self.hash.as_slice(), gas)?,
            events: asc_new(heap, &self.events, gas)?,
        })
    }
}

impl ToAscObj<AscTransactionTypeEnum> for codec::TransactionType {
    fn to_asc_obj<H: AscHeap + ?Sized>(
        &self,
        _heap: &mut H,
        _gas: &GasCounter,
    ) -> Result<AscTransactionTypeEnum, DeterministicHostError> {
        Ok(AscTransactionTypeEnum(AscEnum {
            kind: match self {
                codec::TransactionType::Deploy => AscTransactionType::Deploy,
                codec::TransactionType::InvokeFunction => AscTransactionType::InvokeFunction,
                codec::TransactionType::Declare => AscTransactionType::Declare,
                codec::TransactionType::L1Handler => AscTransactionType::L1Handler,
            },
            _padding: 0,
            payload: EnumPayload(0),
        }))
    }
}

impl ToAscObj<AscEvent> for codec::Event {
    fn to_asc_obj<H: AscHeap + ?Sized>(
        &self,
        heap: &mut H,
        gas: &GasCounter,
    ) -> Result<AscEvent, DeterministicHostError> {
        Ok(AscEvent {
            from_addr: asc_new(heap, self.from_addr.as_slice(), gas)?,
            keys: asc_new(heap, &self.keys, gas)?,
            data: asc_new(heap, &self.data, gas)?,
        })
    }
}

impl ToAscObj<AscTransactionArray> for Vec<codec::Transaction> {
    fn to_asc_obj<H: AscHeap + ?Sized>(
        &self,
        heap: &mut H,
        gas: &GasCounter,
    ) -> Result<AscTransactionArray, DeterministicHostError> {
        let content: Result<Vec<_>, _> = self.iter().map(|x| asc_new(heap, x, gas)).collect();

        Ok(AscTransactionArray(Array::new(&content?, heap, gas)?))
    }
}

impl ToAscObj<AscEventArray> for Vec<codec::Event> {
    fn to_asc_obj<H: AscHeap + ?Sized>(
        &self,
        heap: &mut H,
        gas: &GasCounter,
    ) -> Result<AscEventArray, DeterministicHostError> {
        let content: Result<Vec<_>, _> = self.iter().map(|x| asc_new(heap, x, gas)).collect();

        Ok(AscEventArray(Array::new(&content?, heap, gas)?))
    }
}

impl ToAscObj<AscBytesArray> for Vec<Vec<u8>> {
    fn to_asc_obj<H: AscHeap + ?Sized>(
        &self,
        heap: &mut H,
        gas: &GasCounter,
    ) -> Result<AscBytesArray, DeterministicHostError> {
        let content: Result<Vec<_>, _> = self
            .iter()
            .map(|x| asc_new(heap, x.as_slice(), gas))
            .collect();

        Ok(AscBytesArray(Array::new(&content?, heap, gas)?))
    }
}
