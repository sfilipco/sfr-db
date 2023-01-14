use std::collections::HashMap;

pub struct Hash {
    underlying: HashMap<Vec<u8>, Vec<u8>>,
}

#[derive(Debug, thiserror::Error, Eq, PartialEq)]
pub enum InsertErrorKind {
    #[error("unknown insert error")]
    Unknown,
}
#[derive(Debug, thiserror::Error, Eq, PartialEq)]
pub enum GetErrorKind {
    #[error("unknown get error")]
    Unknown,
}
#[derive(Debug, thiserror::Error, Eq, PartialEq)]
pub enum RemoveErrorKind {
    #[error("unknown remove error")]
    Unknown,
}

impl Hash {
    pub fn new() -> Self {
        Self {
            underlying: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: Vec<u8>, value: Vec<u8>) -> Result<(), InsertErrorKind> {
        self.underlying.insert(key, value);
        Ok(())
    }

    pub fn get(&mut self, key: Vec<u8>) -> Result<Option<Vec<u8>>, GetErrorKind> {
        Ok(self.underlying.get(&key).map(|v| v.to_owned()))
    }

    pub fn remove(&mut self, key: Vec<u8>) -> Result<Option<Vec<u8>>, RemoveErrorKind> {
        Ok(self.underlying.remove(&key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut hash = Hash::new();
        let k = vec![1, 2, 3];
        let v1 = vec![4, 5];
        let v2 = vec![6, 7];
        assert_eq!(hash.get(k.clone()), Ok(None));
        assert_eq!(hash.insert(k.clone(), v1.clone()), Ok(()));
        assert_eq!(hash.get(k.clone()), Ok(Some(v1)));
        assert_eq!(hash.insert(k.clone(), v2.clone()), Ok(()));
        assert_eq!(hash.get(k.clone()), Ok(Some(v2.clone())));
        assert_eq!(hash.remove(k.clone()), Ok(Some(v2)));
        assert_eq!(hash.get(k.clone()), Ok(None));
        assert_eq!(hash.remove(k.clone()), Ok(None));
    }
}
