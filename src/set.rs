use std::fmt::{Debug, Display};
use std::hash::Hash;

#[derive(Copy, Clone, PartialEq, Hash, Debug)]
#[allow(dead_code)]
pub enum SetError {
    Impossible
}

impl Eq for SetError {}

pub trait Set<T: Sized + Clone + Eq + Display + Debug>: Clone + Eq + Debug +
    Display + IntoIterator {
    fn new() -> Self;
    fn add(&mut self, elem: T) -> Result<(), SetError>;
    fn remove(&mut self, elem: T) -> Result<(), SetError>;
    fn contains(&self, elem: T) -> Result<bool, SetError>;
    fn size(&self) -> Result<usize, SetError>;
    fn clear(&mut self) -> Result<(), SetError>;
}

