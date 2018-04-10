#[macro_use]
extern crate binary_macros;

use std::fmt::Debug;

#[derive(Debug)]
pub struct KvError(pub String);

pub trait KvStorage
where
    Self: Sync + Send + Debug,
{
    fn get(&self, k: &str) -> Result<Option<String>, KvError>;
}

impl<K> KvStorage for Box<K>
where
    K: KvStorage + Sync + Send + Debug + ?Sized,
{
    fn get(&self, k: &str) -> Result<Option<String>, KvError> {
        (**self).get(k)
    }
}
