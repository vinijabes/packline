use tokio::net::{TcpStream, ToSocketAddrs};

use packline_flow::messages::connect::ConnectRequestV1;
use packline_flow::messages::Message;

use crate::connection::Connection;

use packline_flow::messages::subscribe::SubscribeTopicRequestV1;
use tokio::sync::mpsc::Sender;

pub struct Client {
    connection: Connection,

    #[allow(dead_code)]
    consumers: Vec<Sender<bool>>,
}

pub async fn connect<T: ToSocketAddrs>(addr: T) -> Result<Client, Box<dyn std::error::Error>> {
    let socket = TcpStream::connect(addr).await?;

    let mut connection = Connection::new(socket);

    connection
        .send((1, 1), Message::ConnectRequestV1(ConnectRequestV1 {}))
        .await?;

    Ok(Client {
        connection,
        consumers: Vec::new(),
    })
}

impl Client {
    #[allow(clippy::unused_unit)]
    pub async fn consume<F>(&mut self, topic: String, handler: F)
    where
        F: Fn(u32) -> () + Send + 'static,
    {
        let mut stream = self
            .connection
            .open_stream(
                (2, 1),
                Message::SubscribeTopicRequestV1(SubscribeTopicRequestV1 {
                    topic: topic.clone(),
                    consumer_group_id: "".to_string(),
                }),
            )
            .await
            .unwrap();

        tokio::spawn(async move {
            while let Some(message) = stream.recv().await {
                if let Message::ConsumeV1(c) = message {
                    for record in c.records {
                        handler(record);
                    }
                }
            }
        });
    }
}
