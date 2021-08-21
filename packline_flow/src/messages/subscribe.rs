use crate::{FlowDeserializable, FlowSerializable, FlowSized};

pub mod flow {
    pub use crate::codec;
    pub use crate::flow::*;
}

#[derive(FlowDeserializable, FlowSerializable, FlowSized, Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct SubscribeTopicRequestV1 {
    pub topic: String,
    pub consumer_group_id: String,
}
