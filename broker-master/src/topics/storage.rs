use crate::topics::{Topic, TopicManagerError};
use std::collections::{HashMap, HashSet};

pub trait TopicStorage {
    fn add_subscriber(
        &mut self,
        node_name: &str,
        topic_name: &str,
        msg_type: &str,
    ) -> Result<(), TopicManagerError>;
    fn add_publisher(
        &mut self,
        node_name: &str,
        topic_name: &str,
        msg_type: &str,
    ) -> Result<(), TopicManagerError>;
}

pub struct MemoryTopicStorage {
    topics: HashMap<String, Topic>,
}

impl MemoryTopicStorage {
    pub fn new() -> MemoryTopicStorage {
        MemoryTopicStorage {
            topics: HashMap::new(),
        }
    }

    fn add_topic_if_missing(
        &mut self,
        topic_name: &str,
        msg_type: &str,
    ) -> Result<(), TopicManagerError> {
        match self.topics.get(topic_name) {
            Some(topic) => {
                if topic.msg_type != msg_type {
                    Err(TopicManagerError::TopicTypeDoesNotMatch {
                        topic_name: topic_name.into(),
                        requested_type: msg_type.into(),
                        real_type: topic.msg_type.clone(),
                    })
                } else {
                    Ok(())
                }
            }
            None => {
                self.topics.insert(
                    topic_name.into(),
                    Topic {
                        name: topic_name.into(),
                        msg_type: msg_type.into(),
                        publishers: HashSet::new(),
                        subscribers: HashSet::new(),
                        channels: HashMap::new(),
                    },
                );
                Ok(())
            }
        }
    }
}

impl TopicStorage for MemoryTopicStorage {
    fn add_subscriber(
        &mut self,
        node_name: &str,
        topic_name: &str,
        msg_type: &str,
    ) -> Result<(), TopicManagerError> {
        self.add_topic_if_missing(topic_name, msg_type)?;
        if self.topics[topic_name].subscribers.contains(node_name) {
            return Err(TopicManagerError::NodeAlreadySubscribed {
                node_name: node_name.into(),
                topic_name: topic_name.into(),
            });
        }
        self.topics
            .get_mut(topic_name)
            .unwrap()
            .subscribers
            .insert(node_name.into());
        Ok(())
    }

    fn add_publisher(
        &mut self,
        node_name: &str,
        topic_name: &str,
        msg_type: &str,
    ) -> Result<(), TopicManagerError> {
        self.add_topic_if_missing(topic_name, msg_type)?;
        if self.topics[topic_name].publishers.contains(node_name) {
            return Err(TopicManagerError::NodeAlreadyPublishing {
                node_name: node_name.into(),
                topic_name: topic_name.into(),
            });
        }
        self.topics
            .get_mut(topic_name)
            .unwrap()
            .publishers
            .insert(node_name.into());
        Ok(())
    }
}
