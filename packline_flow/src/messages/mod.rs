pub mod connect;

#[derive(Debug)]
pub enum Message {
    ConnectRequestV1(connect::ConnectRequestV1),
}

pub type Route = u16;
pub type RouteVersion = u16;

pub type RouteWithVersion = (Route, RouteVersion);

pub struct Packet {
    pub route: RouteWithVersion,
    pub message: Message,
}
