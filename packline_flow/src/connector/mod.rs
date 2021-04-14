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
use tracing::{debug, info};

use packline_core::app::App;
use packline_core::connector::{TCPConnectionHandler, TCPConnectorHandler};

use crate::codec::FlowCodec;
use crate::messages::Packet;

pub struct FlowConnector<'a> {
    pub app: &'a App,
}

#[cfg_attr(debug_assertions, derive(Debug))]
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

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct ConnectionState {
    sink: Mutex<SplitSink<Framed<TcpStream, FlowCodec>, Packet>>,
}

#[async_trait]
impl TCPConnectionHandler for FlowConnectionHandler {
    #[cfg_attr(debug_assertions, tracing::instrument)]
    async fn handle(&mut self) -> Result<(), std::io::Error> {
        let handle = Handle::current();
        debug!("New Flow Connection: {}", self.addr);

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
                            let result = sink.send(packet).await;

                            debug!("Wrote to stream; success={:?}", result.is_ok());
                        }
                    });
                }
            }
        }

        debug!("Flow connection finished");
        Ok(())
    }
}

impl FlowConnectionHandler {
    #[cfg_attr(debug_assertions, tracing::instrument)]
    fn handle_packet(state: Arc<ConnectionState>, packet: Packet) -> Result<Packet, std::io::Error> {
        use super::messages::Message;

        info!("handling packet");
        match &packet.message {
            Message::SubscribeTopicRequestV1(c) => Ok(packet),
            _ => Ok(packet),
        }
    }
}
