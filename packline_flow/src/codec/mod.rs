pub mod decoder;
pub mod encoder;

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct FlowCodec;

impl FlowCodec {
    pub fn new() -> FlowCodec {
        FlowCodec {}
    }
}
