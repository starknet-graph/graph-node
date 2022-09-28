use graph::blockchain::NodeCapabilities as NodeCapabilitiesTrait;
use std::fmt::Display;

use crate::{data_source::DataSource, Chain};

pub struct NodeCapabilities;

impl NodeCapabilitiesTrait<Chain> for NodeCapabilities {
    #[allow(unused)]
    fn from_data_sources(data_sources: &[DataSource]) -> Self {
        todo!()
    }
}

impl Display for NodeCapabilities {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "starknet")
    }
}
