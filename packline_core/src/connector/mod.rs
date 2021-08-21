use async_trait::async_trait;
use futures::future::Fuse;
pub use tcp::*;
use tokio::runtime::Handle;
use tokio::sync::oneshot::Receiver;

use crate::app::App;

pub mod tcp;

#[async_trait]
pub trait Connector: Send {
    async fn run(&mut self, app: &mut App, handle: Handle, mut signal: &mut Fuse<Receiver<bool>>);
}
