use graph::blockchain::TriggerFilter as TriggerFilterTrait;

use crate::{
    capabilities::NodeCapabilities,
    data_source::{DataSource, DataSourceTemplate},
    Chain,
};

#[derive(Default, Clone)]
pub struct TriggerFilter;

impl TriggerFilterTrait<Chain> for TriggerFilter {
    #[allow(unused)]
    fn extend_with_template(&mut self, data_source: impl Iterator<Item = DataSourceTemplate>) {
        todo!()
    }

    #[allow(unused)]
    fn extend<'a>(&mut self, data_sources: impl Iterator<Item = &'a DataSource> + Clone) {
        todo!()
    }

    fn node_capabilities(&self) -> NodeCapabilities {
        todo!()
    }

    fn to_firehose_filter(self) -> Vec<prost_types::Any> {
        todo!()
    }
}
