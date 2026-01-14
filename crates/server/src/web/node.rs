use crate::prelude::*;

#[derive(Serialize)]
pub struct Node {
    pub public_key: String,
    pub alias: String,
    pub capacity: String,
    pub first_seen: String
}

impl From<crate::data::mempool::Node> for Node {
    fn from(value: crate::data::mempool::Node) -> Self {
        Self {
            public_key: value.public_key,
            alias: value.alias,
            capacity: value.capacity.to_string(),
            first_seen: value.first_seen.to_string()
        }
    }
}