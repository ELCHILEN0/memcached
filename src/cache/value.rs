#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Value {
    // TODO: Vec<u8>
    pub item: String,
}

impl Value {
    pub fn new(item: String) -> Value {
        Value { item: item }
    }

    pub fn len(&self) -> usize {
        self.item.len()
    }
}