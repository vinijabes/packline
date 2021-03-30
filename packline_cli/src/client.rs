use crate::connection::Connection;
use packline_flow::messages::connect::ConnectRequestV1;
use packline_flow::messages::Message;
use tokio::net::{TcpStream, ToSocketAddrs};

pub struct Client {
    connection: Connection,
}

pub async fn connect<T: ToSocketAddrs>(addr: T) -> Result<Client, Box<dyn std::error::Error>> {
    let socket = TcpStream::connect(addr).await?;

    let mut connection = Connection::new(socket);

    connection
        .send((1, 1), Message::ConnectRequestV1(ConnectRequestV1 {}))
        .await?;

    Ok(Client { connection })
}
