use crate::app::channel::Inner;
use crate::internal::queue::iterator::AsyncIterator;
use async_trait::async_trait;
use std::convert::TryInto;

pub(crate) trait ChannelStorage: Send + Sync {
    fn new(app: &mut crate::app::App, channel: &mut Inner) -> Self
    where
        Self: Sized;

    fn enqueue(&mut self, elements: &mut Vec<u32>);
    fn dequeue(&mut self, count: usize) -> Vec<u32>;
    fn peek(&self, offset: usize, count: usize) -> Vec<u32>;

    fn iter(&self) -> Box<dyn AsyncIterator<Item = u32>>;
}

pub struct VecStorage {
    data: Vec<u32>,
}

pub struct Iter {}

#[async_trait]
impl AsyncIterator for Iter {
    type Item = u32;
    async fn next_count(&mut self, _count: usize) -> Vec<u32> {
        vec![42u32]
    }
}

impl ChannelStorage for VecStorage {
    fn new(_app: &mut crate::app::App, _channel: &mut Inner) -> Self {
        VecStorage { data: Vec::new() }
    }

    fn enqueue(&mut self, elements: &mut Vec<u32>) {
        self.data.append(elements);
    }

    fn dequeue(&mut self, _count: usize) -> Vec<u32> {
        Vec::new()
    }

    fn peek(&self, offset: usize, count: usize) -> Vec<u32> {
        if offset > self.data.len() {
            return vec![];
        }

        let mut count = count;
        if offset + count > self.data.len() {
            count = self.data.len() - offset;
        }

        self.data[offset..offset + count].try_into().unwrap()
    }

    fn iter(&self) -> Box<dyn AsyncIterator<Item = u32>> {
        Box::new(Iter {})
    }
}
