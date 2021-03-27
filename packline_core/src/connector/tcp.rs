use std::io::Error;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use async_trait::async_trait;
use futures::{future::Fuse, select, FutureExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::runtime::Handle;
use tokio::sync::oneshot::Receiver;

use super::{App, Connector};

#[async_trait]
pub trait TCPConnectorHandler: Send {
    fn handle_connection(&self, conn: (TcpStream, SocketAddr)) -> Box<dyn TCPConnectionHandler>;
}

#[async_trait]
pub trait TCPConnectionHandler: Send {
    async fn handle(&mut self);
}

pub struct TCPConnector {
    handler: Box<dyn TCPConnectorHandler>,
}

impl TCPConnector {
    pub fn new(handler: Box<dyn TCPConnectorHandler>) -> TCPConnector {
        TCPConnector { handler: handler }
    }
}

#[async_trait]
impl Connector for TCPConnector {
    async fn run(&mut self, _: &mut App, handle: Handle, mut signal: &mut Fuse<Receiver<bool>>) {
        println!("Running TCPConnector");

        let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 1883); // localhost:1883
        let listener = TcpListener::bind(address).await.unwrap();

        loop {
            let accept_fuse = listener.accept().fuse();
            tokio::pin!(accept_fuse);

            let res: Result<(TcpStream, SocketAddr), Error> = select! {
                _ = signal => return,
                conn = accept_fuse => conn,
                complete => break,
            };

            if res.is_ok() {
                let mut conn_handler = self.handler.handle_connection(res.unwrap());

                handle.spawn(async move {
                    conn_handler.handle().await;
                });
            }

            tokio::task::yield_now().await;
        }
    }
}
