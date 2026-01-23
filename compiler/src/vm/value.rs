#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    String(String),
    Void,
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Int(v) => *v != 0,
            _ => false,
        }
    }
}
