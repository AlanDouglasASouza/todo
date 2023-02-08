use crate::terminal::TerminalError;
use crate::todo::Todo;
use std::collections::BTreeMap;
use tokio::fs::{read_to_string, write};

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

#[async_trait::async_trait]
pub trait TodoStorage {
    fn insert_todo(&mut self, todo: Todo);
    fn update(&mut self, id: u32, new_todo: Todo) -> bool;
    fn get_one_todo(&self, key: u32) -> Option<&Todo>;
    fn remove(&mut self, key: u32);
    fn is_empty(&self) -> usize;
    fn get_collection(&self) -> &BTreeMap<u32, Todo>;
    fn resolve_one_todo(&mut self, key: u32) -> bool;
    async fn parse_file_for_todos(&mut self) -> Result<(), TerminalError>;
    fn parse_line_for_todo(&mut self, line: &str) -> Result<(u32, String, bool), TerminalError>;
    async fn parse_map_write_file(&mut self) -> Result<(), TerminalError>;
}

#[async_trait::async_trait]
impl TodoStorage for Todos {
    fn insert_todo(&mut self, todo: Todo) {
        self.length += 1;
        self.todo_collection.entry(self.length).or_insert(todo);
    }

    fn update(&mut self, id: u32, new_todo: Todo) -> bool {
        if let std::collections::btree_map::Entry::Occupied(mut e) = self.todo_collection.entry(id)
        {
            e.insert(new_todo);
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

    fn is_empty(&self) -> usize {
        self.todo_collection.len()
    }

    fn get_collection(&self) -> &BTreeMap<u32, Todo> {
        &self.todo_collection
    }

    fn resolve_one_todo(&mut self, key: u32) -> bool {
        let Some(todo) = self.todo_collection.get_mut(&key) else {
            return false;
        };
        todo.resolved = true;
        true
    }

    async fn parse_file_for_todos(&mut self) -> Result<(), TerminalError> {
        let todo_file = read_to_string("todo_list.txt")
            .await
            .map_err(TerminalError::StdinErr)?;

        for line in todo_file.lines() {
            let (key, todo_message, resolve) = self.parse_line_for_todo(line)?;
            self.todo_collection.entry(key).or_insert(Todo {
                message: format!("{todo_message}\n"),
                resolved: resolve,
            });
        }

        self.length = self
            .todo_collection
            .keys()
            .cloned()
            .collect::<Vec<u32>>()
            .pop()
            .unwrap_or(0);
        Ok(())
    }

    fn parse_line_for_todo(&mut self, line: &str) -> Result<(u32, String, bool), TerminalError> {
        let mut text_slice = line.split('-');
        let key: u32 = text_slice
            .next()
            .ok_or_else(|| {
                TerminalError::NotFound("Erro no parse_line [key not found]".to_string())
            })?
            .parse()
            .map_err(TerminalError::ParseErr)?;

        let resolve = matches!(
            text_slice.next().ok_or_else(|| TerminalError::NotFound(
                "Erro no parse_line [resolve not found]".to_string()
            ))?,
            "true"
        );

        let message = text_slice.collect::<Vec<&str>>().join("-");

        Ok((key, message, resolve))
    }

    async fn parse_map_write_file(&mut self) -> Result<(), TerminalError> {
        let todo_string = self
            .todo_collection
            .iter()
            .map(|(key, todo)| format!("{key}-{}-{}", todo.resolved, todo.message))
            .collect::<Vec<String>>()
            .join("");

        write("todo_list.txt", todo_string.as_bytes())
            .await
            .map_err(TerminalError::StdoutErr)?;
        Ok(())
    }
}
