use crate::prelude::*;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    /// Node public key
    pub public_key: String,
    /// Node alias
    pub alias: String,
    /// Capacity in satoshis
    pub capacity: i64,
    /// First seen timestamp in unix time
    pub first_seen: i64,
    /// Last updated timestamp in unix time
    pub updated_at: i64,
}

impl Node {
    pub async fn get() -> Result<Vec<Node>> {
        let client = reqwest::Client::new();
        let response = client.get("https://mempool.space/api/v1/lightning/nodes/rankings/connectivity").send().await?;
        Ok(response.json().await?)
    } 
}