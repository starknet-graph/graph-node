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
            number: asc_new(heap, &BigInt::from(self.height), gas)?,
            hash: asc_new(heap, self.hash.as_slice(), gas)?,
            prev_hash: asc_new(heap, self.prev_hash.as_slice(), gas)?,
            timestamp: asc_new(heap, &BigInt::from(self.timestamp), gas)?,
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
                codec::TransactionType::DeployAccount => AscTransactionType::DeployAccount,
            },
            _padding: 0,
            payload: EnumPayload(0),
        }))
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
