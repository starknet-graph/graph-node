use graph::{
    anyhow::{anyhow, Error},
    blockchain::{self, Block as BlockchainBlock, TriggerWithHandler},
    components::{link_resolver::LinkResolver, store::StoredDynamicDataSource},
    data::subgraph::DataSourceContext,
    prelude::{async_trait, BlockNumber, DataSourceTemplateInfo, Deserialize, Link, Logger},
    semver,
};
use serde::de;
use starknet::core::utils::get_selector_from_name;
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
    pub event_handlers: Vec<MappingEventHandler>,
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
    #[serde(default, deserialize_with = "deserialize_address")]
    pub address: Option<Vec<u8>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnresolvedMapping {
    #[serde(default)]
    pub block_handlers: Vec<MappingBlockHandler>,
    #[serde(default)]
    pub event_handlers: Vec<MappingEventHandler>,
    pub file: Link,
}

#[derive(Clone, Deserialize)]
pub struct MappingBlockHandler {
    pub handler: String,
}

#[derive(Clone, Deserialize)]
pub struct MappingEventHandler {
    pub handler: String,
    pub event: String,
}

impl MappingEventHandler {
    fn key(&self) -> Vec<u8> {
        get_selector_from_name(self.event.as_str())
            .expect("MappingEventHandler.event is invalid")
            .to_bytes_be()
            .to_vec()
    }
}

#[derive(Debug, Clone)]
pub struct DataSourceTemplate;

#[derive(Clone, Default, Deserialize)]
pub struct UnresolvedDataSourceTemplate;

impl blockchain::DataSource<Chain> for DataSource {
    fn from_template_info(_template_info: DataSourceTemplateInfo<Chain>) -> Result<Self, Error> {
        Err(anyhow!("StarkNet subgraphs do not support templates"))
    }

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
        if self.start_block() > block.number() {
            return Ok(None);
        }

        let handler = match trigger {
            StarknetTrigger::Block(_) => match self.mapping.block_handlers.first() {
                Some(handler) => handler.handler.clone(),
                None => return Ok(None),
            },
            StarknetTrigger::Event(event) => match self.handler_for_event(event) {
                Some(handler) => handler.handler,
                None => return Ok(None),
            },
        };

        println!("Handler found: {}", handler);

        Ok(Some(TriggerWithHandler::<Chain>::new(
            trigger.clone(),
            handler,
            block.ptr(),
        )))
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

impl DataSource {
    /// Returns event trigger if an event.key matches the handler.key and optionally
    /// if event.fromAddr matches the source address. Note this only supports the default
    /// starknet behavior of one key per event.
    fn handler_for_event(&self, event: &codec::Event) -> Option<MappingEventHandler> {
        return self
            .mapping
            .event_handlers
            .iter()
            .find(|handler| match event.keys.first() {
                Some(key) => match &self.source.address {
                    Some(address) => address == &event.from_addr && key == &handler.key(),
                    None => key == &handler.key(),
                },
                None => false,
            })
            .cloned();
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
                event_handlers: self.mapping.event_handlers,
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

fn deserialize_address<'de, D>(deserializer: D) -> Result<Option<Vec<u8>>, D::Error>
where
    D: de::Deserializer<'de>,
{
    use serde::de::Error;

    let s: String = de::Deserialize::deserialize(deserializer)?;
    let address = s.trim_start_matches("0x");
    hex::decode(address).map_err(D::Error::custom).map(Some)
}
