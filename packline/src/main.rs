use futures::FutureExt;
use packline_cli::client::connect;
use packline_core::app::channel::storage::VecStorage;
use packline_core::app::channel::Channel;
use packline_core::app::App;
use packline_core::connector::{Connector, TCPConnector};
use packline_flow::FlowConnector;

#[tokio::main]
async fn main() {
    let _ = tokio::spawn(async {
        let mut app = &mut packline_core::app::App {};

        //TODO: detect program shutdown step and send oneshot signal.
        let (_tx, rx) = tokio::sync::oneshot::channel();

        let mut connector = TCPConnector::new(Box::new(FlowConnector { app: &App {} }));

        tokio::spawn(async {
            let _ = connect("127.0.0.1:1883").await;
        });

        connector
            .run(app, tokio::runtime::Handle::current(), &mut rx.fuse())
            .await;

        println!("After run!")
    })
    .await;
}
