use crate::codec::FlowCodec;
use crate::messages::Packet;
use async_trait::async_trait;
use futures::stream::StreamExt;
use futures::SinkExt;
use packline_core::app::App;
use packline_core::connector::{TCPConnectionHandler, TCPConnectorHandler};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::runtime::Handle;
use tokio::sync::Mutex;
use tokio_util::codec::Framed;

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
    async fn handle(&mut self) -> Result<(), std::io::Error> {
        println!("New Flow Connection: {}", self.addr);

        let mut framed = Framed::new(std::mem::replace(&mut self.stream, None).unwrap(), FlowCodec::new());
        let (mut sink, mut stream) = framed.split();

        let rc_sink = Arc::new(Mutex::new(sink));

        loop {
            let packet = stream.next().await;
            match packet {
                None => break,
                Some(r) => {
                    let mut sink = rc_sink.clone();
                    Handle::current().spawn(async move {
                        let packet = FlowConnectionHandler::handle_packet(r.unwrap()).unwrap();
                        {
                            let mut sink = sink.lock().await;
                            let _ = sink.send(packet).await;
                        }
                    });
                }
            }
        }

        println!("Flow connection finished");
        Ok(())
    }
}

impl FlowConnectionHandler {
    fn handle_packet(packet: Packet) -> Result<Packet, std::io::Error> {
        Ok(packet)
    }
}
