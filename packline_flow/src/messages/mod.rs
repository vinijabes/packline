use crate::{DeserializableSchema, SerializableSchema, SizedSchema};
use bytes::BytesMut;
use std::convert::Infallible;

pub mod connect;

#[derive(Debug)]
pub enum Message {
    ConnectRequestV1(connect::ConnectRequestV1),
    Invalid,
}

impl SizedSchema for Message {
    fn size(&self) -> usize {
        match self {
            Message::ConnectRequestV1(m) => m.size(),
            _ => 0,
        }
    }
}

impl DeserializableSchema for Message {
    type Error = Infallible;
    type Item = Option<Message>;

    fn deserialize(decoder: &mut crate::codec::decoder::ByteDecoder) -> Result<Self::Item, Self::Error> {
        if decoder.len() < 4 {
            return Ok(None);
        }

        let size = i32::deserialize(decoder)?;

        if decoder.len() < (size as usize) {
            return Ok(None);
        }

        let route = i16::deserialize(decoder)?;
        let version = i16::deserialize(decoder)?;

        let result = match (route, version) {
            (1, 1) => Message::ConnectRequestV1(connect::ConnectRequestV1::deserialize(decoder).unwrap()),
            _ => Message::Invalid,
        };

        Ok(Some(result))
    }
}

impl SerializableSchema for Message {
    type Error = Infallible;

    fn serialize(&self, encoder: &mut BytesMut) {
        match self {
            Message::ConnectRequestV1(m) => m.serialize(encoder),
            _ => (),
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
