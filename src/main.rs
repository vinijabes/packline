use futures::FutureExt;
use packline_core::app::App;
use packline_core::connector::{Connector, TCPConnector};
use packline_flow::FlowConnector;

mod core;

#[tokio::main]
async fn main() {
    let _ = tokio::spawn(async {
        let app = &mut packline_core::app::App {};

        //TODO: detect program shutdown step and send oneshot signal.
        let (_tx, rx) = tokio::sync::oneshot::channel();

        let mut connector = TCPConnector::new(Box::new(FlowConnector { app: &App {} }));
        connector
            .run(app, tokio::runtime::Handle::current(), &mut rx.fuse())
            .await;

        println!("After run!")
    }).await;
}
