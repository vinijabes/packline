use std::cell::{RefCell, UnsafeCell};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use tokio::sync::RwLock;

use crate::app::channel::consumer::{BaseConsumerStrategy, Consumer, ConsumerWaker};
use crate::app::channel::storage::VecStorage;
use crate::app::channel::UnsafeSync;

use super::consumer::ConsumerStrategy;
use super::storage::ChannelStorage;
use crate::app::channel::producer::Producer;

pub struct Channel {
    inner: Arc<UnsafeSync<UnsafeCell<Inner>>>,
}

pub(crate) struct Inner {
    pub storage: Option<Rc<RefCell<dyn ChannelStorage>>>,
    pub consumer_strategy: Option<Rc<RefCell<dyn ConsumerStrategy>>>,

    pub consumer_group_handlers: RwLock<HashMap<u128, Arc<ConsumerGroupHandler>>>,
}

unsafe impl Send for Inner {}
unsafe impl Sync for Inner {}

unsafe impl Send for UnsafeSync<UnsafeCell<Inner>> {}
unsafe impl Sync for UnsafeSync<UnsafeCell<Inner>> {}

impl Channel {
    pub fn new(app: crate::app::App) -> Self {
        let inner = Inner::new(app);

        Channel {
            inner: Arc::new(UnsafeSync::new(UnsafeCell::new(inner))),
        }
    }

    pub fn consumer(&self, consumer_id: u128) -> Consumer {
        unsafe {
            let pointer: &mut Inner = &mut *self.inner.get();
            Consumer::new(
                consumer_id,
                futures::executor::block_on(pointer.consumer_group_handler(consumer_id)),
            )
        }
    }

    pub fn producer(&self) -> Producer {
        Producer::new(self.inner.clone())
    }
}

impl Inner {
    pub fn new(app: crate::app::App) -> Self {
        let mut inner = Inner {
            storage: None,
            consumer_strategy: None,
            consumer_group_handlers: RwLock::new(HashMap::new()),
        };

        let storage = Rc::new(RefCell::new(VecStorage::new(app.clone(), &mut inner)));
        inner.storage = Some(storage);

        let consumer_strategy = Rc::new(RefCell::new(BaseConsumerStrategy::new(app, &mut inner)));
        inner.consumer_strategy = Some(consumer_strategy);

        inner
    }

    pub fn produce(&mut self, data: &mut Vec<u32>) {
        self.consumer_strategy.as_ref().unwrap().borrow_mut().produce(data);

        let guard = futures::executor::block_on(self.consumer_group_handlers.read());
        for (_, consumer_group_handler) in guard.iter() {
            consumer_group_handler.waker.wake();
        }
    }

    #[allow(dead_code)]
    pub async fn consume(&mut self, offset: usize, count: usize) -> Option<Vec<u32>> {
        let result = self.consumer_strategy.as_ref().unwrap().borrow().consume(offset, count);

        result
    }

    async fn consumer_group_handler(&self, consumer_id: u128) -> Arc<ConsumerGroupHandler> {
        let guard = self.consumer_group_handlers.read().await;

        if let Some(consumer_group_handler) = guard.get(&consumer_id) {
            return consumer_group_handler.clone();
        }

        drop(guard);

        let handler = Arc::new(ConsumerGroupHandler {
            offset: AtomicUsize::new(0),
            consumer_strategy: self.consumer_strategy.as_ref().unwrap().clone(),
            waker: Arc::new(ConsumerWaker::new()),
        });

        let mut guard = self.consumer_group_handlers.write().await;

        guard.insert(consumer_id, handler.clone());
        handler
    }
}

pub(crate) struct ConsumerGroupHandler {
    offset: AtomicUsize,
    consumer_strategy: Rc<RefCell<dyn ConsumerStrategy>>,

    waker: Arc<ConsumerWaker>,
}

unsafe impl Send for ConsumerGroupHandler {}
unsafe impl Sync for ConsumerGroupHandler {}

impl ConsumerGroupHandler {
    pub fn waker(&self) -> Arc<ConsumerWaker> {
        self.waker.clone()
    }

    pub async fn consume(&self, count: usize) -> Option<Vec<u32>> {
        let current_offset = self.offset.load(Ordering::Relaxed);

        let result = self.consumer_strategy.borrow().consume(current_offset, count);
        if let Some(data) = &result {
            self.offset.store(current_offset + data.len(), Ordering::Relaxed);
        }

        result
    }
}
