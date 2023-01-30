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
    async fn insert_todo(&mut self, todo: Todo) -> Result<(), TerminalError>;
    async fn update(&mut self, id: u32, new_todo: Todo) -> Result<bool, TerminalError>;
    fn get_one_todo(&self, key: u32) -> Option<&Todo>;
    async fn remove(&mut self, key: u32) -> Result<(), TerminalError>;
    fn is_empty(&self) -> usize;
    fn get_collection(&self) -> &BTreeMap<u32, Todo>;
    async fn resolve_one_todo(&mut self, key: u32) -> Result<bool, TerminalError>;
    async fn parse_file_for_todos(&mut self) -> Result<(), TerminalError>;
    fn parse_line_for_todo(&mut self, line: &str) -> Result<(u32, String, bool), TerminalError>;
    async fn parse_map_write_file(&mut self) -> Result<(), TerminalError>;
}

#[async_trait::async_trait]
impl TodoStorage for Todos {
    async fn insert_todo(&mut self, todo: Todo) -> Result<(), TerminalError> {
        self.length += 1;
        self.todo_collection.entry(self.length).or_insert(todo);
        self.parse_map_write_file().await?;
        Ok(())
    }

    async fn update(&mut self, id: u32, new_todo: Todo) -> Result<bool, TerminalError> {
        if self.todo_collection.contains_key(&id) {
            self.todo_collection.insert(id, new_todo);
            self.parse_map_write_file().await?;
            return Ok(true);
        }
        Ok(false)
    }

    fn get_one_todo(&self, key: u32) -> Option<&Todo> {
        self.todo_collection.get(&key)
    }

    async fn remove(&mut self, key: u32) -> Result<(), TerminalError> {
        self.todo_collection.remove(&key);
        self.parse_map_write_file().await?;
        Ok(())
    }

    fn is_empty(&self) -> usize {
        self.todo_collection.len()
    }

    fn get_collection(&self) -> &BTreeMap<u32, Todo> {
        &self.todo_collection
    }

    async fn resolve_one_todo(&mut self, key: u32) -> Result<bool, TerminalError> {
        match self.todo_collection.get(&key) {
            Some(todo) => self.todo_collection.insert(
                key,
                Todo {
                    message: todo.message.clone(),
                    resolved: true,
                },
            ),
            None => return Ok(false),
        };

        self.parse_map_write_file().await?;
        Ok(true)
    }

    async fn parse_file_for_todos(&mut self) -> Result<(), TerminalError> {
        let todo_file = read_to_string("todo_list.txt")
            .await
            .map_err(TerminalError::StdinErr)?;

        for line in todo_file.lines() {
            let (key, todo_message, resolve) = self.parse_line_for_todo(line)?;
            self.todo_collection.entry(key).or_insert(Todo {
                message: todo_message,
                resolved: resolve,
            });
        }

        self.length = match self
            .todo_collection
            .keys()
            .cloned()
            .collect::<Vec<u32>>()
            .pop()
        {
            Some(key) => key,
            None => 0,
        };
        Ok(())
    }

    fn parse_line_for_todo(&mut self, line: &str) -> Result<(u32, String, bool), TerminalError> {
        let mut text_slice: Vec<&str> = line.split("-").collect();
        let resolve = match text_slice.remove(1) {
            "true" => true,
            _ => false,
        };
        let key: u32 = text_slice
            .remove(0)
            .parse()
            .map_err(TerminalError::ParseErr)?;
        let message = text_slice
            .iter()
            .map(|slice| format!("{slice}\n"))
            .collect::<Vec<String>>()
            .join("-");

        Ok((key, message, resolve))
    }

    async fn parse_map_write_file(&mut self) -> Result<(), TerminalError> {
        let todo_string = self
            .todo_collection
            .iter()
            .map(|(key, todo)| {
                format!("{key}-{}-{}", todo.resolved, todo.message.replace("\n", ""))
            })
            .collect::<Vec<String>>()
            .join("\n");

        write("todo_list.txt", todo_string.as_bytes())
            .await
            .map_err(TerminalError::StdoutErr)?;
        Ok(())
    }
}
