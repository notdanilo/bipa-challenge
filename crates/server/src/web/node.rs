use serde::{Serialize, Serializer};
use chrono::{DateTime, SecondsFormat, Utc};

#[derive(Serialize)]
pub struct Node {
    pub public_key: String,
    pub alias: String,
    #[serde(serialize_with = "serialize_btc")]
    pub capacity: i64,
    #[serde(serialize_with = "serialize_timestamp")]
    pub first_seen: i64
}

impl From<crate::importer::mempool::Node> for Node {
    fn from(value: crate::importer::mempool::Node) -> Self {
        Self {
            public_key: value.public_key,
            alias: value.alias,
            capacity: value.capacity,
            first_seen: value.first_seen
        }
    }
}

fn serialize_btc<S>(sats: &i64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let sats = *sats;

    let btc = sats / 100_000_000;
    let remainder = sats % 100_000_000;

    let formatted = format!("{}.{}", btc, format!("{:08}", remainder.abs()));

    serializer.serialize_str(&formatted)
}

pub fn serialize_timestamp<S>(
    unix_seconds: &i64,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let dt = DateTime::<Utc>::from_timestamp(*unix_seconds, 0)
        .ok_or_else(|| serde::ser::Error::custom("invalid unix timestamp"))?;

    let formatted = dt.to_rfc3339_opts(SecondsFormat::Secs, true);
    serializer.serialize_str(&formatted)
}
