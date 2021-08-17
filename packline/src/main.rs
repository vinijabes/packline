use std::error::Error;
use std::sync::Arc;

use futures::FutureExt;
use tokio::time::Duration;
use tracing::{debug, info};

use packline_cli::client::connect;
use packline_core::app::App;
use packline_core::connector::{Connector, TCPConnector};
use packline_flow::connector::FlowConnector;

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
        let consumer = channel_test.consumer(0);
        let mut producer = channel_test.producer();

        tokio::spawn(async move {
            let mut data = vec![0u32];
            producer.produce(&mut data).await;

            tokio::time::sleep(Duration::from_millis(100)).await;

            let mut data = vec![10u32];
            producer.produce(&mut data).await;
        });

        let result = consumer.consume().await;
        println!("{:?}", result);
        //consumer_test.consume().await;

        tokio::spawn(async move {
            let mut client = connect("127.0.0.1:1883").await.unwrap();

            client
                .consume("testing_topic".to_string(), || {
                    debug!("Handling packets");
                })
                .await;

            let _ = client_rx.await;
        });

        let _ = connector
            .run(app, tokio::runtime::Handle::current(), &mut rx.fuse())
            .await;

        let _ = client_tx.send(true);
        info!("After run!")
    })
    .await;

    Ok(())
}
