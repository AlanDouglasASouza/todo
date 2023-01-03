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
    fn get_one_todo(&self, key: u32) -> Option<&Todo>;
    fn remove(&mut self, key: u32);
    fn len(&self) -> usize;
    fn get_collection(&self) -> &BTreeMap<u32, Todo>;
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

    fn get_one_todo(&self, key: u32) -> Option<&Todo> {
        self.todo_collection.get(&key)
    }

    fn remove(&mut self, key: u32) {
        self.todo_collection.remove(&key);
    }

    fn len(&self) -> usize {
        self.todo_collection.len()
    }

    fn get_collection(&self) -> &BTreeMap<u32, Todo> {
        &self.todo_collection
    }
}
