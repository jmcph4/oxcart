#[derive(Clone, Copy, PartialEq, Debug)]
#[allow(dead_code)]
pub enum MapError {
    KeyNotFound,
}

pub trait Map<K: Sized + Eq + Clone, V: Sized + Eq + Clone>: IntoIterator +
    Eq + Clone {
    fn new() -> Self;
    fn get(&self, key: K) -> Result<&V, MapError>;
    fn get_mut(&mut self, key: K) -> Result<&mut V, MapError>;
    fn set(&mut self, key: K, value: V) -> Result<(), MapError>;
    fn remove(&mut self, key: K) -> Result<(), MapError>;
    fn size(&self) -> Result<usize, MapError>;
    fn contains_key(&self, key: K) -> Result<bool, MapError>;
    fn contains_value(&self, value: V) -> Result<bool, MapError>;
    fn clear(&mut self) -> Result<(), MapError>;
}

