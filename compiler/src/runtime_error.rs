#[allow(dead_code)]
#[derive(Debug)]
pub struct RuntimeError {
    pub message: String,
}


impl RuntimeError {
    pub fn new(msg: &str) -> Self {
        RuntimeError {
            message: msg.to_string(),
        }
    }
}
