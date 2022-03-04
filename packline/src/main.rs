use std::error::Error;

use futures::FutureExt;
use tokio::time::Duration;
use tracing::{debug, info};

use packline_cli::client::connect;
use packline_core::{
    app::ChannelConfig,
    connector::{Connector, TCPConnector},
};
use packline_flow::connector::FlowConnector;
use rand::{rngs::StdRng, Rng, SeedableRng};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .try_init()?;

    info!("Starting Packline");
    let _ = tokio::spawn(async {
        let mut app = packline_core::app::App::new();

        //TODO: detect program shutdown step and send oneshot signal.
        let (_tx, rx) = tokio::sync::oneshot::channel();
        let (client_tx, client_rx) = tokio::sync::oneshot::channel();

        let mut connector = TCPConnector::new(Box::new(FlowConnector { app: app.clone() }));

        let _ = app
            .create_channel(ChannelConfig {
                name: "testing_topic".to_string(),
                partitions: 1,
            })
            .await;

        let channel = app.get_channel(&("testing_topic".to_string(), 1)).await.unwrap();
        let mut producer = channel.producer();

        tokio::spawn(async move {
            let mut data = vec![0u32];
            producer.produce(&mut data).await;

            tokio::time::sleep(Duration::from_millis(100)).await;

            let mut data = vec![10u32];
            producer.produce(&mut data).await;

            let mut interval = tokio::time::interval(Duration::from_millis(10));

            let mut rng = StdRng::from_entropy();
            loop {
                interval.tick().await;

                let value: u32 = rng.gen_range(0u32..100u32);
                producer.produce(&mut vec![value]).await;
            }
        });

        tokio::spawn(async move {
            let mut client = connect("127.0.0.1:1883").await.unwrap();

            client
                .consume("testing_topic".to_string(), |record| {
                    debug!("Handling packets {}", record);
                })
                .await;

            let _ = client_rx.await;
        });

        let _ = connector
            .run(&mut app, tokio::runtime::Handle::current(), &mut rx.fuse())
            .await;

        let _ = client_tx.send(true);
        info!("After run!")
    })
    .await;

    Ok(())
}
