use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};
use packline_flow::codec::FlowCodec;
use packline_flow::messages::{Message, Packet, RouteWithVersion};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::oneshot::{channel, Sender};
use tokio::sync::Mutex;
use tokio_util::codec::Framed;

pub struct Connection {
    requests: Arc<Mutex<HashMap<u32, Sender<Packet>>>>,
    sink: SplitSink<Framed<TcpStream, FlowCodec>, Packet>,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        let (sink, mut stream) = Framed::new(stream, FlowCodec::new()).split();
        let requests: Arc<Mutex<HashMap<u32, Sender<Packet>>>> = Arc::new(Mutex::new(HashMap::new()));

        let clone = requests.clone();
        tokio::spawn(async move {
            loop {
                let packet = stream.next().await;
                match packet {
                    None => break,
                    Some(r) => {
                        let packet = r.unwrap();

                        let mut request_table = clone.lock().await;
                        let sender = request_table.remove(&packet.context_id);

                        let _ = sender.unwrap().send(packet);
                    }
                }
            }
        });

        Connection { requests, sink }
    }

    pub async fn send(&mut self, route: RouteWithVersion, message: Message) -> Result<Message, std::io::Error> {
        let (tx, rx) = channel::<Packet>();

        let packet = Packet::new(route, message);
        {
            let mut requests_table = self.requests.lock().await;
            requests_table.insert(packet.context_id, tx);
        }

        let _ = self.sink.send(packet).await;

        rx.await
            .map(|packet| packet.message)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "oh no!"))
    }
}
