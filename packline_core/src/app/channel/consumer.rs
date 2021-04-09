use std::rc::Rc;

use super::storage::ChannelStorage;
use super::Channel;
use std::cell::RefCell;

pub trait ConsumerStrategy {
    fn new(app: &mut crate::app::App, channel: &mut Channel) -> Self
    where
        Self: Sized;

    fn produce(&mut self, data: &mut Vec<u32>);
    fn consume(&self, offset: usize, count: usize) -> Vec<u32>;
}

pub struct BaseConsumerStrategy {
    storage: Rc<RefCell<dyn ChannelStorage>>,
}

impl ConsumerStrategy for BaseConsumerStrategy {
    fn new(_: &mut crate::app::App, channel: &mut Channel) -> Self {
        BaseConsumerStrategy {
            storage: channel.storage.as_ref().unwrap().clone(),
        }
    }

    fn produce(&mut self, data: &mut Vec<u32>) {
        self.storage.borrow_mut().enqueue(data)
    }

    fn consume(&self, offset: usize, count: usize) -> Vec<u32> {
        self.storage.borrow().peek(offset, count)
    }
}
