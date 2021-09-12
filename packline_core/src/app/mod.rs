use std::{collections::HashMap, sync::Arc};

use self::channel::Channel;

pub mod channel;

/// Handle for packline core functions.
#[derive(Clone)]
pub struct App {
    inner: Arc<Inner>,
}
struct Inner {
    #[allow(dead_code)]
    channels: HashMap<String, Channel>,
}

pub struct TopicConfig {
    #[allow(dead_code)]
    name: String,
    #[allow(dead_code)]
    partitions: u16,
}

impl App {
    pub fn new() -> App {
        App {
            inner: Arc::new(Inner {
                channels: Default::default(),
            }),
        }
    }

    pub async fn create_topic(&self, _config: TopicConfig) -> &[Channel] {
        &[]
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{channel::Channel, App, TopicConfig};

    #[tokio::test]
    async fn test_create_topic() {
        let app = App::new();
        let config = TopicConfig {
            partitions: 3,
            name: "testing_topic".to_string(),
        };

        let channels: &[Channel] = app.create_topic(config).await;

        assert_eq!(3, channels.len());
    }
}
