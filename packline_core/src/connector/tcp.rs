use std::io::Error;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;

use async_trait::async_trait;
use futures::{future::Fuse, FutureExt, select};
use tokio::net::{TcpListener, TcpStream};
use tokio::runtime::Handle;
use tokio::sync::Mutex;
use tokio::sync::oneshot::Receiver;

use super::{App, Connector};

#[async_trait]
pub trait TCPConnectorHandler: Send {
    async fn handle_connection(&self, conn: (TcpStream, SocketAddr));
}

pub struct TCPConnector {
    handler: Arc<Mutex<Box<dyn TCPConnectorHandler>>>
}

impl TCPConnector {
    pub fn new(handler: Box<dyn TCPConnectorHandler>) -> TCPConnector {
        TCPConnector {
            handler: Arc::new(Mutex::new(handler))
        }
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
                let counter = self.handler.clone();
                //TODO: Perhaps there is a better way to do this, as it would be unnecessary lock to spawn the connection task.
                handle.spawn(async move {
                    let handler = counter.lock().await;
                    let conn = res.unwrap();

                    let result = (*handler).handle_connection(conn);
                    result.await;
                });
            }

            tokio::task::yield_now().await;
        }
    }
}
