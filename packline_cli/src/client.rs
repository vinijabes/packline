use tokio::net::{TcpStream, ToSocketAddrs};

use packline_flow::messages::connect::ConnectRequestV1;
use packline_flow::messages::Message;

use crate::connection::Connection;

use tokio::runtime::Handle;
use tokio::sync::mpsc::channel;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;

pub struct Client {
    connection: Connection,

    consumer_worker: Sender<(String, Box<Fn() -> () + Send>)>,
    consumers: Vec<Sender<bool>>,
}

pub async fn connect<T: ToSocketAddrs>(addr: T) -> Result<Client, Box<dyn std::error::Error>> {
    let socket = TcpStream::connect(addr).await?;

    let mut connection = Connection::new(socket);

    connection
        .send((1, 1), Message::ConnectRequestV1(ConnectRequestV1 {}))
        .await?;

    let (tx, mut rx) = channel(16);

    Handle::current().spawn(Client::consumer_worker(rx));

    Ok(Client {
        connection,
        consumer_worker: tx,
        consumers: Vec::new(),
    })
}

impl Client {
    pub async fn consume<F>(&mut self, topic: String, handler: F)
    where
        F: Fn() -> () + Send + 'static,
    {
        self.consumer_worker.send((topic, Box::new(handler))).await;
    }

    async fn consumer_worker<F>(mut rx: Receiver<(String, F)>)
    where
        F: Fn() -> (),
    {
        println!("Starting consumer worker {:?}", rx);

        while let Some((topic, handler)) = rx.recv().await {
            println!("{}", topic);

            //TODO: start a stream between client and server.
        }

        println!("Stopping consumer worker {:?}", rx);
    }
}
