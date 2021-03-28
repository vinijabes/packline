use crate::{SerializableSchema, SizedSchema};
use bytes::BytesMut;
use std::convert::Infallible;

pub mod connect;

#[derive(Debug)]
pub enum Message {
    ConnectRequestV1(connect::ConnectRequestV1),
}

impl SizedSchema for Message {
    fn size(&self) -> usize {
        match self {
            Message::ConnectRequestV1(m) => m.size(),
        }
    }
}

impl SerializableSchema for Message {
    type Error = Infallible;

    fn serialize(&self, encoder: &mut BytesMut) {
        match self {
            Message::ConnectRequestV1(m) => m.serialize(encoder),
        };
    }
}

pub type Route = u16;
pub type RouteVersion = u16;

pub type RouteWithVersion = (Route, RouteVersion);

pub struct Packet {
    pub route: RouteWithVersion,
    pub message: Message,
}
