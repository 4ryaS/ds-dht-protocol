use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Node {
    pub id: u64,
    pub ip: String,
    pub port: u16,
}

impl Node {
    // create a new node instance
    pub fn new(id: u64, ip: String, port: u16) -> Self {
        return Node {id, ip, port};
    }
}