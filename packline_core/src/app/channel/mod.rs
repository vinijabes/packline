use std::ops::{Deref, DerefMut};

pub use channel::Channel;
pub(crate) use channel::Inner;

#[allow(clippy::module_inception)]
mod channel;
pub mod consumer;
pub mod producer;
pub mod storage;

pub struct UnsafeSync<T> {
    inner: T,
}

impl<T> UnsafeSync<T> {
    fn new(value: T) -> Self {
        UnsafeSync { inner: value }
    }
}

impl<T> Deref for UnsafeSync<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for UnsafeSync<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_channel_produce_and_consume() {
        const CONSUMER_ID1: u128 = 0;
        const CONSUMER_ID2: u128 = 1;

        let mut app = &mut crate::app::App {};
        let channel = Channel::new(&mut app);

        let mut producer = channel.producer();
        let consumer1 = channel.consumer(CONSUMER_ID1);
        let consumer2 = channel.consumer(CONSUMER_ID2);

        producer.produce(&mut vec![1, 2, 3, 4]).await;
        assert_eq!(consumer1.consume().await, vec![1, 2, 3, 4]);

        producer.produce(&mut vec![5, 6]).await;
        assert_eq!(consumer1.consume().await, vec![5, 6]);
        assert_eq!(consumer2.consume().await, vec![1, 2, 3, 4, 5, 6]);
    }
}
