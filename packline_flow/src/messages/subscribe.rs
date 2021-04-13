use crate::{FlowDeserializable, FlowSerializable, FlowSized};

pub mod flow {
    pub use crate::codec;
    pub use crate::flow::*;
}

#[derive(FlowDeserializable, FlowSerializable, FlowSized)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct SubscribeTopicV1 {
    pub topic: String,
    pub consumer_group_id: String,
}
