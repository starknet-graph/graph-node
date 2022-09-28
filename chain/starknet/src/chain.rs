use graph::{
    anyhow::Result,
    blockchain::{
        block_stream::{BlockStream, FirehoseCursor, TriggersAdapter as TriggersAdapterTrait},
        BlockPtr, Blockchain, BlockchainKind, EmptyNodeCapabilities, IngestorError,
        RuntimeAdapter as RuntimeAdapterTrait,
    },
    components::store::DeploymentLocator,
    data::subgraph::UnifiedMappingApiVersion,
    prelude::{async_trait, BlockNumber, ChainStore, Error, Logger},
};
use std::sync::Arc;

use crate::{
    codec,
    data_source::{
        DataSource, DataSourceTemplate, UnresolvedDataSource, UnresolvedDataSourceTemplate,
    },
};

#[derive(Debug)]
pub struct Chain;

#[async_trait]
impl Blockchain for Chain {
    // TODO: change this to new variant once added
    const KIND: BlockchainKind = BlockchainKind::Ethereum;

    type Block = codec::Block;
    type DataSource = DataSource;
    type UnresolvedDataSource = UnresolvedDataSource;

    type DataSourceTemplate = DataSourceTemplate;
    type UnresolvedDataSourceTemplate = UnresolvedDataSourceTemplate;

    type TriggerData = crate::trigger::StarknetTrigger;

    type MappingTrigger = crate::trigger::StarknetTrigger;

    type TriggerFilter = crate::adapter::TriggerFilter;

    type NodeCapabilities = EmptyNodeCapabilities<Self>;

    #[allow(unused)]
    fn triggers_adapter(
        &self,
        log: &DeploymentLocator,
        capabilities: &Self::NodeCapabilities,
        unified_api_version: UnifiedMappingApiVersion,
    ) -> Result<Arc<dyn TriggersAdapterTrait<Self>>, Error> {
        todo!()
    }

    #[allow(unused)]
    async fn new_firehose_block_stream(
        &self,
        deployment: DeploymentLocator,
        block_cursor: FirehoseCursor,
        start_blocks: Vec<BlockNumber>,
        subgraph_current_block: Option<BlockPtr>,
        filter: Arc<Self::TriggerFilter>,
        unified_api_version: UnifiedMappingApiVersion,
    ) -> Result<Box<dyn BlockStream<Self>>, Error> {
        todo!()
    }

    fn is_refetch_block_required(&self) -> bool {
        todo!()
    }

    #[allow(unused)]
    async fn refetch_firehose_block(
        &self,
        logger: &Logger,
        cursor: FirehoseCursor,
    ) -> Result<codec::Block, Error> {
        todo!()
    }

    #[allow(unused)]
    async fn new_polling_block_stream(
        &self,
        deployment: DeploymentLocator,
        start_blocks: Vec<BlockNumber>,
        subgraph_current_block: Option<BlockPtr>,
        filter: Arc<Self::TriggerFilter>,
        unified_api_version: UnifiedMappingApiVersion,
    ) -> Result<Box<dyn BlockStream<Self>>, Error> {
        todo!()
    }

    fn chain_store(&self) -> Arc<dyn ChainStore> {
        todo!()
    }

    #[allow(unused)]
    async fn block_pointer_from_number(
        &self,
        logger: &Logger,
        number: BlockNumber,
    ) -> Result<BlockPtr, IngestorError> {
        todo!()
    }

    fn runtime_adapter(&self) -> Arc<dyn RuntimeAdapterTrait<Self>> {
        todo!()
    }

    fn is_firehose_supported(&self) -> bool {
        todo!()
    }
}
