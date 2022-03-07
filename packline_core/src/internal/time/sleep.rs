use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;
use std::{pin::Pin, task::Context};

use futures::task::Poll;
use futures::Future;
use spin::Mutex;
use tokio::time::Sleep as TokioSleep;

pub trait SleepTrait: Future {
    fn new(duration: Duration) -> Self;
    fn is_elapsed(&self) -> bool;
}

impl SleepTrait for TokioSleep {
    fn new(duration: Duration) -> Self {
        tokio::time::sleep(duration)
    }

    fn is_elapsed(&self) -> bool {
        TokioSleep::is_elapsed(self)
    }
}

#[derive(Clone)]
pub struct MockSleep {
    inner: Arc<Inner>,
}

impl Deref for MockSleep {
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct Inner {
    duration: Duration,
    elapsed: Mutex<Duration>,
}

impl Inner {
    fn poll(&self, _: &mut Context<'_>) -> Poll<()> {
        if self.is_elapsed() {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }

    fn is_elapsed(&self) -> bool {
        *self.elapsed.lock() >= self.duration
    }

    #[allow(dead_code)]
    pub fn set_elapsed(&self, elapsed: Duration) {
        *self.elapsed.lock() = elapsed;
    }

    #[allow(dead_code)]
    pub fn force_complete(&self) {
        *self.elapsed.lock() = self.duration;
    }
}

impl Future for MockSleep {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.inner.poll(cx)
    }
}

impl SleepTrait for MockSleep {
    fn new(duration: Duration) -> Self {
        MockSleep {
            inner: Arc::new(Inner {
                duration,
                elapsed: Mutex::new(Duration::ZERO),
            }),
        }
    }

    fn is_elapsed(&self) -> bool {
        self.inner.is_elapsed()
    }
}
