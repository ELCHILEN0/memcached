#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Value {
    // TODO: Vec<u8>
    pub item: String,
    pub cas: u64,
}

impl Value {
    pub fn new(item: String) -> Value {
        Value { 
            item: item, 
            cas: 0 
        }
    }

    pub fn inc_cas(&mut self) {
        self.cas += 1;
    }

    pub fn len(&self) -> usize {
        self.item.len()
    }
}