use crate::{FlowDeserializable, FlowSerializable, FlowSized};

pub mod flow {
    pub use crate::flow::*;
}

#[derive(FlowDeserializable, FlowSerializable, FlowSized)]
pub struct ConnectRequestV1 {
    pub x: i64,
}
