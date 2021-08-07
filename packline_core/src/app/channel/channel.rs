use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

use tokio::sync::Mutex;

use crate::app::channel::UnsafeSync;
use crate::app::channel::consumer::{BaseConsumerStrategy, Consumer};
use crate::app::channel::storage::VecStorage;

use super::consumer::ConsumerStrategy;
use super::storage::ChannelStorage;

pub struct Channel {
    inner: Arc<UnsafeSync<RefCell<Inner>>>,
}

pub(crate) struct Inner {
    pub storage: Option<Rc<RefCell<dyn ChannelStorage>>>,
    pub consumer_strategy: Option<Rc<RefCell<dyn ConsumerStrategy>>>,

    pub consumer_group_handlers: HashMap<u128, Mutex<ConsumerGroupHandler>>,
}

unsafe impl Send for Inner {}
unsafe impl Sync for Inner {}

unsafe impl Send for UnsafeSync<RefCell<Inner>> {}
unsafe impl Sync for UnsafeSync<RefCell<Inner>> {}

pub(crate) struct ConsumerGroupHandler {
    offset: usize,

    channel: Rc<RefCell<Inner>>,
}

impl Channel {
    pub fn new(app: &mut crate::app::App) -> Self {
        let inner = Inner::new(app);

        let mut channel = Channel {
            inner: Arc::new(UnsafeSync::new(RefCell::new(inner)))
        };

        channel
    }

    pub fn consumer(&self, consumer_id: u128) -> Consumer {
        Consumer::new(self.inner.clone(), consumer_id)
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
        self.consumer_strategy.as_ref().unwrap().borrow_mut().produce(data)
    }

    pub async fn consume(&mut self, offset: usize, count: usize) -> Option<Vec<u32>> {
        let result = self
            .consumer_strategy
            .as_ref()
            .unwrap()
            .borrow()
            .consume(offset, count);

        result
    }
}
