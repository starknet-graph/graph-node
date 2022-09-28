#[allow(clippy::all)]
#[rustfmt::skip]
#[path = "protobuf/zklend.starknet.r#type.v1.rs"]
pub mod pbcodec;

use graph::blockchain::{Block as BlockchainBlock, BlockPtr};

pub use pbcodec::*;

impl BlockchainBlock for Block {
    fn number(&self) -> i32 {
        todo!()
    }

    fn ptr(&self) -> BlockPtr {
        todo!()
    }

    fn parent_ptr(&self) -> Option<BlockPtr> {
        todo!()
    }
}
