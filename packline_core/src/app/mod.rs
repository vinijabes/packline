use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

use self::channel::Channel;

pub mod channel;

/// Handle for packline core functions.
#[derive(Clone)]
pub struct App {
    inner: Arc<Inner>,
}

type ChannelIdentifier = (String, u16);

struct Inner {
    channels: RwLock<HashMap<ChannelIdentifier, Channel>>,
}

pub struct ChannelConfig {
    pub name: String,
    pub partitions: u16,
}

pub struct ChannelPartitionMetadata {}

pub struct ChannelMetadata {
    pub channels: Vec<ChannelPartitionMetadata>,
}

impl App {
    pub fn new() -> App {
        App {
            inner: Arc::new(Inner {
                channels: Default::default(),
            }),
        }
    }

    pub async fn create_channel(&self, config: ChannelConfig) -> Result<ChannelMetadata, ()> {
        let mut guard = self.inner.channels.write().await;

        let channels =
            (1..=config.partitions).fold(Vec::with_capacity(config.partitions.into()), |mut acc, partition| {
                let channel = Channel::new(self.clone());

                guard.insert((config.name.clone(), partition), channel);

                acc.push(ChannelPartitionMetadata {});
                acc
            });

        Ok(ChannelMetadata { channels })
    }

    pub async fn get_channel(&self, identifier: &ChannelIdentifier) -> Option<Channel> {
        let guard = self.inner.channels.read().await;
        guard.get(identifier).cloned()
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{App, ChannelConfig};

    #[tokio::test]
    async fn test_create_channel() {
        let app = App::new();
        let config = ChannelConfig {
            partitions: 3,
            name: "testing_channel".to_string(),
        };

        let result = app.create_channel(config).await;

        assert!(result.is_ok());
        assert_eq!(3, result.unwrap().channels.len());
    }

    #[tokio::test]
    async fn test_get_channel_return_some() {
        let name = "testing_channel".to_string();

        let app = App::new();
        let config = ChannelConfig {
            partitions: 3,
            name: name.clone(),
        };

        let _ = app.create_channel(config).await;
        let channel = app.get_channel(&(name, 1)).await;

        assert!(channel.is_some());
    }
}
