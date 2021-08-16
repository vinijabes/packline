use std::cell::UnsafeCell;
use std::sync::Arc;

use crate::app::channel::{Inner, UnsafeSync};

use super::consumer::ConsumerWaker;

pub struct Producer {
    inner: Arc<UnsafeSync<UnsafeCell<Inner>>>,
}

unsafe impl Send for Producer {}

impl<'a> Producer {
    pub(crate) fn new(inner: Arc<UnsafeSync<UnsafeCell<Inner>>>) -> Self {
        Producer { inner }
    }

    pub async fn produce(&mut self, data: &mut Vec<u32>) {
        unsafe {
            let pointer: &mut Inner = &mut *self.inner.get();
            pointer.produce(data);
        }
    }
}
