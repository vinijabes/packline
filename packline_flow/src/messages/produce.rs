use crate::{FlowDeserializable, FlowSerializable, FlowSized};

pub mod flow {
    pub use crate::codec;
    pub use crate::flow::*;
}

#[derive(FlowDeserializable, FlowSerializable, FlowSized, Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct ProduceV1 {
    pub topic: String,

    #[rustfmt::skip]
    pub records: Vec::<u32>,
}

#[derive(FlowDeserializable, FlowSerializable, FlowSized, Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct ProduceV1Response {}
