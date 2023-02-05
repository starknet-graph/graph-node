use graph::{
    blockchain::TriggerData,
    runtime::{asc_new, gas::GasCounter, AscPtr, DeterministicHostError},
};
use graph_runtime_wasm::module::ToAscPtr;
use starknet_core::types::FieldElement;
use std::{cmp::Ordering, sync::Arc};

use crate::codec;

#[derive(Debug, Clone)]
pub enum StarknetTrigger {
    Block(Arc<codec::Block>),
    Event(Arc<codec::Event>),
}

impl PartialEq for StarknetTrigger {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Block(l0), Self::Block(r0)) => l0 == r0,
            (Self::Event(a), Self::Event(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for StarknetTrigger {}

impl PartialOrd for StarknetTrigger {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for StarknetTrigger {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Block(l0), Self::Block(r0)) => l0.height.cmp(&r0.height),

            // Block triggers always come last
            (Self::Block(..), _) => Ordering::Greater,
            (_, Self::Block(..)) => Ordering::Less,

            // Keep the order when comparing two event triggers
            (Self::Event(..), Self::Event(..)) => Ordering::Equal,
        }
    }
}

impl TriggerData for StarknetTrigger {
    fn error_context(&self) -> String {
        match self {
            Self::Block(block) => format!("block #{}", block.height),
            Self::Event(event) => {
                format!(
                    "event from {}",
                    match FieldElement::from_byte_slice_be(&event.from_addr) {
                        Ok(from_addr) => format!("{from_addr:#x}"),
                        Err(_) => "[unable to parse source address]".into(),
                    }
                )
            }
        }
    }
}

impl ToAscPtr for StarknetTrigger {
    #[allow(unused)]
    fn to_asc_ptr<H: graph::runtime::AscHeap>(
        self,
        heap: &mut H,
        gas: &GasCounter,
    ) -> Result<AscPtr<()>, DeterministicHostError> {
        Ok(match self {
            StarknetTrigger::Block(block) => asc_new(heap, block.as_ref(), gas)?.erase(),
            StarknetTrigger::Event(event) => asc_new(heap, event.as_ref(), gas)?.erase(),
        })
    }
}
