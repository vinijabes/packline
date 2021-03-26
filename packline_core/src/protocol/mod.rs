use std::sync::mpsc::Receiver;

pub trait Protocol<'a> {
    fn new(app: &'a mut crate::app::App) -> Self;
    fn spawn(
        app: &'a mut crate::app::App,
        handle: tokio::runtime::Handle,
        stop_signal: Receiver<bool>,
    ) -> tokio::task::JoinHandle<()>;

    fn start(&mut self, handle: tokio::runtime::Handle);
    fn stop(&mut self);
}
