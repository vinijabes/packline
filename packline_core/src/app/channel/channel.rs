use std::rc::Rc;

use super::consumer::ConsumerStrategy;
use super::storage::ChannelStorage;
use super::Channel;
use crate::app::channel::consumer::BaseConsumerStrategy;
use crate::app::channel::storage::VecStorage;
use std::cell::RefCell;
use std::collections::HashMap;

unsafe impl Send for Channel {}

impl Channel {
    pub fn new(app: &mut crate::app::App) -> Self {
        let mut channel = Channel {
            storage: None,
            consumer_strategy: None,
            consumers_offsets: HashMap::new(),
        };

        let storage = Rc::new(RefCell::new(VecStorage::new(app, &mut channel)));
        channel.storage = Some(storage);

        let consumer_strategy = Rc::new(RefCell::new(BaseConsumerStrategy::new(app, &mut channel)));
        channel.consumer_strategy = Some(consumer_strategy);

        channel
    }

    pub fn produce(&mut self, data: &mut Vec<u32>) {
        self.consumer_strategy.as_ref().unwrap().borrow_mut().produce(data)
    }

    pub async fn consume(&mut self, consumer_id: u128, count: usize) -> Vec<u32> {
        let result = self
            .consumer_strategy
            .as_ref()
            .unwrap()
            .borrow()
            .consume(*self.consumers_offsets.get(&consumer_id).unwrap_or(&0), count);

        self.consumers_offsets.insert(
            consumer_id,
            self.consumers_offsets.get(&consumer_id).unwrap_or(&0) + result.len(),
        );

        result
    }
}
