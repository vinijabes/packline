use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};
use log::debug;
use packline_flow::codec::FlowCodec;
use packline_flow::messages::{Message, Packet, PacketType, RouteWithVersion};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::oneshot::{channel as oneshot_channel, Sender as OneshotSender};
use tokio::sync::Mutex;
use tokio_util::codec::Framed;

pub struct Connection {
    requests: Arc<Mutex<HashMap<u32, OneshotSender<Packet>>>>,
    streams: Arc<Mutex<HashMap<u32, Sender<Message>>>>,
    sink: SplitSink<Framed<TcpStream, FlowCodec>, Packet>,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        let (sink, mut stream) = Framed::new(stream, FlowCodec::new()).split();
        let requests: Arc<Mutex<HashMap<u32, OneshotSender<Packet>>>> = Arc::new(Mutex::new(HashMap::new()));
        let streams: Arc<Mutex<HashMap<u32, Sender<Message>>>> = Arc::new(Mutex::new(HashMap::new()));

        let clone = requests.clone();
        let streams_clone = streams.clone();
        tokio::spawn(async move {
            loop {
                let packet = stream.next().await;
                match packet {
                    None => break,
                    Some(r) => {
                        let packet = r.unwrap();

                        match packet.packet_type {
                            PacketType::Stream => {
                                let stream_table = streams_clone.lock().await;
                                let sender = stream_table.get(&packet.context_id);

                                let _ = sender.unwrap().send(packet.message).await;
                            }
                            PacketType::Request => {
                                let mut request_table = clone.lock().await;
                                let sender = request_table.remove(&packet.context_id);

                                debug!("received response {:?}", &packet);
                                let _ = sender.unwrap().send(packet);
                            }
                        }
                    }
                }
            }
        });

        Connection {
            streams,
            requests,
            sink,
        }
    }

    pub async fn send(&mut self, route: RouteWithVersion, message: Message) -> Result<Message, std::io::Error> {
        let (tx, rx) = oneshot_channel::<Packet>();

        let packet = Packet::new(route, message);
        {
            let mut requests_table = self.requests.lock().await;
            requests_table.insert(packet.context_id, tx);
        }

        self.sink.send(packet).await?;

        rx.await
            .map(|packet| packet.message)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "oh no!"))
    }

    pub async fn open_stream(
        &mut self,
        route: RouteWithVersion,
        message: Message,
    ) -> Result<Receiver<Message>, std::io::Error> {
        let (tx, rx) = channel::<Message>(16);

        let packet = Packet::new(route, message);
        {
            let mut streams_table = self.streams.lock().await;
            streams_table.insert(packet.context_id, tx);
        }

        self.sink.send(packet).await?;

        Ok(rx)
    }
}
