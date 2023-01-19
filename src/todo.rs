use std::fmt::{Display, Formatter, Result};

pub struct Todo {
    pub message: String,
    pub resolved: bool,
}

impl Todo {
    pub fn new(todo: String) -> Self {
        Self {
            message: todo,
            resolved: false,
        }
    }
}

impl Display for Todo {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.message)
    }
}
