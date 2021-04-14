use crate::{DeserializableSchema, SerializableSchema, SizedSchema};
use bytes::BytesMut;
use rand::random;
use std::convert::Infallible;

pub mod connect;
pub mod subscribe;

#[derive(Debug)]
pub enum Message {
    ConnectRequestV1(connect::ConnectRequestV1),
    SubscribeTopicRequestV1(subscribe::SubscribeTopicRequestV1),
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

impl SerializableSchema for Message {
    type Error = Infallible;

    fn serialize(&self, encoder: &mut BytesMut) {
        match self {
            Message::ConnectRequestV1(m) => m.serialize(encoder),
            Message::SubscribeTopicRequestV1(m) => m.serialize(encoder),
            _ => (),
        };
    }
}

pub type Route = u16;
pub type RouteVersion = u16;

pub type RouteWithVersion = (Route, RouteVersion);

#[derive(Debug)]
pub struct Packet {
    route: RouteWithVersion,
    pub context_id: u32,
    pub message: Message,
}

impl Packet {
    pub fn new(route: RouteWithVersion, message: Message) -> Packet {
        Packet {
            route,
            context_id: random::<u32>(),
            message,
        }
    }
}

impl SizedSchema for Packet {
    fn size(&self) -> usize {
        self.route.0.size() + self.route.1.size() + self.message.size()
    }
}

impl DeserializableSchema for Packet {
    type Error = Infallible;
    type Item = Option<Packet>;

    fn deserialize(decoder: &mut crate::codec::decoder::ByteDecoder) -> Result<Option<Packet>, Self::Error> {
        if decoder.len() < 4 {
            return Ok(None);
        }

        let size = i32::deserialize(decoder)? + 4;

        if decoder.len() < (size as usize) {
            return Ok(None);
        }

        let route = u16::deserialize(decoder)?;
        let version = u16::deserialize(decoder)?;
        let request_id = u32::deserialize(decoder)?;

        let message = match (route, version) {
            (1, 1) => Message::ConnectRequestV1(connect::ConnectRequestV1::deserialize(decoder).unwrap()),
            (2, 1) => {
                Message::SubscribeTopicRequestV1(subscribe::SubscribeTopicRequestV1::deserialize(decoder).unwrap())
            }
            _ => Message::Invalid,
        };

        Ok(Some(Packet {
            route: (route, version),
            context_id: request_id,
            message,
        }))
    }
}

impl SerializableSchema for Packet {
    type Error = Infallible;

    fn serialize(&self, encoder: &mut BytesMut) {
        (self.size() as i32).serialize(encoder);

        self.route.0.serialize(encoder);
        self.route.1.serialize(encoder);
        self.context_id.serialize(encoder);

        self.message.serialize(encoder);
    }
}
