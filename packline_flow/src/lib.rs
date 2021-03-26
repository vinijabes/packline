use std::net::SocketAddr;

use async_trait::async_trait;
pub use flow_derive::*;
use packline_core::{app::App};
use packline_core::connector::TCPConnectorHandler;
pub use schema::{DeserializableSchema, Schema, SerializableSchema, SizedSchema};
use tokio::net::{TcpStream};

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

#[async_trait]
impl<'a> TCPConnectorHandler for FlowConnector<'a> {
    async fn handle_connection(&self, _conn: (TcpStream, SocketAddr)) {
        println!("New Flow Connection");
    }
}
