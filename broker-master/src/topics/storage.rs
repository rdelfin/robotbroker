use crate::{nodes::Node, topics::TopicManagerError};

pub trait TopicStorage {
    fn add_subscriber(
        &mut self,
        node: &Node,
        topic_name: &str,
        msg_type: &str,
    ) -> Result<(), TopicManagerError>;
    fn add_publisher(
        &mut self,
        node: &Node,
        topic_name: &str,
        msg_type: &str,
    ) -> Result<(), TopicManagerError>;
}
