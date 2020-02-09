use std::fmt::{Display, Debug};

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PriorityQueueError {
    OutOfBounds
}

pub trait PriorityQueue<T: Sized + Eq + Clone + Ord + Display + Debug>: Eq + 
    Clone + IntoIterator {
    fn new() -> Self;
    fn push(&mut self, elem: T) -> Result<(), PriorityQueueError>;
    fn pop(&mut self) -> Result<T, PriorityQueueError>;
    fn peek(&self) -> Result<&T, PriorityQueueError>;
    fn find(&self, elem: T) -> Result<Option<usize>, PriorityQueueError>;
    fn length(&self) -> Result<usize, PriorityQueueError>;
}

