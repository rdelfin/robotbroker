use std::collections::{HashMap, HashSet};
use thiserror::Error;
use tonic::Status;

#[derive(Error, Debug)]
pub enum TopicManagerError {
    #[error("Node `{node_name}` is already subscribed to `{topic_name}`")]
    NodeAlreadySubscribed {
        node_name: String,
        topic_name: String,
    },
    #[error("Node `{node_name}` is already publishing to `{topic_name}`")]
    NodeAlreadyPublishing {
        node_name: String,
        topic_name: String,
    },
    #[error("Topic `{topic_name}` was requested with type `{requested_type}` but is already of  type `{real_type}`")]
    TopicTypeDoesNotMatch {
        topic_name: String,
        requested_type: String,
        real_type: String,
    },
    #[error("Topic `{0}` does not exist")]
    TopicDoesNotExist(String),
    #[error("Node `{publisher}` is not publishing on topic `{topic_name}`")]
    NotPublishing {
        publisher: String,
        topic_name: String,
    },
    #[error(
        "Channel from `{publisher}` to `{subscriber}` over topic `{topic_name}` already exists."
    )]
    ChannelAlreadyExists {
        publisher: String,
        subscriber: String,
        topic_name: String,
    },
    #[error(
        "Channel from `{publisher}` to `{subscriber}` over topic `{topic_name}` does not exist."
    )]
    ChannelDoesNotExist {
        publisher: String,
        subscriber: String,
        topic_name: String,
    },
}

#[derive(Debug, Clone)]
pub struct Topic {
    pub name: String,
    pub msg_type: String,
    pub publishers: HashSet<String>,  // HashSet of node names
    pub subscribers: HashSet<String>, // HashSet of node names
    pub channels: HashMap<(String, String), String>, // Map (pub, sub) -> channel name
}

impl Into<Status> for TopicManagerError {
    fn into(self) -> Status {
        Status::internal(format!("There was an unkown error: {:?}", self))
    }
}
