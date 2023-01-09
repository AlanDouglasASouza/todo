use std::fmt::{Display, Formatter, Result};

pub struct Todo {
    pub message: String,
    pub resolved: bool
}

impl Display for Todo {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.message)
    }
}
