use std::net::SocketAddr;

use async_trait::async_trait;
pub use flow_derive::*;
use futures::stream::StreamExt;
use packline_core::app::App;
use packline_core::connector::{TCPConnectionHandler, TCPConnectorHandler};
pub use schema::{DeserializableSchema, Schema, SerializableSchema, SizedSchema};
use tokio::net::TcpStream;
use tokio_util::codec::{BytesCodec, Framed};

pub mod messages;
pub mod request;
pub mod response;
pub mod schema;

pub(crate) mod flow {
    pub use crate::schema::{DeserializableSchema, Schema, SerializableSchema, SizedSchema};
}

pub struct FlowConnector<'a> {
    pub app: &'a App,
}

pub struct FlowConnectionHandler {
    addr: SocketAddr,
    stream: Option<TcpStream>,
}

#[async_trait]
impl<'a> TCPConnectorHandler for FlowConnector<'a> {
    fn handle_connection(&self, conn: (TcpStream, SocketAddr)) -> Box<dyn TCPConnectionHandler> {
        Box::new(FlowConnectionHandler {
            addr: conn.1,
            stream: Some(conn.0),
        })
    }
}

#[async_trait]
impl TCPConnectionHandler for FlowConnectionHandler {
    async fn handle(&mut self) {
        println!("New Flow Connection: {}", self.addr);
        let mut framed = Framed::new(std::mem::replace(&mut self.stream, None).unwrap(), BytesCodec::new());

        loop {
            let packet = framed.next().await;
            match packet {
                None => break,
                Some(r) => println!("{:#?}", r),
            }
        }

        println!("Flow connection finished");
    }
}
