use graph::{
    blockchain::TriggerData,
    runtime::{asc_new, gas::GasCounter, AscPtr, DeterministicHostError},
};
use graph_runtime_wasm::module::ToAscPtr;
use std::{cmp::Ordering, sync::Arc};

use crate::codec;

#[derive(Debug, Clone)]
pub enum StarknetTrigger {
    Block(Arc<codec::Block>),
}

impl PartialEq for StarknetTrigger {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Block(l0), Self::Block(r0)) => l0 == r0,
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
        }
    }
}

impl TriggerData for StarknetTrigger {
    fn error_context(&self) -> String {
        match self {
            Self::Block(block) => format!("block #{}", block.height),
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
        })
    }
}
