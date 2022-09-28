use graph::blockchain::{Block as BlockchainBlock, BlockPtr};

#[derive(Debug, Clone, Default)]
pub struct Block;

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
