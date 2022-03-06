use std::sync::Arc;

use crate::app::channel::Inner;

pub struct Producer {
    inner: Arc<Inner>,
}

unsafe impl Send for Producer {}

impl<'a> Producer {
    pub(crate) fn new(inner: Arc<Inner>) -> Self {
        Producer { inner }
    }

    pub async fn produce(&mut self, data: &mut Vec<u32>) {
        self.inner.produce(data);
    }
}
