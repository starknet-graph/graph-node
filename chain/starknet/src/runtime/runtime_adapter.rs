use graph::{
    anyhow::Error,
    blockchain::{HostFn, RuntimeAdapter as RuntimeAdapterTrait},
};

use crate::{data_source::DataSource, Chain};

pub struct RuntimeAdapter;

impl RuntimeAdapterTrait<Chain> for RuntimeAdapter {
    fn host_fns(&self, _ds: &DataSource) -> Result<Vec<HostFn>, Error> {
        Ok(vec![])
    }
}
