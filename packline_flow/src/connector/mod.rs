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
use crate::messages::consume::ConsumeV1;
use crate::messages::produce::ProduceV1Response;
use crate::messages::Message;
use crate::messages::Packet;

pub struct FlowConnector {
    pub app: App,
}

pub struct FlowConnectionHandler {
    app: App,
    addr: SocketAddr,
    stream: Option<TcpStream>,
}

#[async_trait]
impl TCPConnectorHandler for FlowConnector {
    fn handle_connection(&self, conn: (TcpStream, SocketAddr)) -> Box<dyn TCPConnectionHandler> {
        Box::new(FlowConnectionHandler {
            app: self.app.clone(),
            addr: conn.1,
            stream: Some(conn.0),
        })
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
struct ConnectionState {
    sink: Mutex<SplitSink<Framed<TcpStream, FlowCodec>, Packet>>,
}

#[async_trait]
impl TCPConnectionHandler for FlowConnectionHandler {
    async fn handle(&mut self) -> Result<(), std::io::Error> {
        let handle = Handle::current();
        debug!("New Flow Connection: {}", self.addr);

        let framed = Framed::new(std::mem::replace(&mut self.stream, None).unwrap(), FlowCodec::new());
        let (sink, mut stream) = framed.split();

        let rc_state = Arc::new(ConnectionState { sink: Mutex::new(sink) });

        handle.spawn(async {
            debug!("Starting connection stream handler");
        });

        loop {
            let packet = stream.next().await;
            debug!(?packet);
            match packet {
                None => break,
                Some(r) => {
                    let state = rc_state.clone();
                    let packet = self.handle_packet(state.clone(), r.unwrap()).await.unwrap();
                    if let Some(packet) = packet {
                        let mut sink = state.sink.lock().await;
                        let result = sink.send(packet).await;

                        debug!("Wrote to stream; success={:?}", result.is_ok());
                    }
                }
            }
        }

        debug!("Flow connection finished");
        Ok(())
    }
}

impl FlowConnectionHandler {
    async fn handle_packet(
        &self,
        state: Arc<ConnectionState>,
        packet: Packet,
    ) -> Result<Option<Packet>, std::io::Error> {
        info!("handling packet {:?}", &packet.message);
        match &packet.message {
            Message::SubscribeTopicRequestV1(subscribe) => {
                self.handle_subscribe_topic_request(state, packet.context_id, subscribe.clone());
                Ok(None)
            }
            Message::ProduceV1(produce) => Ok(Some(
                self.handle_produce_request(state, packet.context_id, produce.clone())
                    .await,
            )),
            _ => Ok(Some(packet)),
        }
    }

    async fn handle_produce_request(
        &self,
        _: Arc<ConnectionState>,
        context_id: u32,
        mut produce: super::messages::produce::ProduceV1,
    ) -> Packet {
        let app = self.app.clone();

        let topic = produce.topic.to_string();
        let channel = app.get_channel(&(topic, 1u16)).await;

        if let Some(channel) = channel {
            let mut producer = channel.producer();
            producer.produce(&mut produce.records).await;
        }

        Packet::new_with_context_id(context_id, (5, 1), Message::ProduceV1Response(ProduceV1Response {}))
    }

    fn handle_subscribe_topic_request(
        &self,
        state: Arc<ConnectionState>,
        context_id: u32,
        subscribe: super::messages::subscribe::SubscribeTopicRequestV1,
    ) {
        let handle = Handle::current();

        let app = self.app.clone();
        handle.spawn(async move {
            let topic = subscribe.topic.to_string();
            let channel = app.get_channel(&(topic.clone(), 1u16)).await;

            if let Some(channel) = channel {
                info!("Starting consuming from channel {:?}", &topic);
                let consumer = channel.consumer(0);

                loop {
                    let records = consumer.consume().await;

                    {
                        let mut guard = state.sink.lock().await;

                        let packet = Packet::new_stream_packet(
                            context_id,
                            (3, 1),
                            Message::ConsumeV1(ConsumeV1 {
                                topic: topic.clone(),
                                records,
                            }),
                        );

                        let _ = guard.send(packet).await;
                    }
                }
            }
        });
    }
}
