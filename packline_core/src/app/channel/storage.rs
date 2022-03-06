use crate::app::channel::Inner;
use spin::Mutex;
use std::convert::TryInto;

pub(crate) trait ChannelStorage: Send + Sync {
    fn new(app: crate::app::App, channel: &mut Inner) -> Self
    where
        Self: Sized;

    fn enqueue(&self, elements: &mut Vec<u32>);
    fn dequeue(&self, count: usize) -> Vec<u32>;
    fn peek(&self, offset: usize, count: usize) -> Vec<u32>;
}

pub struct VecStorage {
    data: Mutex<Vec<u32>>,
}

impl ChannelStorage for VecStorage {
    fn new(_app: crate::app::App, _channel: &mut Inner) -> Self {
        VecStorage {
            data: Mutex::new(Vec::new()),
        }
    }

    fn enqueue(&self, elements: &mut Vec<u32>) {
        let mut guard = self.data.lock();
        guard.append(elements);
    }

    fn dequeue(&self, _count: usize) -> Vec<u32> {
        Vec::new()
    }

    fn peek(&self, offset: usize, count: usize) -> Vec<u32> {
        let guard = self.data.lock();

        if offset > guard.len() {
            return vec![];
        }

        let mut count = count;
        if offset + count > guard.len() {
            count = guard.len() - offset;
        }

        guard[offset..offset + count].try_into().unwrap()
    }
}
