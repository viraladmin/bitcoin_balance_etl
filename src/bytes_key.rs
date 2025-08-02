use db_key::Key;
use std::cmp::Ordering;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct BytesKey(pub Vec<u8>);

impl Ord for BytesKey {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for BytesKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Key for BytesKey {
    fn from_u8(key: &[u8]) -> Self {
        BytesKey(key.to_vec())
    }

    fn as_slice<T, F: FnOnce(&[u8]) -> T>(&self, f: F) -> T {
        f(&self.0)
    }
}
