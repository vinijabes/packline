use tokio::net::{TcpStream, ToSocketAddrs};

use packline_flow::messages::connect::ConnectRequestV1;
use packline_flow::messages::Message;

use crate::connection::Connection;

use log::debug;
use packline_flow::messages::subscribe::SubscribeTopicRequestV1;
use tokio::runtime::Handle;
use tokio::sync::mpsc::channel;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;

pub struct Client {
    connection: Connection,

    #[allow(clippy::unused_unit)]
    consumer_worker: Sender<(String, Box<dyn Fn() -> () + Send>)>,
    #[allow(dead_code)]
    consumers: Vec<Sender<bool>>,
}

pub async fn connect<T: ToSocketAddrs>(addr: T) -> Result<Client, Box<dyn std::error::Error>> {
    let socket = TcpStream::connect(addr).await?;

    let mut connection = Connection::new(socket);

    connection
        .send((1, 1), Message::ConnectRequestV1(ConnectRequestV1 {}))
        .await?;

    let (tx, rx) = channel(16);

    Handle::current().spawn(Client::consumer_worker(rx));

    Ok(Client {
        connection,
        consumer_worker: tx,
        consumers: Vec::new(),
    })
}

impl Client {
    #[allow(clippy::unused_unit)]
    pub async fn consume<F>(&mut self, topic: String, handler: F)
    where
        F: Fn() -> () + Send + 'static,
    {
        let _ = self
            .connection
            .send(
                (2, 1),
                Message::SubscribeTopicRequestV1(SubscribeTopicRequestV1 {
                    topic: topic.clone(),
                    consumer_group_id: "".to_string(),
                }),
            )
            .await;

        let _ = self.consumer_worker.send((topic, Box::new(handler))).await;
    }

    #[allow(clippy::unused_unit)]
    async fn consumer_worker<F>(mut rx: Receiver<(String, F)>)
    where
        F: Fn() -> (),
    {
        debug!("Starting consumer worker {:?}", rx);

        while let Some((topic, _)) = rx.recv().await {
            debug!("{}", topic);

            //TODO: start a stream between client and server.
        }

        debug!("Stopping consumer worker {:?}", rx);
    }
}
