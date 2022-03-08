use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use spin::RwLock as SpinRwLock;
use tokio::sync::RwLock;

use crate::app::channel::consumer::{BaseConsumerStrategy, Consumer, ConsumerWaker};
use crate::app::channel::storage::VecStorage;

use super::consumer::ConsumerStrategy;
use super::storage::ChannelStorage;
use crate::app::channel::producer::Producer;

#[derive(Clone)]
pub struct Channel {
    inner: Arc<SpinRwLock<Inner>>,
}

struct Inner {
    pub storage: Option<Arc<dyn ChannelStorage>>,
    pub consumer_strategy: Option<Arc<dyn ConsumerStrategy>>,

    pub consumer_group_handlers: Arc<RwLock<HashMap<u128, Arc<ConsumerGroupHandler>>>>,
}

impl Channel {
    pub fn new(app: crate::app::App) -> Self {
        let channel = Channel {
            inner: Arc::new(SpinRwLock::new(Inner::new())),
        };
        let consumer_strategy = BaseConsumerStrategy::new(app.clone(), channel.clone());
        let storage = VecStorage::new(app, channel.clone());
        {
            let mut inner = channel.inner.write();

            inner.consumer_strategy = Some(Arc::new(consumer_strategy));
            inner.storage = Some(Arc::new(storage));
        }

        channel
    }

    pub fn consumer(&self, consumer_id: u128) -> Consumer {
        Consumer::new(
            consumer_id,
            futures::executor::block_on(self.consumer_group_handler(consumer_id)),
        )
    }

    pub fn producer(&self) -> Producer {
        Producer::new(self.clone())
    }

    pub(crate) async fn consumer_group_handler(&self, consumer_id: u128) -> Arc<ConsumerGroupHandler> {
        let inner = self.inner.read();
        let guard = inner.consumer_group_handlers.read().await;

        if let Some(consumer_group_handler) = guard.get(&consumer_id) {
            return consumer_group_handler.clone();
        }

        drop(guard);

        let handler = Arc::new(ConsumerGroupHandler::new(
            inner.consumer_strategy.as_ref().unwrap().clone(),
        ));

        let mut guard = inner.consumer_group_handlers.write().await;

        guard.insert(consumer_id, handler.clone());
        handler
    }

    pub(crate) fn storage(&self) -> Option<Arc<dyn ChannelStorage>> {
        self.inner.read().storage.clone()
    }

    pub(crate) fn consumer_strategy(&self) -> Option<Arc<dyn ConsumerStrategy>> {
        self.inner.read().consumer_strategy.clone()
    }

    pub(crate) fn consumer_group_handlers(&self) -> Arc<RwLock<HashMap<u128, Arc<ConsumerGroupHandler>>>> {
        self.inner.read().consumer_group_handlers.clone()
    }
}

impl Inner {
    pub fn new() -> Self {
        Inner {
            storage: None,
            consumer_strategy: None,
            consumer_group_handlers: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

pub(crate) struct ConsumerGroupHandler {
    offset: AtomicUsize,
    consumer_strategy: Arc<dyn ConsumerStrategy>,

    waker: Arc<ConsumerWaker>,
}

impl ConsumerGroupHandler {
    pub fn new(consumer_strategy: Arc<dyn ConsumerStrategy>) -> ConsumerGroupHandler {
        ConsumerGroupHandler {
            offset: AtomicUsize::new(0),
            consumer_strategy,
            waker: Arc::new(ConsumerWaker::new()),
        }
    }

    pub fn waker(&self) -> Arc<ConsumerWaker> {
        self.waker.clone()
    }

    pub async fn consume(&self, count: usize) -> Option<Vec<u32>> {
        let current_offset = self.offset.load(Ordering::Relaxed);

        let result = self.consumer_strategy.consume(current_offset, count);
        if let Some(data) = &result {
            self.offset.store(current_offset + data.len(), Ordering::Relaxed);
        }

        result
    }
}
