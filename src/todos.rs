use crate::terminal::{Terminal, TerminalError};
use crate::todo::Todo;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Todos {
    all_todos: HashMap<u8, Todo>,
    length: u8,
}

impl Todos {
    pub fn new() -> Self {
        Todos {
            all_todos: HashMap::<u8, Todo>::new(),
            length: 0,
        }
    }

    pub fn insert_todo(&mut self, todo: Todo) {
        self.length += 1;
        self.all_todos.entry(self.length).or_insert(todo);
    }

    pub fn update(&mut self, id: u8, new_todo: Todo) -> bool {
        if let Some(_) = self.all_todos.get(&id) {
            self.all_todos.insert(id, new_todo);
            true
        } else {
            false
        }
    }

    pub fn show_all_todos(&mut self, show_keys: bool) -> Result<(), TerminalError> {
        let mut terminal = Terminal::new();
        let todos = &self.all_todos;
        let mut keys: Vec<&u8> = todos.keys().collect();
        keys.sort();

        for key in keys {
            if let Some(message) = todos.get(key) {
                if show_keys {
                    terminal.show_todo(message, format!("{key}: ").as_str())?;
                } else {
                    terminal.show_todo(message, "✅: ")?;
                }
            }
        }
        Ok(())
    }

    pub fn get_one_todo(&mut self, key: u8) -> Option<&Todo> {
        self.all_todos.get(&key)
    }

    pub fn remove(&mut self, key: u8) {
        self.all_todos.remove(&key);
    }
}
