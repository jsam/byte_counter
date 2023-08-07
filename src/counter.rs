use std::{iter::Step, mem};

use serde_derive::{Deserialize, Serialize};

use crate::timestamp::Timestamp;

pub const IDENTIFIER_SIZE: usize = mem::size_of::<u64>();

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ByteCounter {
    pub prefix: Option<String>,
    pub timestamp: Timestamp,
    pub id: [u8; IDENTIFIER_SIZE],

    #[serde(skip_serializing, skip_deserializing)]
    pub valid: bool,
}

impl Step for ByteCounter {
    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        let diff = end.to_u128() - start.to_u128();

        if diff > usize::MAX as u128 {
            return None;
        }
        Some(diff as usize)
    }

    fn forward_checked(start: Self, count: usize) -> Option<Self> {
        let mut result = start;
        for _ in 0..count {
            result = result.next_id();
        }
        Some(result)
    }

    fn backward_checked(start: Self, count: usize) -> Option<Self> {
        let mut result = start;
        for _ in 0..count {
            result = result.prev_id();
        }
        Some(result)
    }
}

impl ByteCounter {
    pub fn new() -> Self {
        ByteCounter::default()
    }

    pub fn new_with_prefix(prefix: String) -> Self {
        let mut id = ByteCounter::default();
        id.prefix = Some(prefix);
        id
    }

    pub fn decode_bytes(str_bytes: &str) -> Self {
        let byte_count = str_bytes.len() / 3;
        let _aligned = match byte_count.cmp(&IDENTIFIER_SIZE) {
            std::cmp::Ordering::Less => {
                let prefix_size = IDENTIFIER_SIZE - byte_count;
                let result = format!(
                    "{0}{1}",
                    String::from_utf8(vec![b'0'; prefix_size * 3]).unwrap(),
                    str_bytes
                );
                result
            }
            std::cmp::Ordering::Equal => str_bytes.to_string(),
            std::cmp::Ordering::Greater => {
                let start_idx = str_bytes.len() - (IDENTIFIER_SIZE * 3);
                let sl = str_bytes.as_bytes().to_owned();
                let slice = &sl[start_idx..];
                String::from_utf8(slice.to_vec()).unwrap()
            }
        };

        let __bytes = _aligned
            .chars()
            .collect::<Vec<char>>()
            .chunks(3)
            .map(|c| c.iter().collect::<String>().parse::<u8>().unwrap())
            .collect::<Vec<u8>>();

        let mut mem_id = [0x0_u8; IDENTIFIER_SIZE];
        mem_id.clone_from_slice(__bytes.as_ref());

        Self {
            prefix: None,
            timestamp: Timestamp::new(),
            id: mem_id,
            valid: true,
        }
    }
}

impl Default for ByteCounter {
    fn default() -> Self {
        let id = [0; IDENTIFIER_SIZE];

        Self {
            id,
            timestamp: Timestamp::new(),
            prefix: None,
            valid: true,
        }
    }
}

impl ToString for ByteCounter {
    fn to_string(&self) -> String {
        let _id = self
            .id
            .map(|b| {
                if b < 10 {
                    return format!("00{}", b);
                }

                if b < 100 {
                    return format!("0{}", b);
                }

                format!("{}", b)
            })
            .to_vec()
            .join("");

        if self.prefix.is_some() {
            return format!(
                "{}:{}:{}",
                self.prefix.clone().unwrap(),
                self.timestamp.value(),
                _id
            );
        }
        format!("{0}:{1}", self.timestamp.value(), _id)
    }
}

impl From<&String> for ByteCounter {
    fn from(key: &String) -> Self {
        let mut parts = key.split(':').collect::<Vec<&str>>();

        if parts.len() == 3 {
            let prefix = parts.remove(0);
            let timestamp = parts.remove(0);
            let id = parts.remove(0);

            let mut obj = ByteCounter::decode_bytes(id);
            obj.prefix = Some(prefix.to_string());
            obj.timestamp = Timestamp::from(timestamp);
            if obj.timestamp.value() == 0 {
                obj.valid = false;
            }

            return obj;
        }

        if parts.len() == 2 {
            let timestamp = parts.remove(0);
            let id = parts.remove(0);
            let mut obj = ByteCounter::decode_bytes(id);
            obj.timestamp = Timestamp::from(timestamp);
            if obj.timestamp.value() == 0 {
                obj.valid = false;
            }

            return obj;
        }

        return Self {
            prefix: None,
            timestamp: Timestamp::new(),
            id: [0; IDENTIFIER_SIZE],
            valid: false,
        };
    }
}

impl ByteCounter {
    pub fn max() -> Self {
        let id = [u8::MAX; IDENTIFIER_SIZE];

        Self {
            prefix: None,
            timestamp: Timestamp::new(),
            id,
            valid: true,
        }
    }

    pub fn max_with_prefix(prefix: String) -> Self {
        let id = [u8::MAX; IDENTIFIER_SIZE];

        Self {
            prefix: Some(prefix),
            timestamp: Timestamp::new(),
            id,
            valid: true,
        }
    }

    pub fn raw_value(&self) -> &[u8] {
        self.id.as_ref()
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.id.to_vec()
    }

    pub fn next_id(&self) -> Self {
        let mut next_id = self.id;
        for byte in next_id.iter_mut().rev() {
            if *byte == u8::MAX {
                *byte = 0
            } else {
                *byte += 1;
                break;
            }
        }

        Self {
            prefix: self.prefix.clone(),
            timestamp: Timestamp::new(),
            id: next_id,
            valid: true,
        }
    }

    pub fn prev_id(&self) -> Self {
        let mut next_id = self.id;
        for byte in next_id.iter_mut().rev() {
            if *byte == 0 {
                *byte = u8::MAX
            } else {
                *byte -= 1;
                break;
            }
        }

        Self {
            prefix: self.prefix.clone(),
            timestamp: Timestamp::new(),
            id: next_id,
            valid: true,
        }
    }

    pub fn to_u128(&self) -> u128 {
        let mut result: u128 = 0;
        for byte in self.id.iter().rev() {
            let deref = *byte as u128;
            if deref == 0 {
                break;
            }
            if result == 0 {
                result = deref;
                continue;
            } else {
                // TODO: Check for overflow
                result *= deref;
            }
        }

        result
    }

    pub fn distance(&self, other: &ByteCounter) -> u128 {
        let lhs = self.to_u128();
        let rhs = other.to_u128();

        if lhs < rhs {
            return rhs - lhs;
        }

        lhs - rhs
    }
}

#[cfg(test)]
mod tests {
    use crate::counter::{ByteCounter, IDENTIFIER_SIZE};

    #[test]
    fn test_stream_id() {
        {
            assert_eq!(IDENTIFIER_SIZE, 8);
        }

        {
            assert_eq!(ByteCounter::default().id, [0, 0, 0, 0, 0, 0, 0, 0]);
        }

        {
            let mut bid = ByteCounter::default();

            let next_bid = bid.next_id();
            assert_eq!(next_bid.id, [0, 0, 0, 0, 0, 0, 0, 1]);

            let next_next_bid = next_bid.next_id();

            assert_eq!(next_bid.id, [0, 0, 0, 0, 0, 0, 0, 1]);
            assert_eq!(next_next_bid.id, [0, 0, 0, 0, 0, 0, 0, 2]);

            for _ in 0..1e+6 as u64 {
                bid = bid.next_id();
            }

            assert_eq!(bid.id, [0, 0, 0, 0, 0, 15, 66, 64]);
        }

        {
            let mut bid = ByteCounter::new_with_prefix("stream".to_string());
            for _ in 0..1e+6 as u64 {
                bid = bid.next_id();
            }

            assert_eq!(
                format!("stream:{}:000000000000000015066064", bid.timestamp.value()),
                bid.to_string()
            );

            let _from = format!("stream:{}:000000000000000015066064", bid.timestamp.value());
            let _bid: ByteCounter = ByteCounter::from(&_from);
            assert_eq!(bid.to_string(), _bid.to_string());
            assert_eq!(Some("stream".to_string()), bid.prefix);
            assert_eq!(bid.timestamp.value(), _bid.timestamp.value());
        }
    }

    #[test]
    fn test_distance() {
        {
            let default = ByteCounter::default();
            let nextnext = default.next_id().next_id();

            assert_eq!(default.to_u128(), 0);
            assert_eq!(nextnext.to_u128(), 2);
            assert_eq!(default.distance(&nextnext), 2);
        }

        {
            let default = ByteCounter::default();
            let mut default2 = ByteCounter::default();

            default2 = default2.next_id();
            default2 = default2.next_id();
            default2 = default2.next_id();
            default2 = default2.next_id();

            assert_eq!(default2.distance(&default), 4);
            assert_eq!(default.distance(&default2), 4);
        }
    }
}
