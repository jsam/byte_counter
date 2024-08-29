use crate::counter::ByteCounter;

#[derive(Clone, Debug)]
pub struct Generator {
    current: ByteCounter,
    end: ByteCounter,
}

impl Generator {
    pub fn new() -> Self {
        Generator {
            current: ByteCounter::new(),
            end: ByteCounter::new(),
        }
    }

    pub fn new_with_prefix(prefix: String) -> Self {
        Generator {
            current: ByteCounter::new_with_prefix(prefix.clone()),
            end: ByteCounter::new_with_prefix(prefix.clone()),
        }
    }
}

impl Default for Generator {
    fn default() -> Self {
        Generator::new()
    }
}

impl Iterator for Generator {
    type Item = ByteCounter;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            let result = self.current.clone();
            self.current = self.current.next_id();
            Some(result)
        } else {
            None
        }
    }
}

impl DoubleEndedIterator for Generator {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            let result = self.end.clone();
            self.end = self.end.prev_id();
            Some(result)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::counter::ByteCounter;
    use crate::generator::Generator;
    use rand::distributions::{Alphanumeric, DistString};

    #[test]
    fn test_generator() {
        let prefix: String = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);

        {
            let generator = Generator::new();
            let mut expected = ByteCounter::default();
            for item in generator.take(1000) {
                assert_eq!(item, expected);
                assert_eq!(item.prefix, None);
                expected = expected.next_id();
            }
        }

        {
            let generator = Generator::new();
            let mut expected = ByteCounter::max();
            for item in generator.rev().take(100) {
                assert_eq!(item, expected);
                assert_eq!(item.prefix, None);
                expected = expected.prev_id();
            }
        }

        {
            let generator = Generator::new_with_prefix(prefix.clone());
            let mut expected = ByteCounter::new_with_prefix(prefix.clone());
            for item in generator.take(100) {
                assert_eq!(item, expected);
                assert_eq!(item.prefix, Some(prefix.clone()));
                expected = expected.next_id();
            }
        }

        {
            let generator = Generator::new_with_prefix(prefix.clone());
            let mut expected = ByteCounter::max_with_prefix(prefix.clone());
            for item in generator.rev().take(100) {
                println!("{:?}", item.to_string());
                assert_eq!(item, expected);
                assert_eq!(item.prefix, Some(prefix.clone()));
                expected = expected.prev_id();
            }
        }
    }
}
