use crate::terminal::{Terminal, TerminalError, UserInterface};
use crate::todo::Todo;
use std::collections::BTreeMap;

pub struct Todos {
    todo_collection: BTreeMap<u32, Todo>,
    length: u32,
}

impl Todos {
    pub fn new() -> Self {
        Self {
            todo_collection: BTreeMap::<u32, Todo>::new(),
            length: 0,
        }
    }
}

pub trait TodoStorage {
    fn insert_todo(&mut self, todo: Todo);
    fn update(&mut self, id: u32, new_todo: Todo) -> bool;
    fn show_all_todos(&self, show_keys: bool) -> Result<(), TerminalError>;
    fn get_one_todo(&self, key: u32) -> Option<&Todo>;
    fn remove(&mut self, key: u32);
    fn len(&self) -> usize;
}

impl TodoStorage for Todos {
    fn insert_todo(&mut self, todo: Todo) {
        self.length += 1;
        self.todo_collection.entry(self.length).or_insert(todo);
    }

    fn update(&mut self, id: u32, new_todo: Todo) -> bool {
        if self.todo_collection.contains_key(&id) {
            self.todo_collection.insert(id, new_todo);
            return true;
        }
        false
    }

    fn show_all_todos(&self, show_keys: bool) -> Result<(), TerminalError> {
        let terminal = Terminal::new();

        for (key, todo) in &self.todo_collection {
            if show_keys {
                terminal.show_todo(todo, format!("{key}: ").as_str())?;
            } else {
                terminal.show_todo(todo, "✅: ")?;
            }
        }

        Ok(())
    }

    fn get_one_todo(&self, key: u32) -> Option<&Todo> {
        self.todo_collection.get(&key)
    }

    fn remove(&mut self, key: u32) {
        self.todo_collection.remove(&key);
    }

    fn len(&self) -> usize {
        self.todo_collection.len()
    }
}
