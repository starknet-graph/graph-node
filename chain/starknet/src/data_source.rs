use graph::{
    anyhow::Error,
    blockchain::{self, Block as BlockchainBlock, TriggerWithHandler},
    components::{link_resolver::LinkResolver, store::StoredDynamicDataSource},
    data::subgraph::DataSourceContext,
    prelude::{async_trait, BlockNumber, DataSourceTemplateInfo, Deserialize, Link, Logger},
    semver,
};
use std::sync::Arc;

use crate::{chain::Chain, codec, trigger::StarknetTrigger};

#[derive(Clone)]
pub struct DataSource {
    pub kind: String,
    pub network: String,
    pub name: String,
    pub source: Source,
    pub mapping: Mapping,
}

#[derive(Clone)]
pub struct Mapping {
    pub block_handlers: Vec<MappingBlockHandler>,
    pub runtime: Arc<Vec<u8>>,
}

#[derive(Deserialize)]
pub struct UnresolvedDataSource {
    pub kind: String,
    pub network: String,
    pub name: String,
    pub source: Source,
    pub mapping: UnresolvedMapping,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    pub start_block: BlockNumber,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnresolvedMapping {
    #[serde(default)]
    pub block_handlers: Vec<MappingBlockHandler>,
    pub file: Link,
}

#[derive(Clone, Deserialize)]
pub struct MappingBlockHandler {
    pub handler: String,
}

#[derive(Debug, Clone)]
pub struct DataSourceTemplate;

#[derive(Clone, Default, Deserialize)]
pub struct UnresolvedDataSourceTemplate;

impl blockchain::DataSource<Chain> for DataSource {
    fn address(&self) -> Option<&[u8]> {
        None
    }

    fn start_block(&self) -> BlockNumber {
        self.source.start_block
    }

    #[allow(unused)]
    fn match_and_decode(
        &self,
        trigger: &StarknetTrigger,
        block: &Arc<codec::Block>,
        logger: &Logger,
    ) -> Result<Option<TriggerWithHandler<Chain>>, Error> {
        if self.mapping.block_handlers.is_empty() {
            Ok(None)
        } else {
            let handler = &self.mapping.block_handlers[0];

            println!("Handler found: {}", handler.handler);

            Ok(Some(TriggerWithHandler::<Chain>::new(
                trigger.clone(),
                handler.handler.clone(),
                block.ptr(),
            )))
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn kind(&self) -> &str {
        &self.kind
    }

    fn network(&self) -> Option<&str> {
        Some(&self.network)
    }

    fn context(&self) -> Arc<Option<DataSourceContext>> {
        Arc::new(None)
    }

    fn creation_block(&self) -> Option<BlockNumber> {
        todo!()
    }

    #[allow(unused)]
    fn is_duplicate_of(&self, other: &Self) -> bool {
        todo!()
    }

    fn as_stored_dynamic_data_source(&self) -> StoredDynamicDataSource {
        todo!()
    }

    #[allow(unused)]
    fn from_stored_dynamic_data_source(
        template: &DataSourceTemplate,
        stored: StoredDynamicDataSource,
    ) -> Result<Self, Error> {
        todo!()
    }

    fn validate(&self) -> Vec<Error> {
        Default::default()
    }

    fn api_version(&self) -> semver::Version {
        semver::Version::new(0, 0, 5)
    }

    fn runtime(&self) -> Option<Arc<Vec<u8>>> {
        Some(self.mapping.runtime.clone())
    }
}

impl TryFrom<DataSourceTemplateInfo<Chain>> for DataSource {
    type Error = Error;

    #[allow(unused)]
    fn try_from(value: DataSourceTemplateInfo<Chain>) -> Result<Self, Self::Error> {
        todo!()
    }
}

#[async_trait]
impl blockchain::UnresolvedDataSource<Chain> for UnresolvedDataSource {
    #[allow(unused)]
    async fn resolve(
        self,
        resolver: &Arc<dyn LinkResolver>,
        logger: &Logger,
        manifest_idx: u32,
    ) -> Result<DataSource, Error> {
        let module_bytes = resolver.cat(logger, &self.mapping.file).await?;

        Ok(DataSource {
            kind: self.kind,
            network: self.network,
            name: self.name,
            source: self.source,
            mapping: Mapping {
                block_handlers: self.mapping.block_handlers,
                runtime: Arc::new(module_bytes),
            },
        })
    }
}

impl blockchain::DataSourceTemplate<Chain> for DataSourceTemplate {
    fn api_version(&self) -> semver::Version {
        todo!()
    }

    fn runtime(&self) -> Option<Arc<Vec<u8>>> {
        todo!()
    }

    fn name(&self) -> &str {
        todo!()
    }

    fn manifest_idx(&self) -> u32 {
        todo!()
    }
}

#[async_trait]
impl blockchain::UnresolvedDataSourceTemplate<Chain> for UnresolvedDataSourceTemplate {
    #[allow(unused)]
    async fn resolve(
        self,
        resolver: &Arc<dyn LinkResolver>,
        logger: &Logger,
        manifest_idx: u32,
    ) -> Result<DataSourceTemplate, Error> {
        todo!()
    }
}
