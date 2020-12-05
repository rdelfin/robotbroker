use crate::topics::{Topic, TopicManagerError};
use std::collections::HashMap;

pub struct TopicManager {
    topics: HashMap<String, Topic>,
}

impl TopicManager {
    pub fn new() -> TopicManager {
        TopicManager {
            topics: HashMap::new(),
        }
    }

    pub fn add_subscriber(
        &mut self,
        topic_name: &str,
        node_name: &str,
    ) -> Result<(), TopicManagerError> {
        self.topics
            .entry(topic_name.to_string())
            .or_insert(Topic::new(topic_name))
            .add_subscriber(node_name)
    }

    pub fn add_publisher(
        mut self,
        topic_name: &str,
        node_name: &str,
    ) -> Result<(), TopicManagerError> {
        self.topics
            .entry(topic_name.to_string())
            .or_insert(Topic::new(topic_name))
            .add_publisher(node_name)
    }
}
