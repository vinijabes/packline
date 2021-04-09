mod channel;
pub mod consumer;
pub mod storage;

use consumer::ConsumerStrategy;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use storage::ChannelStorage;

pub struct Channel {
    pub(super) storage: Option<Rc<RefCell<dyn ChannelStorage>>>,
    pub(self) consumer_strategy: Option<Rc<RefCell<dyn ConsumerStrategy>>>,

    pub(self) consumers_offsets: HashMap<u128, usize>,
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
