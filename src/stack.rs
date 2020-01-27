#[derive(Clone, Copy, PartialEq, Debug)]
#[allow(dead_code)]
pub enum StackError {
    OutOfBounds,
    Impossible
}

pub trait Stack<T: Sized + Eq + Clone>: Eq + Clone {
    fn new() -> Self;
    fn push(&mut self, elem: T) -> Result<(), StackError>;
    fn pop(&mut self) -> Result<T, StackError>;
    fn peek(&self) -> Result<&T, StackError>;
    fn depth(&self) -> Result<usize, StackError>;
    fn clear(&mut self) -> Result<(), StackError>;
}

