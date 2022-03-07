use spin::Mutex;
use std::convert::TryInto;

use super::Channel;

pub(crate) trait ChannelStorage: Send + Sync {
    fn new(app: crate::app::App, channel: Channel) -> Self
    where
        Self: Sized;

    fn enqueue(&self, elements: &mut Vec<u32>);
    fn remove(&self, count: usize);
    fn peek(&self, offset: usize, count: usize) -> Vec<u32>;
}

pub struct VecStorage {
    data: Mutex<Vec<u32>>,
}

impl ChannelStorage for VecStorage {
    fn new(_app: crate::app::App, _channel: Channel) -> Self {
        VecStorage {
            data: Mutex::new(Vec::new()),
        }
    }

    fn enqueue(&self, elements: &mut Vec<u32>) {
        let mut guard = self.data.lock();
        guard.append(elements);
    }

    fn remove(&self, count: usize) {
        let mut guard = self.data.lock();
        guard.drain(0..count);
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

#[cfg(test)]
mod tests {
    use crate::app::{App, ChannelConfig};

    use super::{ChannelStorage, VecStorage};

    #[tokio::test]
    async fn test_vec_storage() {
        let topic_name: String = "test_topic".to_string();

        let app = App::new();
        let result = app
            .create_channel(ChannelConfig {
                name: topic_name.clone(),
                partitions: 1,
            })
            .await;
        assert!(result.is_ok());

        let channel = app.get_channel(&(topic_name, 1)).await;
        assert!(channel.is_some());

        let channel = channel.unwrap();
        let storage = VecStorage::new(app, channel);

        assert_eq!(storage.peek(0, 1), vec![]);

        storage.enqueue(&mut vec![0u32]);
        assert_eq!(storage.peek(0, 1), vec![0u32]);

        storage.remove(1);
        assert_eq!(storage.peek(0, 1), vec![]);
    }
}
