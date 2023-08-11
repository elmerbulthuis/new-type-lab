use std::fmt::Display;

#[derive(Debug)]
pub struct ValidationError {
    r#type: &'static str,
}

impl ValidationError {
    pub fn new(r#type: &'static str) -> Self {
        Self { r#type }
    }
}

impl Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "validation error for type {}", self.r#type)
    }
}
