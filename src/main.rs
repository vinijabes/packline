use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::net::{TcpListener, TcpStream};
use tokio_util::codec::{BytesCodec, Framed};

use futures::StreamExt;

use flow::FlowSchema;

#[derive(FlowSchema)]
pub struct SchemaTest {
    x: u32,
    y: u32,
    z: u32,
    w: u32,
}

#[tokio::main]
async fn main() {
    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 1883); // localhost:1883
    let listener = TcpListener::bind(address).await.unwrap();

    loop {
        let (stream, addr) = listener.accept().await.unwrap();
        println!("New connection: {}", addr);
        tokio::spawn(async move {
            handle_client(stream).await;
        });
    }
}

async fn handle_client(stream: TcpStream) {
    let mut framed = Framed::new(stream, BytesCodec::new());
    println!("New client thread spawned");

    let packet = framed.next().await;
    println!("{:#?}", packet);
}
