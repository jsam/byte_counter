use std::marker::PhantomData;
use std::ops::RangeFrom;

pub trait Gen {
    type Output;

    fn next(&mut self) -> Option<Self::Output>;
}

#[derive(
    Clone,
    Debug, 
)]
pub struct Generator<T> {
    generator: RangeFrom<u128>,
    marker: PhantomData<T>,
}

impl<T: From<u128>> Generator<T> {
    pub fn new() -> Self {
        Generator {
            generator: 0..,
            marker: PhantomData,
        }
    }
}

impl<T: From<u128>> Gen for Generator<T> {
    type Output = T;

    fn next(&mut self) -> Option<Self::Output> {
        self.generator.next().map(|i| i.into())
    }
}

impl<T: From<u128>> Default for Generator<T> {
    fn default() -> Self {
        Generator::new()
    }
}
