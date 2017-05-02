#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Key {
    // TODO: Vec<u8>
    pub item: String,
}

impl Key {
    pub fn new(item: String) -> Key {
        Key { item: item }
    }

    pub fn len(&self) -> usize {
        self.item.len()
    }
}