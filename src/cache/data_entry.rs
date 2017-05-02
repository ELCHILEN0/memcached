use cache::key::Key;
use cache::value::Value;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct DataEntry {
    pub key: Key,
    pub value: Value
}

impl DataEntry {
    pub fn new(key: Key, value: Value) -> DataEntry {
        DataEntry { 
            key: key,
            value: value
         }
    }

    pub fn len(&self) -> usize {
        self.key.len() + self.value.len()
    }
}