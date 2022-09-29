use graph::{
    anyhow::Result,
    blockchain::{
        block_stream::{
            BlockStream, BlockStreamBuilder, BlockStreamEvent, BlockWithTriggers, FirehoseCursor,
            FirehoseError, FirehoseMapper as FirehoseMapperTrait,
            TriggersAdapter as TriggersAdapterTrait,
        },
        firehose_block_stream::FirehoseBlockStream,
        Block, BlockPtr, Blockchain, BlockchainKind, EmptyNodeCapabilities, IngestorError,
        RuntimeAdapter as RuntimeAdapterTrait,
    },
    cheap_clone::CheapClone,
    components::store::DeploymentLocator,
    data::subgraph::UnifiedMappingApiVersion,
    firehose::{self, FirehoseEndpoint, FirehoseEndpoints, ForkStep},
    prelude::{
        async_trait, BlockHash, BlockNumber, ChainStore, Error, Logger, LoggerFactory,
        MetricsRegistry, TryFutureExt,
    },
    slog::o,
};
use prost::Message;
use std::sync::Arc;

use crate::{
    adapter::TriggerFilter,
    codec,
    data_source::{
        DataSource, DataSourceTemplate, UnresolvedDataSource, UnresolvedDataSourceTemplate,
    },
    runtime::RuntimeAdapter,
    trigger::StarknetTrigger,
};

pub struct Chain {
    logger_factory: LoggerFactory,
    name: String,
    registry: Arc<dyn MetricsRegistry>,
    firehose_endpoints: Arc<FirehoseEndpoints>,
    chain_store: Arc<dyn ChainStore>,
    block_stream_builder: Arc<dyn BlockStreamBuilder<Self>>,
}

pub struct StarknetStreamBuilder;

#[allow(unused)]
pub struct FirehoseMapper {
    endpoint: Arc<FirehoseEndpoint>,
}

pub struct TriggersAdapter;

impl Chain {
    pub fn new(
        logger_factory: LoggerFactory,
        name: String,
        registry: Arc<dyn MetricsRegistry>,
        firehose_endpoints: FirehoseEndpoints,
        chain_store: Arc<dyn ChainStore>,
        block_stream_builder: Arc<dyn BlockStreamBuilder<Self>>,
    ) -> Self {
        Self {
            logger_factory,
            name,
            registry,
            firehose_endpoints: Arc::new(firehose_endpoints),
            chain_store,
            block_stream_builder,
        }
    }
}

impl std::fmt::Debug for Chain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "chain: starknet")
    }
}

#[async_trait]
impl Blockchain for Chain {
    const KIND: BlockchainKind = BlockchainKind::Starknet;

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
        Ok(Arc::new(TriggersAdapter))
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
        self.block_stream_builder
            .build_firehose(
                self,
                deployment,
                block_cursor,
                start_blocks,
                subgraph_current_block,
                filter,
                unified_api_version,
            )
            .await
    }

    fn is_refetch_block_required(&self) -> bool {
        false
    }

    #[allow(unused)]
    async fn refetch_firehose_block(
        &self,
        logger: &Logger,
        cursor: FirehoseCursor,
    ) -> Result<codec::Block, Error> {
        // Taken from `chain/near/src/chain.rs`
        // TODO: check if we can support this
        unimplemented!("This chain does not support Dynamic Data Sources. is_refetch_block_required always returns false, this shouldn't be called.")
    }

    #[allow(unused)]
    async fn new_polling_block_stream(
        &self,
        _deployment: DeploymentLocator,
        _start_blocks: Vec<BlockNumber>,
        _subgraph_current_block: Option<BlockPtr>,
        _filter: Arc<Self::TriggerFilter>,
        _unified_api_version: UnifiedMappingApiVersion,
    ) -> Result<Box<dyn BlockStream<Self>>, Error> {
        panic!("StarkNet does not support polling block stream")
    }

    fn chain_store(&self) -> Arc<dyn ChainStore> {
        self.chain_store.clone()
    }

    async fn block_pointer_from_number(
        &self,
        logger: &Logger,
        number: BlockNumber,
    ) -> Result<BlockPtr, IngestorError> {
        let firehose_endpoint = self.firehose_endpoints.random()?;

        firehose_endpoint
            .block_ptr_for_number::<codec::Block>(logger, number)
            .map_err(Into::into)
            .await
    }

    fn runtime_adapter(&self) -> Arc<dyn RuntimeAdapterTrait<Self>> {
        Arc::new(RuntimeAdapter)
    }

    fn is_firehose_supported(&self) -> bool {
        true
    }
}

#[async_trait]
impl BlockStreamBuilder<Chain> for StarknetStreamBuilder {
    async fn build_firehose(
        &self,
        chain: &Chain,
        deployment: DeploymentLocator,
        block_cursor: FirehoseCursor,
        start_blocks: Vec<BlockNumber>,
        subgraph_current_block: Option<BlockPtr>,
        filter: Arc<TriggerFilter>,
        unified_api_version: UnifiedMappingApiVersion,
    ) -> Result<Box<dyn BlockStream<Chain>>> {
        let adapter = chain
            .triggers_adapter(
                &deployment,
                &EmptyNodeCapabilities::default(),
                unified_api_version,
            )
            .unwrap_or_else(|_| panic!("no adapter for network {}", chain.name));

        let firehose_endpoint = chain.firehose_endpoints.random()?;

        let logger = chain
            .logger_factory
            .subgraph_logger(&deployment)
            .new(o!("component" => "FirehoseBlockStream"));

        let firehose_mapper = Arc::new(FirehoseMapper {
            endpoint: firehose_endpoint.cheap_clone(),
        });

        Ok(Box::new(FirehoseBlockStream::new(
            deployment.hash,
            firehose_endpoint,
            subgraph_current_block,
            block_cursor,
            firehose_mapper,
            adapter,
            filter,
            start_blocks,
            logger,
            chain.registry.clone(),
        )))
    }

    async fn build_polling(
        &self,
        _chain: Arc<Chain>,
        _deployment: DeploymentLocator,
        _start_blocks: Vec<BlockNumber>,
        _subgraph_current_block: Option<BlockPtr>,
        _filter: Arc<TriggerFilter>,
        _unified_api_version: UnifiedMappingApiVersion,
    ) -> Result<Box<dyn BlockStream<Chain>>> {
        panic!("StarkNet does not support polling block stream")
    }
}

#[async_trait]
impl FirehoseMapperTrait<Chain> for FirehoseMapper {
    async fn to_block_stream_event(
        &self,
        logger: &Logger,
        response: &firehose::Response,
        adapter: &Arc<dyn TriggersAdapterTrait<Chain>>,
        filter: &TriggerFilter,
    ) -> Result<BlockStreamEvent<Chain>, FirehoseError> {
        let step = ForkStep::from_i32(response.step).unwrap_or_else(|| {
            panic!(
                "unknown step i32 value {}, maybe you forgot update & re-regenerate the protobuf definitions?",
                response.step
            )
        });

        let any_block = response
            .block
            .as_ref()
            .expect("block payload information should always be present");

        // Right now, this is done in all cases but in reality, with how the BlockStreamEvent::Revert
        // is defined right now, only block hash and block number is necessary. However, this information
        // is not part of the actual bstream::BlockResponseV2 payload. As such, we need to decode the full
        // block which is useless.
        //
        // Check about adding basic information about the block in the bstream::BlockResponseV2 or maybe
        // define a slimmed down stuct that would decode only a few fields and ignore all the rest.
        let block = codec::Block::decode(any_block.value.as_ref())?;

        use ForkStep::*;
        match step {
            StepNew => Ok(BlockStreamEvent::ProcessBlock(
                adapter.triggers_in_block(logger, block, filter).await?,
                FirehoseCursor::from(response.cursor.clone()),
            )),

            StepUndo => {
                let parent_ptr = block
                    .parent_ptr()
                    .expect("Genesis block should never be reverted");

                Ok(BlockStreamEvent::Revert(
                    parent_ptr,
                    FirehoseCursor::from(response.cursor.clone()),
                ))
            }

            StepFinal => {
                panic!("irreversible step is not handled and should not be requested in the Firehose request")
            }

            StepUnset => {
                panic!("unknown step should not happen in the Firehose response")
            }
        }
    }

    /// Returns the [BlockPtr] value for this given block number. This is the block pointer
    /// of the longuest according to Firehose view of the blockchain state.
    ///
    /// This is a thin wrapper around [FirehoseEndpoint#block_ptr_for_number] to make
    /// it chain agnostic and callable from chain agnostic [FirehoseBlockStream].
    #[allow(unused)]
    async fn block_ptr_for_number(
        &self,
        logger: &Logger,
        endpoint: &Arc<FirehoseEndpoint>,
        number: BlockNumber,
    ) -> Result<BlockPtr, Error> {
        todo!()
    }

    /// Returns the closest final block ptr to the block ptr received.
    /// On probablitics chain like Ethereum, final is determined by
    /// the confirmations threshold configured for the Firehose stack (currently
    /// hard-coded to 200).
    ///
    /// On some other chain like NEAR, the actual final block number is determined
    /// from the block itself since it contains information about which block number
    /// is final against the current block.
    ///
    /// To take an example, assuming we are on Ethereum, the final block pointer
    /// for block #10212 would be the determined final block #10012 (10212 - 200 = 10012).
    #[allow(unused)]
    async fn final_block_ptr_for(
        &self,
        logger: &Logger,
        endpoint: &Arc<FirehoseEndpoint>,
        block: &codec::Block,
    ) -> Result<BlockPtr, Error> {
        todo!()
    }
}

#[async_trait]
impl TriggersAdapterTrait<Chain> for TriggersAdapter {
    // Return the block that is `offset` blocks before the block pointed to
    // by `ptr` from the local cache. An offset of 0 means the block itself,
    // an offset of 1 means the block's parent etc. If the block is not in
    // the local cache, return `None`
    #[allow(unused)]
    async fn ancestor_block(
        &self,
        ptr: BlockPtr,
        offset: BlockNumber,
    ) -> Result<Option<codec::Block>, Error> {
        todo!()
    }

    // Returns a sequence of blocks in increasing order of block number.
    // Each block will include all of its triggers that match the given `filter`.
    // The sequence may omit blocks that contain no triggers,
    // but all returned blocks must part of a same chain starting at `chain_base`.
    // At least one block will be returned, even if it contains no triggers.
    // `step_size` is the suggested number blocks to be scanned.
    #[allow(unused)]
    async fn scan_triggers(
        &self,
        from: BlockNumber,
        to: BlockNumber,
        filter: &crate::adapter::TriggerFilter,
    ) -> Result<Vec<BlockWithTriggers<Chain>>, Error> {
        todo!()
    }

    // Used for reprocessing blocks when creating a data source.
    #[allow(unused)]
    async fn triggers_in_block(
        &self,
        logger: &Logger,
        block: codec::Block,
        filter: &crate::adapter::TriggerFilter,
    ) -> Result<BlockWithTriggers<Chain>, Error> {
        let triggers = vec![StarknetTrigger::Block(Arc::new(block.clone()))];

        Ok(BlockWithTriggers::new(block, triggers, logger))
    }

    /// Return `true` if the block with the given hash and number is on the
    /// main chain, i.e., the chain going back from the current chain head.
    #[allow(unused)]
    async fn is_on_main_chain(&self, ptr: BlockPtr) -> Result<bool, Error> {
        todo!()
    }

    /// Get pointer to parent of `block`. This is called when reverting `block`.
    #[allow(unused)]
    async fn parent_ptr(&self, block: &BlockPtr) -> Result<Option<BlockPtr>, Error> {
        // Panics if `block` is genesis.
        // But that's ok since this is only called when reverting `block`.
        Ok(Some(BlockPtr {
            hash: BlockHash::from(vec![0xff; 32]),
            number: block.number.saturating_sub(1),
        }))
    }
}
