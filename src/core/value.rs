#[derive(Debug, Clone, PartialEq)]
pub struct Value {
    pub data: String,
    pub ttl: i32,
}

impl Value {
    pub fn data(&self) -> &String {
        &self.data
    }

    pub fn ttl(&self) -> i32 {
        self.ttl
    }
}
