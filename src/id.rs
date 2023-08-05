use std::mem;

use serde_derive::{Deserialize, Serialize};

pub const IDENTIFIER_SIZE: usize = mem::size_of::<u128>();

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct BigID {
    pub id: [u8; IDENTIFIER_SIZE],
    pub prefix: Option<String>,

    #[serde(skip_serializing, skip_deserializing)]
    pub valid: bool,
}

impl BigID {
    pub fn new(prefix: Option<String>) -> Self {
        let id = [0; IDENTIFIER_SIZE];

        Self {
            id,
            prefix,
            valid: true,
        }
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
            id: mem_id,
            valid: true,
            prefix: None,
        }
    }
}

impl Default for BigID {
    fn default() -> Self {
        let id = [0; IDENTIFIER_SIZE];

        Self {
            id,
            prefix: None,
            valid: true,
        }
    }
}

impl ToString for BigID {
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
            return format!("{}:{}", self.prefix.clone().unwrap(), _id);
        }
        format!("{0}", _id)
    }
}

impl From<&str> for BigID {
    fn from(key: &str) -> Self {
        let mut parts = key.split(':').collect::<Vec<&str>>();

        if parts.len() == 2 {
            let prefix = parts.remove(0);
            let id = parts.remove(0);
            let mut obj = BigID::decode_bytes(id);
            obj.prefix = Some(prefix.to_string());
            return obj;
        }

        if parts.len() == 1 {
            let id = parts.remove(0);
            return BigID::decode_bytes(id);
        }

        return Self {
            id: [0; IDENTIFIER_SIZE],
            valid: false,
            prefix: None,
        };
    }
}

impl From<Box<[u8]>> for BigID {
    fn from(key: Box<[u8]>) -> Self {
        return BigID::from(String::from_utf8_lossy(&key).as_ref());
    }
}

impl BigID {
    pub fn raw_value(&self) -> &[u8] {
        self.id.as_ref()
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.id.to_vec()
    }

    pub fn next(&self) -> Self {
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
            id: next_id,
            valid: true,
            prefix: self.prefix.clone(),
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
                result *= deref;
            }
        }

        result
    }

    pub fn distance(&self, other: &BigID) -> u128 {
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
    use crate::id::{BigID, IDENTIFIER_SIZE};

    #[test]
    fn test_stream_id() {
        {
            assert_eq!(IDENTIFIER_SIZE, 16);
        }

        {
            assert_eq!(
                BigID::default().id,
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            );
        }

        {
            let mut bid = BigID::default();

            let next_bid = bid.next();
            assert_eq!(
                next_bid.id,
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]
            );

            let next_next_bid = next_bid.next();

            assert_eq!(
                next_bid.id,
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]
            );
            assert_eq!(
                next_next_bid.id,
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2]
            );

            for _ in 0..1e+6 as u64 {
                bid = bid.next();
            }

            assert_eq!(bid.id, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 66, 64]);
        }

        {
            let mut bid = BigID::new(Some("stream".to_string()));
            for _ in 0..1e+6 as u64 {
                bid = bid.next();
            }

            assert_eq!(
                "stream:000000000000000000000000000000000000000015066064",
                bid.to_string()
            );

            let _bid: BigID =
                BigID::from("stream:000000000000000000000000000000000000000015066064");
            assert_eq!(bid.to_string(), _bid.to_string());
            assert_eq!(Some("stream".to_string()), bid.prefix);
        }
    }

    #[test]
    fn test_distance() {
        {
            let default = BigID::default();
            let nextnext = default.next().next();

            assert_eq!(default.to_u128(), 0);
            assert_eq!(nextnext.to_u128(), 2);
            assert_eq!(default.distance(&nextnext), 2);
        }

        {
            let default = BigID::default();
            let mut default2 = BigID::default();

            default2 = default2.next();
            default2 = default2.next();
            default2 = default2.next();
            default2 = default2.next();

            assert_eq!(default2.distance(&default), 4);
            assert_eq!(default.distance(&default2), 4);
        }
    }
}
