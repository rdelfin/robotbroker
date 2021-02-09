use crate::nodes::Node;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TopicManagerError {}


#[derive(Debug, Clone)]
pub enum Topic {
    pub name: String,
    pub msg_type: String,
    pub publishers: Vec<Node>,
    pub subscribers: Vec<Node>,
    pub channels: HashMap<(String, String), String>,
}

impl Into<Status> for TopicManagerError {
    fn into(self) -> Status {
            Status::internal(format!("There was an unkown error", self))
        }
    }
}
