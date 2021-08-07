use std::task::Context;

use futures::task::AtomicWaker;
use tokio::sync::Mutex;

use crate::internal::queue::Queue;

pub struct WaitList {
    inner: Mutex<Inner>,
}

struct Inner {
    queue: Queue<AtomicWaker>,
}

impl WaitList {
    #[inline]
    pub fn new() -> WaitList {
        WaitList {
            inner: Mutex::new(Inner { queue: Queue::new() }),
        }
    }

    pub async fn wait(&self, cx: &mut Context<'_>) {
        let waker = AtomicWaker::new();
        waker.register(cx.waker());

        let mut guard = self.inner.lock().await;
        guard.wait(waker);
    }
}

impl Inner {
    pub fn wait(&mut self, waker: AtomicWaker) {
        self.queue.push(waker);
    }

    pub fn notify_one(&mut self) {
        if let Some(waker) = self.queue.pop() {
            waker.wake();
        }
    }

    pub fn notify_all(&mut self) {
        while let Some(waker) = self.queue.pop() {
            waker.wake();
        }
    }
}
