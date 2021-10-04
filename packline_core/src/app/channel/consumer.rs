use std::cell::RefCell;
use std::collections::LinkedList;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Mutex;
use std::sync::{Arc, Weak};
use std::task::{Context, Poll, Waker};

use futures::task::AtomicWaker;
#[allow(unused_imports)]
use futures::FutureExt;
use tokio::time::{self, Duration};

use crate::app::channel::Inner;

use super::channel::ConsumerGroupHandler;
use super::storage::ChannelStorage;

pub(crate) trait ConsumerStrategy: Send + Sync {
    fn new(app: crate::app::App, channel: &mut Inner) -> Self
    where
        Self: Sized;

    fn produce(&mut self, data: &mut Vec<u32>);
    fn consume(&self, offset: usize, count: usize) -> Option<Vec<u32>>;
}

pub struct BaseConsumerStrategy {
    storage: Rc<RefCell<dyn ChannelStorage>>,
}

unsafe impl Send for BaseConsumerStrategy {}

unsafe impl Sync for BaseConsumerStrategy {}

impl ConsumerStrategy for BaseConsumerStrategy {
    fn new(_: crate::app::App, channel: &mut Inner) -> Self {
        BaseConsumerStrategy {
            storage: channel.storage.as_ref().unwrap().clone(),
        }
    }

    fn produce(&mut self, data: &mut Vec<u32>) {
        self.storage.borrow_mut().enqueue(data);
    }

    fn consume(&self, offset: usize, count: usize) -> Option<Vec<u32>> {
        let result = self.storage.borrow().peek(offset, count);

        if result.is_empty() {
            None
        } else {
            Some(result)
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

        let mut guard = self.wakers.lock().unwrap();
        guard.push_back(Arc::downgrade(&handle.clone()));

        handle
    }

    fn remove(&self, handle: *const ConsumerWakerHandle) {
        let mut guard = self.wakers.lock().unwrap();
        let pos = guard.iter().position(|x| std::ptr::eq(x.as_ptr(), handle)).unwrap();

        guard.remove(pos);
    }

    pub fn wake(&self) {
        let mut guard = self.wakers.lock().unwrap();
        if let Some(weak) = guard.pop_front() {
            if let Some(waker) = weak.upgrade() {
                waker.wake();
                guard.push_back(weak);
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
        self.parent
            .upgrade()
            .unwrap()
            .remove(self as *const ConsumerWakerHandle);
    }
}

pub struct Consumer {
    #[allow(dead_code)]
    consumer_id: u128,

    configs: ConsumerConfigs,
    handler: Arc<ConsumerGroupHandler>,
}

unsafe impl Send for Consumer {}

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

    pub fn consume(&self) -> ConsumerFuture {
        ConsumerFuture::new(
            self.handler.waker().handle(),
            self.handler.clone(),
            self.configs.timeout,
        )
    }
}

type PinConsumerFuture = Pin<Box<dyn Future<Output = Option<Vec<u32>>>>>;

pub struct ConsumerFuture {
    timeout_future: Pin<Box<time::Sleep>>,
    consumer_future: Option<PinConsumerFuture>,
    waker_handle: Arc<ConsumerWakerHandle>,
    handler: Arc<ConsumerGroupHandler>,

    buffer: Vec<u32>,
}

unsafe impl Send for ConsumerFuture {}

unsafe impl Sync for ConsumerFuture {}

impl<'a> ConsumerFuture {
    fn new(handle: Arc<ConsumerWakerHandle>, handler: Arc<ConsumerGroupHandler>, timeout: u64) -> Self {
        let sleep = unsafe { Pin::new_unchecked(Box::new(time::sleep(Duration::from_millis(timeout)))) };

        ConsumerFuture {
            timeout_future: sleep,
            consumer_future: None,
            waker_handle: handle,
            handler,

            buffer: Vec::new(),
        }
    }
}

impl<'a> Future for ConsumerFuture {
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
