use super::Channel;

pub struct Producer {
    channel: Channel,
}

impl<'a> Producer {
    pub(crate) fn new(channel: Channel) -> Self {
        Producer { channel }
    }

    pub async fn produce(&mut self, data: &mut Vec<u32>) {
        self.channel.consumer_strategy().as_ref().unwrap().produce(data);
        let consumer_group_handlers = self.channel.consumer_group_handlers();
        let guard = consumer_group_handlers.read().await;

        for (_, consumer_group_handler) in guard.iter() {
            consumer_group_handler.waker().wake();
        }
    }
}
