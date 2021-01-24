use crate::broker_internal::topics::{Topic, TopicManagerError};
use std::collections::HashMap;

pub struct TopicManager {
    topics: HashMap<String, Topic>,
}

impl Default for TopicManager {
    fn default() -> TopicManager {
        TopicManager {
            topics: HashMap::new(),
        }
    }
}

impl TopicManager {
    pub fn add_subscriber(
        &mut self,
        channel_name: &str,
        node_name: &str,
    ) -> Result<(), TopicManagerError> {
        self.topics
            .entry(channel_name.to_string())
            .or_insert(Topic::new(channel_name))
            .add_subscriber(node_name)
    }

    pub fn add_publisher(
        &mut self,
        channel_name: &str,
        node_name: &str,
    ) -> Result<(), TopicManagerError> {
        self.topics
            .entry(channel_name.to_string())
            .or_insert(Topic::new(channel_name))
            .add_publisher(node_name)
    }
}
