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

    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]
    fn test_channel_produce_and_consume() {
        let mut app = &mut crate::app::App {};
        let mut channel = Channel::new(&mut app);

        channel.produce(&mut vec![1, 2, 3, 4]);
        assert_eq!(aw!(channel.consume(0, 4)), vec![1, 2, 3, 4]);

        assert_eq!(aw!(channel.consume(0, 4)), vec![]);

        channel.produce(&mut vec![1, 2]);
        assert_eq!(aw!(channel.consume(0, 4)), vec![1, 2]);
    }
}
