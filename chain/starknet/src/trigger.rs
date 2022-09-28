use graph::blockchain::TriggerData;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum StarknetTrigger {}

impl TriggerData for StarknetTrigger {
    fn error_context(&self) -> String {
        todo!()
    }
}
