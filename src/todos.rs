use crate::terminal::{Terminal, TerminalError};
use crate::todo::Todo;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Todos {
    todo_collection: BTreeMap<u32, Todo>,
    length: u32,
}

impl Todos {
    pub fn new() -> Self {
        Todos {
            todo_collection: BTreeMap::<u32, Todo>::new(),
            length: 0,
        }
    }

    pub fn insert_todo(&mut self, todo: Todo) {
        self.length += 1;
        self.todo_collection.entry(self.length).or_insert(todo);
    }

    pub fn update(&mut self, id: u32, new_todo: Todo) -> bool {
        if self.todo_collection.contains_key(&id) {
            self.todo_collection.insert(id, new_todo);
            return true;
        }

        false
    }

    pub fn show_all_todos(&mut self, show_keys: bool) -> Result<(), TerminalError> {
        let mut terminal = Terminal::new();

        for (key, todo) in &self.todo_collection {
            if show_keys {
                terminal.show_todo(todo, format!("{key}: ").as_str())?;
            } else {
                terminal.show_todo(todo, "✅: ")?;
            }
        }

        Ok(())
    }

    pub fn get_one_todo(&mut self, key: u32) -> Option<&Todo> {
        self.todo_collection.get(&key)
    }

    pub fn remove(&mut self, key: u32) {
        self.todo_collection.remove(&key);
    }

    pub fn len(&mut self) -> usize {
        self.todo_collection.len()
    }
}
