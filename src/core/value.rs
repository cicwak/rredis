#[derive(Debug, Clone, PartialEq)]
pub struct Value {
    pub data: String,
    pub ttl: i32,
}

impl Value {
    pub fn new(data: String, ttl: Option<i32>) -> Self {
        Self {
            data,
            ttl: ttl.unwrap_or(-1),
        }
    }

    pub fn data(&self) -> &String {
        &self.data
    }

    pub fn ttl(&self) -> i32 {
        self.ttl
    }
}
