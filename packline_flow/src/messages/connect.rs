use crate::{FlowDeserializable, FlowSerializable, FlowSized};
use std::fmt::Display;

pub mod flow {
    pub use crate::flow::*;
}

#[derive(FlowDeserializable, FlowSerializable, FlowSized, Debug)]
pub struct ConnectRequestV1 {
    pub x: i64,
}

impl Display for ConnectRequestV1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.x)
    }
}
