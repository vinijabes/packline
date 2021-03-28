use crate::{FlowDeserializable, FlowSerializable, FlowSized};

pub mod flow {
    pub use crate::codec;
    pub use crate::flow::*;
}

#[derive(FlowDeserializable, FlowSerializable, FlowSized)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct ConnectRequestV1 {}
