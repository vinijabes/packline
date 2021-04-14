use std::net::SocketAddr;
use std::sync::Arc;

use async_trait::async_trait;
use futures::stream::SplitSink;
use futures::stream::StreamExt;
use futures::SinkExt;
use tokio::net::TcpStream;
use tokio::runtime::Handle;
use tokio::sync::Mutex;
use tokio_util::codec::Framed;
use tracing::info;

use packline_core::app::App;
use packline_core::connector::{TCPConnectionHandler, TCPConnectorHandler};

use crate::codec::FlowCodec;
use crate::messages::Packet;

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

pub struct ConnectionState {
    sink: Mutex<SplitSink<Framed<TcpStream, FlowCodec>, Packet>>,
}

#[async_trait]
impl TCPConnectionHandler for FlowConnectionHandler {
    async fn handle(&mut self) -> Result<(), std::io::Error> {
        let handle = Handle::current();
        info!("New Flow Connection: {}", self.addr);

        let mut framed = Framed::new(std::mem::replace(&mut self.stream, None).unwrap(), FlowCodec::new());
        let (mut sink, mut stream) = framed.split();

        let rc_state = Arc::new(ConnectionState { sink: Mutex::new(sink) });

        loop {
            let packet = stream.next().await;
            match packet {
                None => break,
                Some(r) => {
                    let mut state = rc_state.clone();
                    handle.spawn(async move {
                        let packet = FlowConnectionHandler::handle_packet(state.clone(), r.unwrap()).unwrap();
                        {
                            let mut sink = state.sink.lock().await;
                            let _ = sink.send(packet).await;
                        }
                    });
                }
            }
        }

        info!("Flow connection finished");
        Ok(())
    }
}

impl FlowConnectionHandler {
    fn handle_packet(state: Arc<ConnectionState>, packet: Packet) -> Result<Packet, std::io::Error> {
        use super::messages::Message;

        match &packet.message {
            Message::SubscribeTopicRequestV1(c) => Ok(packet),
            _ => Ok(packet),
        }
    }
}
