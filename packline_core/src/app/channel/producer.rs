use std::cell::RefCell;
use std::sync::Arc;

use crate::app::channel::{Inner, UnsafeSync};

use super::consumer::ConsumerWaker;

pub struct Producer {
    inner: Arc<UnsafeSync<RefCell<Inner>>>,
}

unsafe impl Send for Producer {}

impl<'a> Producer {
    pub(crate) fn new(inner: Arc<UnsafeSync<RefCell<Inner>>>) -> Self {
        Producer { inner }
    }

    pub async fn produce(&mut self, data: &mut Vec<u32>) {
        self.inner.borrow_mut().produce(data);
    }
}
