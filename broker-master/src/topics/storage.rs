use crate::topics::{Topic, TopicManagerError};
use std::collections::{HashMap, HashSet};

/// This trait should be implemented by any struct that will act as a storage device
/// for topic data. This can be an in-memory storage, a database, a file, etc. The
/// methods are supposed to be basic operations over the data, with a little bit of
/// management and consistency-maintaining.
pub trait TopicStorage {
    /// Add a subscriber to a given topic. If the topic does not exist, create it.
    fn add_subscriber(
        &mut self,
        node_name: &str,
        topic_name: &str,
        msg_type: &str,
    ) -> Result<(), TopicManagerError>;

    /// Add a publisher to a given topic. If the topic does not exist, create it.
    fn add_publisher(
        &mut self,
        node_name: &str,
        topic_name: &str,
        msg_type: &str,
    ) -> Result<(), TopicManagerError>;

    /// Add a channel between a publisher and a subscriber. The channel ID should be a URI-like
    /// identifier for the communication method.
    fn add_channel(
        &mut self,
        publisher: &str,
        subscriber: &str,
        topic: &str,
        channel_id: &str,
    ) -> Result<(), TopicManagerError>;

    /// Remove a subscriber from a topic. This should remove any channels that were added to the
    /// topic for this subscriber. If the topic is empty, it should delete that too.
    fn remove_subscriber(
        &mut self,
        node_name: &str,
        topic_name: &str,
    ) -> Result<(), TopicManagerError>;

    /// Remove a publisher from a topic. This should remove any channels that were added to the
    /// topic for this subscriber. If the topic is empty, it should delete that too.
    fn remove_publisher(
        &mut self,
        node_name: &str,
        topic_name: &str,
    ) -> Result<(), TopicManagerError>;

    /// Fetches the channel connecting a given publisher and subscriber in a topic. Throws an error
    /// if such a channel doesn't exist. Clients should ensure that channels exist for every
    /// publisher-subscriber pairs on a given topic.
    fn get_channel(
        &self,
        publisher: &str,
        subscriber: &str,
        topic: &str,
    ) -> Result<String, TopicManagerError>;

    /// Fetches all the subscribers on a given topic, as a hash set of node names
    fn get_subscribers(&self, topic_name: &str) -> Result<HashSet<String>, TopicManagerError>;

    /// Fetches all the publishers on a given topic, as a hash set of node names
    fn get_publishers(&self, topic_name: &str) -> Result<HashSet<String>, TopicManagerError>;

    /// Returns a list of topics a given node is subscribed to, as a hash set
    fn topics_subscribed_to(&self, node_name: &str) -> Result<HashSet<String>, TopicManagerError>;

    /// Returns a list of topics a given node is subscribed to, as a hash set
    fn topics_publishing_to(&self, node_name: &str) -> Result<HashSet<String>, TopicManagerError>;
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

    fn add_channel(
        &mut self,
        publisher: &str,
        subscriber: &str,
        topic_name: &str,
        channel_id: &str,
    ) -> Result<(), TopicManagerError> {
        let topic = self
            .topics
            .get_mut(topic_name)
            .ok_or_else(|| TopicManagerError::TopicDoesNotExist(topic_name.into()))?;

        // Make sure publishers and subscribers exists and the channel doesn't
        topic
            .publishers
            .get(publisher)
            .ok_or_else(|| TopicManagerError::NotPublishing {
                publisher: publisher.into(),
                topic_name: topic.name.clone(),
            })?;
        topic
            .subscribers
            .get(publisher)
            .ok_or_else(|| TopicManagerError::NotPublishing {
                publisher: publisher.into(),
                topic_name: topic.name.clone(),
            })?;
        topic
            .channels
            .get(&(publisher.into(), subscriber.into()))
            .ok_or_else(|| TopicManagerError::ChannelAlreadyExists {
                publisher: publisher.into(),
                subscriber: subscriber.into(),
                topic_name: topic.name.clone(),
            })?;

        topic
            .channels
            .insert((publisher.into(), subscriber.into()), channel_id.into());
        Ok(())
    }

    fn remove_subscriber(
        &mut self,
        node_name: &str,
        topic_name: &str,
    ) -> Result<(), TopicManagerError> {
        let topic = self
            .topics
            .get_mut(topic_name)
            .ok_or_else(|| TopicManagerError::TopicDoesNotExist(topic_name.into()))?;

        // Generate a hash set of all potential channels
        let potential_channels = topic
            .publishers
            .iter()
            .map(|publisher| (publisher.clone(), node_name.to_string()))
            .collect::<HashSet<_>>();
        // Remove any channels in the potential_channels set
        topic
            .channels
            .retain(|k, _| !potential_channels.contains(k));

        // Remove the subscriber
        topic.subscribers.remove(node_name);

        // Remove the topic if it's empty
        if topic.publishers.is_empty() && topic.subscribers.is_empty() {
            self.topics.remove(topic_name);
        }
        Ok(())
    }

    fn remove_publisher(
        &mut self,
        node_name: &str,
        topic_name: &str,
    ) -> Result<(), TopicManagerError> {
        let topic = self
            .topics
            .get_mut(topic_name)
            .ok_or_else(|| TopicManagerError::TopicDoesNotExist(topic_name.into()))?;

        // Generate a hash set of all potential channels
        let potential_channels = topic
            .subscribers
            .iter()
            .map(|subscriber| (node_name.to_string(), subscriber.clone()))
            .collect::<HashSet<_>>();
        // Remove any channels in the potential_channels set
        topic
            .channels
            .retain(|k, _| !potential_channels.contains(k));

        // Remove the subscriber
        topic.publishers.remove(node_name);

        // Remove the topic if it's empty
        if topic.publishers.is_empty() && topic.subscribers.is_empty() {
            self.topics.remove(topic_name);
        }
        Ok(())
    }

    fn get_channel(
        &self,
        publisher: &str,
        subscriber: &str,
        topic_name: &str,
    ) -> Result<String, TopicManagerError> {
        self.topics
            .get(topic_name)
            .ok_or_else(|| TopicManagerError::TopicDoesNotExist(topic_name.into()))?
            .channels
            .get(&(publisher.into(), subscriber.into()))
            .cloned()
            .ok_or_else(|| TopicManagerError::ChannelDoesNotExist {
                publisher: publisher.into(),
                subscriber: subscriber.into(),
                topic_name: topic_name.into(),
            })
    }

    fn get_subscribers(&self, topic_name: &str) -> Result<HashSet<String>, TopicManagerError> {
        Ok(self
            .topics
            .get(topic_name)
            .ok_or_else(|| TopicManagerError::TopicDoesNotExist(topic_name.into()))?
            .subscribers
            .clone())
    }

    fn get_publishers(&self, topic_name: &str) -> Result<HashSet<String>, TopicManagerError> {
        Ok(self
            .topics
            .get(topic_name)
            .ok_or_else(|| TopicManagerError::TopicDoesNotExist(topic_name.into()))?
            .publishers
            .clone())
    }

    fn topics_subscribed_to(&self, node_name: &str) -> Result<HashSet<String>, TopicManagerError> {
        Ok(self
            .topics
            .iter()
            .filter_map(|(_, topic)| topic.subscribers.get(node_name).map(|_| topic.name.clone()))
            .collect())
    }

    fn topics_publishing_to(&self, node_name: &str) -> Result<HashSet<String>, TopicManagerError> {
        Ok(self
            .topics
            .iter()
            .filter_map(|(_, topic)| topic.publishers.get(node_name).map(|_| topic.name.clone()))
            .collect())
    }
}
