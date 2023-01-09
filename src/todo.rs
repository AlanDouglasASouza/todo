use std::fmt::{Display, Formatter, Result};

pub struct Todo {
    pub message: String,
    pub resolved: bool,
}

impl Todo {
    pub fn resolve(&self) -> Self {
        Self {
            message: self.message.clone(),
            resolved: true,
        }
    }
}

impl Display for Todo {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.message)
    }
}
