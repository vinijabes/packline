use spin::Mutex;
use std::collections::LinkedList;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Weak};
use std::task::{Context, Poll, Waker};

use futures::task::AtomicWaker;
#[allow(unused_imports)]
use futures::FutureExt;
use tokio::time::{self, Duration};

use super::channel::ConsumerGroupHandler;
use super::Channel;

pub(crate) trait ConsumerStrategy: Send + Sync {
    fn new(app: crate::app::App, channel: Channel) -> Self
    where
        Self: Sized;

    fn produce(&self, data: &mut Vec<u32>);
    fn consume(&self, offset: usize, count: usize) -> Option<Vec<u32>>;
}

pub struct BaseConsumerStrategy {
    channel: Channel,
}

impl ConsumerStrategy for BaseConsumerStrategy {
    fn new(_: crate::app::App, channel: Channel) -> Self {
        BaseConsumerStrategy { channel }
    }

    fn produce(&self, data: &mut Vec<u32>) {
        if let Some(storage) = self.channel.storage() {
            storage.enqueue(data);
        }
    }

    fn consume(&self, offset: usize, count: usize) -> Option<Vec<u32>> {
        if let Some(storage) = self.channel.storage() {
            let result = storage.peek(offset, count);

            if result.is_empty() {
                None
            } else {
                Some(result)
            }
        } else {
            None
        }
    }
}

pub(crate) struct ConsumerWaker {
    wakers: Mutex<LinkedList<Weak<ConsumerWakerHandle>>>,
}

struct ConsumerWakerHandle {
    parent: Weak<ConsumerWaker>,
    inner: AtomicWaker,
}

impl ConsumerWaker {
    pub(super) fn new() -> ConsumerWaker {
        ConsumerWaker {
            wakers: Mutex::new(LinkedList::new()),
        }
    }

    #[allow(clippy::redundant_clone)]
    fn handle(self: &Arc<Self>) -> Arc<ConsumerWakerHandle> {
        let handle = Arc::new(ConsumerWakerHandle {
            parent: Arc::downgrade(&self.clone()),
            inner: AtomicWaker::new(),
        });

        let mut guard = self.wakers.lock();
        guard.push_back(Arc::downgrade(&handle.clone()));

        handle
    }

    fn remove(&self, handle: *const ConsumerWakerHandle) {
        let mut guard = self.wakers.lock();
        guard
            .iter()
            .position(|h| std::ptr::eq(h.as_ptr(), handle))
            .map(|pos| guard.remove(pos));
    }

    pub fn wake(&self) {
        let mut guard = self.wakers.lock();
        while let Some(weak) = guard.pop_front() {
            if let Some(waker) = weak.upgrade() {
                waker.wake();
                guard.push_back(weak);

                drop(guard);
                return;
            }
        }
    }
}

impl ConsumerWakerHandle {
    fn register(&self, waker: &Waker) {
        self.inner.register(waker)
    }

    fn wake(&self) {
        self.inner.wake()
    }
}

impl Drop for ConsumerWakerHandle {
    fn drop(&mut self) {
        if let Some(parent) = self.parent.upgrade() {
            parent.remove(self as *const ConsumerWakerHandle)
        }
    }
}

pub struct Consumer {
    #[allow(dead_code)]
    consumer_id: u128,

    configs: ConsumerConfigs,
    handler: Arc<ConsumerGroupHandler>,
}

struct ConsumerConfigs {
    timeout: u64,
}

impl<'a> Consumer {
    pub(crate) fn new(consumer_id: u128, handler: Arc<ConsumerGroupHandler>) -> Self {
        Consumer {
            consumer_id,
            configs: ConsumerConfigs { timeout: 1000 },
            handler,
        }
    }

    pub fn consume(&self) -> ConsumerFuture<time::Sleep> {
        ConsumerFuture::new(
            self.handler.waker().handle(),
            self.handler.clone(),
            self.configs.timeout,
        )
    }
}

type PinConsumerFuture = Pin<Box<dyn Future<Output = Option<Vec<u32>>>>>;

use crate::internal::time::sleep::SleepTrait;

pub struct ConsumerFuture<S: SleepTrait> {
    timeout_future: Pin<Box<S>>,
    consumer_future: Option<PinConsumerFuture>,
    waker_handle: Arc<ConsumerWakerHandle>,
    handler: Arc<ConsumerGroupHandler>,

    buffer: Vec<u32>,
}

unsafe impl<S: SleepTrait> Send for ConsumerFuture<S> {}
unsafe impl<S: SleepTrait> Sync for ConsumerFuture<S> {}

impl<'a, S> ConsumerFuture<S>
where
    S: SleepTrait,
{
    fn new(handle: Arc<ConsumerWakerHandle>, handler: Arc<ConsumerGroupHandler>, timeout: u64) -> Self {
        let sleep = unsafe { Pin::new_unchecked(Box::new(S::new(Duration::from_millis(timeout)))) };

        ConsumerFuture {
            timeout_future: sleep,
            consumer_future: None,
            waker_handle: handle,
            handler,

            buffer: Vec::new(),
        }
    }

    #[cfg(test)]
    fn new_with_timeout(
        handle: Arc<ConsumerWakerHandle>,
        handler: Arc<ConsumerGroupHandler>,
        timeout_future: S,
    ) -> Self {
        let sleep = unsafe { Pin::new_unchecked(Box::new(timeout_future)) };

        ConsumerFuture {
            timeout_future: sleep,
            consumer_future: None,
            waker_handle: handle,
            handler,

            buffer: Vec::new(),
        }
    }
}

impl<'a, S> Future for ConsumerFuture<S>
where
    S: SleepTrait,
{
    type Output = Vec<u32>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        macro_rules! try_recv {
            () => {
                if let Some(c) = self.consumer_future.as_mut() {
                    if let Poll::Ready(result) = c.as_mut().poll(cx) {
                        self.consumer_future.take();

                        self.buffer.append(&mut result.unwrap_or(Vec::new()));

                        if self.timeout_future.is_elapsed() && !self.buffer.is_empty() {
                            return Poll::Ready(std::mem::take(&mut self.buffer));
                        }
                    }
                } else {
                    let mut_channel = self.handler.clone();
                    self.consumer_future = unsafe {
                        Some(Pin::new_unchecked(Box::new(async move {
                            //let pointer: &mut Inner = &mut *mut_channel.inner.get();
                            mut_channel.consume(50).await
                        })))
                    };
                }
            };
        }

        try_recv!();
        self.waker_handle.register(cx.waker());
        try_recv!();

        match self.timeout_future.as_mut().poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(_) => {
                if self.buffer.is_empty() {
                    Poll::Pending
                } else {
                    Poll::Ready(std::mem::take(&mut self.buffer))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{task::Poll, time::Duration};

    use futures::{task::noop_waker_ref, FutureExt};

    use crate::{
        app::{App, ChannelConfig},
        internal::time::sleep::{MockSleep, SleepTrait},
    };

    use super::ConsumerFuture;

    #[tokio::test]
    async fn test_consumer_future_ready_only_on_available_data() {
        let topic = "testing_topic".to_string();

        let app = App::new();
        let result = app
            .create_channel(ChannelConfig {
                name: topic.clone(),
                partitions: 1,
            })
            .await;
        assert!(result.is_ok());

        let channel = app.get_channel(&(topic.clone(), 1)).await.unwrap();

        let consumer_group_handler = channel.consumer_group_handler(0).await;
        let consumer_waker_handler = consumer_group_handler.waker().handle();

        let mock_sleep = MockSleep::new(Duration::from_millis(1000));
        let mut future =
            ConsumerFuture::new_with_timeout(consumer_waker_handler, consumer_group_handler, mock_sleep.clone());

        let waker = noop_waker_ref();
        let mut cx = std::task::Context::from_waker(waker);

        let mut producer = channel.producer();

        assert_eq!(future.poll_unpin(&mut cx), Poll::Pending);
        producer.produce(&mut vec![0u32]).await;

        assert_eq!(future.poll_unpin(&mut cx), Poll::Pending);
        mock_sleep.force_complete();

        producer.produce(&mut vec![1u32]).await;

        assert_eq!(future.poll_unpin(&mut cx), Poll::Ready(vec![0u32, 1u32]));
    }
}
