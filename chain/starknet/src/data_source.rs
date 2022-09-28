use graph::{
    anyhow::Error,
    blockchain::{self, Blockchain, TriggerWithHandler},
    components::{link_resolver::LinkResolver, store::StoredDynamicDataSource},
    data::subgraph::DataSourceContext,
    prelude::{async_trait, BlockNumber, DataSourceTemplateInfo, Deserialize, Logger},
    semver,
};
use std::sync::Arc;

use crate::chain::Chain;

#[derive(Clone)]
pub struct DataSource;

#[derive(Deserialize)]
pub struct UnresolvedDataSource;

#[derive(Debug, Clone)]
pub struct DataSourceTemplate;

#[derive(Clone, Default, Deserialize)]
pub struct UnresolvedDataSourceTemplate;

impl blockchain::DataSource<Chain> for DataSource {
    fn address(&self) -> Option<&[u8]> {
        todo!()
    }

    fn start_block(&self) -> BlockNumber {
        todo!()
    }

    #[allow(unused)]
    fn match_and_decode(
        &self,
        trigger: &<Chain as Blockchain>::TriggerData,
        block: &Arc<<Chain as Blockchain>::Block>,
        logger: &Logger,
    ) -> Result<Option<TriggerWithHandler<Chain>>, Error> {
        todo!()
    }

    fn name(&self) -> &str {
        todo!()
    }

    fn kind(&self) -> &str {
        todo!()
    }

    fn network(&self) -> Option<&str> {
        todo!()
    }

    fn context(&self) -> Arc<Option<DataSourceContext>> {
        todo!()
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
        todo!()
    }

    fn api_version(&self) -> semver::Version {
        todo!()
    }

    fn runtime(&self) -> Option<Arc<Vec<u8>>> {
        todo!()
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
        todo!()
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
