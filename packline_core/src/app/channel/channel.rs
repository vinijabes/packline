use std::cell::{RefCell, UnsafeCell};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

use tokio::sync::Mutex;

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

    pub consumer_group_handlers: HashMap<u128, Mutex<ConsumerGroupHandler>>,
}

unsafe impl Send for Inner {}
unsafe impl Sync for Inner {}

unsafe impl Send for UnsafeSync<UnsafeCell<Inner>> {}
unsafe impl Sync for UnsafeSync<UnsafeCell<Inner>> {}

pub(crate) struct ConsumerGroupHandler {
    offset: usize,

    waker: Arc<ConsumerWaker>,
}

impl Channel {
    pub fn new(app: &mut crate::app::App) -> Self {
        let inner = Inner::new(app);

        let mut channel = Channel {
            inner: Arc::new(UnsafeSync::new(UnsafeCell::new(inner))),
        };

        channel
    }

    pub fn consumer(&self, consumer_id: u128) -> Consumer {
        unsafe {
            let pointer: &mut Inner = &mut *self.inner.get();
            Consumer::new(
                self.inner.clone(),
                consumer_id,
                futures::executor::block_on(pointer.consumer_waker(consumer_id)),
            )
        }
    }

    pub fn producer(&self) -> Producer {
        Producer::new(self.inner.clone())
    }
}

impl Inner {
    pub fn new(app: &mut crate::app::App) -> Self {
        let mut inner = Inner {
            storage: None,
            consumer_strategy: None,
            consumer_group_handlers: HashMap::new(),
        };

        let storage = Rc::new(RefCell::new(VecStorage::new(app, &mut inner)));
        inner.storage = Some(storage);

        let consumer_strategy = Rc::new(RefCell::new(BaseConsumerStrategy::new(app, &mut inner)));
        inner.consumer_strategy = Some(consumer_strategy);

        inner
    }

    pub fn produce(&mut self, data: &mut Vec<u32>) {
        self.consumer_strategy.as_ref().unwrap().borrow_mut().produce(data);
        for (_, consumer_group_handler) in self.consumer_group_handlers.iter_mut() {
            let mut guard = futures::executor::block_on(consumer_group_handler.lock());
            guard.waker.wake()
        }
    }

    pub async fn consume(&mut self, offset: usize, count: usize) -> Option<Vec<u32>> {
        let result = self.consumer_strategy.as_ref().unwrap().borrow().consume(offset, count);

        result
    }

    async fn consumer_waker(&mut self, consumer_id: u128) -> Arc<ConsumerWaker> {
        if self.consumer_group_handlers.contains_key(&consumer_id) {
            let guard = self.consumer_group_handlers.get(&consumer_id).unwrap().lock().await;
            guard.waker.clone()
        } else {
            let waker = Arc::new(ConsumerWaker::new());
            let handler = ConsumerGroupHandler {
                offset: 0,
                waker: waker.clone(),
            };

            self.consumer_group_handlers.insert(consumer_id, Mutex::new(handler));

            return waker;
        }
    }
}
