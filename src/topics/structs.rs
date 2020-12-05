use std::{collections::HashSet, fmt};
use thiserror::Error;

#[derive(Debug)]
pub enum PubOrSub {
    Publisher,
    Subscriber,
}

#[derive(Error, Debug)]
pub enum TopicManagerError {
    #[error("Node `{node}` was already registered as a {pubsub} of `{topic}`")]
    NodeAlreadyExists {
        node: String,
        topic: String,
        pubsub: PubOrSub,
    },
    #[error("Node `{node}` does not exist as a {pubsub} of `{topic}`")]
    NodeDoesntExist {
        node: String,
        topic: String,
        pubsub: PubOrSub,
    },
}

pub struct Topic {
    name: String,
    subscribers: HashSet<String>,
    publishers: HashSet<String>,
}

impl fmt::Display for PubOrSub {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PubOrSub::Publisher => write!(f, "Publisher"),
            PubOrSub::Subscriber => write!(f, "Subscriber"),
        }
    }
}

impl Topic {
    pub fn new(name: &str) -> Topic {
        Topic {
            name: name.to_string(),
            subscribers: HashSet::new(),
            publishers: HashSet::new(),
        }
    }

    pub fn add_subscriber(&mut self, name: &str) -> Result<(), TopicManagerError> {
        if self.subscribers.insert(name.to_string()) {
            Ok(())
        } else {
            Err(TopicManagerError::NodeAlreadyExists {
                node: name.to_string(),
                topic: self.name.to_string(),
                pubsub: PubOrSub::Subscriber,
            })
        }
    }

    pub fn remove_subscriber(&mut self, name: &str) -> Result<(), TopicManagerError> {
        if self.subscribers.remove(name) {
            Ok(())
        } else {
            Err(TopicManagerError::NodeDoesntExist {
                node: name.to_string(),
                topic: self.name.to_string(),
                pubsub: PubOrSub::Subscriber,
            })
        }
    }

    pub fn add_publisher(&mut self, name: &str) -> Result<(), TopicManagerError> {
        if self.publishers.insert(name.to_string()) {
            Ok(())
        } else {
            Err(TopicManagerError::NodeAlreadyExists {
                node: name.to_string(),
                topic: self.name.to_string(),
                pubsub: PubOrSub::Publisher,
            })
        }
    }

    pub fn remove_publisher(&mut self, name: &str) -> Result<(), TopicManagerError> {
        if self.publishers.remove(name) {
            Ok(())
        } else {
            Err(TopicManagerError::NodeDoesntExist {
                node: name.to_string(),
                topic: self.name.to_string(),
                pubsub: PubOrSub::Publisher,
            })
        }
    }
}
