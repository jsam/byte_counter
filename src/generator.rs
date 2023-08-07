use std::ops::Range;

use crate::counter::ByteCounter;

#[derive(
    Clone,
    Debug, 
)]
pub struct Generator<'a> {
    generator: Range<ByteCounter<'a>>,
}

impl<'a> Generator<'a> {
    pub fn new() -> Self {
        Generator {
            generator: Range { 
                start: ByteCounter::new(), 
                end: ByteCounter::new(),
            },
        }
    }
    
    pub fn new_with_prefix(prefix: &'a str) -> Self {
        Generator {
            generator: Range { 
                start: ByteCounter::new_with_prefix(prefix), 
                end: ByteCounter::new_with_prefix(prefix) 
            },
        }
    }
}

impl<'a> Default for Generator<'a> {
    fn default() -> Self {
        Generator::new()
    }
}

impl<'a> Iterator for Generator<'a> {
    type Item = ByteCounter<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.generator.next().map(|i| i.into())
    }
}

impl<'a> DoubleEndedIterator for Generator<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.generator.next_back().map(|i| i.into())
    }
}


#[cfg(test)]
mod tests {
    use rand::distributions::{Alphanumeric, DistString};
    use crate::generator::Generator;
    use crate::counter::ByteCounter;

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
            let generator = Generator::new_with_prefix(&prefix);
            let mut expected = ByteCounter::new_with_prefix(&prefix);
            for item in generator.take(100) {
                assert_eq!(item, expected);
                assert_eq!(item.prefix, Some(prefix.as_ref()));
                expected = expected.next_id();
            }
        }

        {
            let generator = Generator::new_with_prefix(&prefix);
            let mut expected = ByteCounter::max_with_prefix(&prefix);
            for item in generator.rev().take(100) {
                println!("{:?}", item.to_string());
                assert_eq!(item, expected);
                assert_eq!(item.prefix, Some(prefix.as_ref()));
                expected = expected.prev_id();
            }
        }
    }

}