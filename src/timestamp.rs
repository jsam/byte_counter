use serde_derive::{Deserialize, Serialize};
use std::hash::Hash;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn epoch_ns() -> u128 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(time) => time.as_nanos(),
        Err(_) => panic!("Unable to determine time."),
    }
}

pub fn epoch_secs() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(time) => time.as_secs(),
        Err(_) => panic!("Unable to determine time."),
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Timestamp(u64);

impl Timestamp {
    pub fn new() -> Self {
        Self(epoch_secs())
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self::new()
    }
}

impl From<&str> for Timestamp {
    fn from(s: &str) -> Self {
        let value = s.parse::<u64>().unwrap_or_else(|_| 0);
        Self(value)
    }
}
