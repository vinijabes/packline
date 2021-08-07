use std::error::Error;
use std::thread::yield_now;

use futures::{AsyncWrite, FutureExt};
use tracing::{debug, info};

use packline_cli::client::connect;
use packline_core::app::App;
use packline_core::connector::{Connector, TCPConnector};
use packline_flow::connector::FlowConnector;
use std::sync::Arc;
use std::cell::{RefCell, Cell};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .try_init()?;

    info!("Starting Packline");
    let _ = tokio::spawn(async {
        let app = &mut packline_core::app::App {};

        //TODO: detect program shutdown step and send oneshot signal.
        let (_tx, rx) = tokio::sync::oneshot::channel();
        let (client_tx, client_rx) = tokio::sync::oneshot::channel();

        let mut connector = TCPConnector::new(Box::new(FlowConnector { app: &App {} }));

        let channel_test = Arc::new(Box::new(packline_core::app::channel::Channel::new(app)));
        let mut consumer = channel_test.consumer(0);
        consumer.consume().await;
        //consumer_test.consume().await;


        tokio::spawn(async move {
            let mut client = connect("127.0.0.1:1883").await.unwrap();

            client
                .consume("testing_topic".to_string(), || {
                    debug!("Handling packets");
                })
                .await;

            client_rx.await;
        });

        connector
            .run(app, tokio::runtime::Handle::current(), &mut rx.fuse())
            .await;

        client_tx.send(true);
        info!("After run!")
    })
    .await;

    Ok(())
}
