#[derive(Clone, Copy, PartialEq, Debug)]

#[allow(dead_code)]
pub enum ListError {
    OutOfBounds,
    Impossible,
}

pub trait List<T: Sized + Eq + Clone>: IntoIterator + Eq + Clone  {
    fn new() -> Self;
    fn get(&self, pos: usize) -> Result<&T, ListError>;
    fn get_mut(&mut self, pos: usize) -> Result<&mut T, ListError>;
    fn set(&mut self, pos: usize, elem: T) -> Result<(), ListError>;
    fn insert(&mut self, pos: usize, elem: T) -> Result<(), ListError>;
    fn remove(&mut self, pos: usize) -> Result<T, ListError>;
    fn length(&self) -> Result<usize, ListError>;
    fn append(&mut self, elem: T) -> Result<(), ListError>;
    fn swap(&mut self, a: usize, b: usize) -> Result<(), ListError>;
    fn contains(&self, elem: T) -> Result<bool, ListError>;
    fn find_all(&self, elem: T) -> Result<Option<Vec<usize>>, ListError>;
    fn find(&self, elem: T) -> Result<Option<usize>, ListError>;
    fn count(&self, elem: T) -> Result<usize, ListError>;
    fn clear(&mut self) -> Result<(), ListError>;
}

