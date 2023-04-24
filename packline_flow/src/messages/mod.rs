use crate::{DeserializableSchema, SerializableSchema, SizedSchema};
use bytes::BytesMut;
use rand::random;
use std::convert::Infallible;

pub mod connect;
pub mod consume;
pub mod produce;
pub mod subscribe;

#[derive(Debug, Clone)]
pub enum Message {
    ConnectRequestV1(connect::ConnectRequestV1),
    SubscribeTopicRequestV1(subscribe::SubscribeTopicRequestV1),
    ConsumeV1(consume::ConsumeV1),
    ProduceV1(produce::ProduceV1),
    ProduceV1Response(produce::ProduceV1Response),
    Invalid,
}

impl SizedSchema for Message {
    fn size(&self) -> usize {
        match self {
            Message::ConnectRequestV1(m) => m.size(),
            Message::SubscribeTopicRequestV1(s) => s.size(),
            Message::ConsumeV1(c) => c.size(),
            Message::ProduceV1(p) => p.size(),
            Message::ProduceV1Response(p) => p.size(),
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
            Message::ConsumeV1(m) => m.serialize(encoder),
            Message::ProduceV1(m) => m.serialize(encoder),
            Message::ProduceV1Response(m) => m.serialize(encoder),
            _ => (),
        };
    }
}

pub type Route = u16;
pub type RouteVersion = u16;

pub type RouteWithVersion = (Route, RouteVersion);

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum PacketType {
    Request = 0,
    Stream = 1,
}

impl From<u8> for PacketType {
    fn from(packet_type: u8) -> Self {
        unsafe { std::mem::transmute(packet_type) }
    }
}

#[derive(Debug, Clone)]
pub struct Packet {
    pub packet_type: PacketType,
    route: RouteWithVersion,
    pub context_id: u32,
    pub message: Message,
}

impl Packet {
    pub fn new(route: RouteWithVersion, message: Message) -> Packet {
        Packet {
            packet_type: PacketType::Request,
            route,
            context_id: random::<u32>(),
            message,
        }
    }

    pub fn new_with_context_id(context_id: u32, route: RouteWithVersion, message: Message) -> Packet {
        Packet {
            packet_type: PacketType::Request,
            route,
            context_id,
            message,
        }
    }

    pub fn new_stream_packet(context_id: u32, route: RouteWithVersion, message: Message) -> Packet {
        Packet {
            packet_type: PacketType::Stream,
            route,
            context_id,
            message,
        }
    }

    #[must_use]
    pub fn response(&self, route: RouteWithVersion, message: Message) -> Packet {
        Packet {
            packet_type: self.packet_type,
            route,
            context_id: self.context_id,
            message,
        }
    }
}

impl SizedSchema for Packet {
    fn size(&self) -> usize {
        (self.packet_type as u8).size()
            + self.route.0.size()
            + self.route.1.size()
            + self.context_id.size()
            + self.message.size()
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

        let packet_type = PacketType::from(u8::deserialize(decoder)?);
        let route = u16::deserialize(decoder)?;
        let version = u16::deserialize(decoder)?;
        let request_id = u32::deserialize(decoder)?;

        let message = match (route, version) {
            (1, 1) => Message::ConnectRequestV1(connect::ConnectRequestV1::deserialize(decoder).unwrap()),
            (2, 1) => {
                Message::SubscribeTopicRequestV1(subscribe::SubscribeTopicRequestV1::deserialize(decoder).unwrap())
            }
            (3, 1) => Message::ConsumeV1(consume::ConsumeV1::deserialize(decoder).unwrap()),
            (4, 1) => Message::ProduceV1(produce::ProduceV1::deserialize(decoder).unwrap()),
            (5, 1) => Message::ProduceV1Response(produce::ProduceV1Response::deserialize(decoder).unwrap()),
            _ => Message::Invalid,
        };

        Ok(Some(Packet {
            packet_type,
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

        (self.packet_type as u8).serialize(encoder);
        self.route.0.serialize(encoder);
        self.route.1.serialize(encoder);
        self.context_id.serialize(encoder);

        self.message.serialize(encoder);
    }
}
