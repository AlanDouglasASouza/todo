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

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait TodoStorage {
    fn insert_todo(&mut self, todo: Todo);
    fn update(&mut self, id: u32, new_todo: Todo) -> bool;
    fn get_one_todo(&self, key: u32) -> Option<Todo>;
    fn remove(&mut self, key: u32);
    fn is_empty(&self) -> usize;
    fn get_collection(&self) -> &BTreeMap<u32, Todo>;
    fn resolve_one_todo(&mut self, key: u32) -> bool;
    async fn parse_file_for_todos(&mut self, path: &str) -> Result<(), TerminalError>;
    fn parse_line_for_todo(&mut self, line: &str) -> Result<(u32, String, bool), TerminalError>;
    async fn parse_map_write_file(&mut self, path: &str) -> Result<(), TerminalError>;
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

    fn get_one_todo(&self, key: u32) -> Option<Todo> {
        self.todo_collection.get(&key).cloned()
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

    async fn parse_file_for_todos(&mut self, path: &str) -> Result<(), TerminalError> {
        let todo_file = read_to_string(path)
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

    async fn parse_map_write_file(&mut self, path: &str) -> Result<(), TerminalError> {
        let todo_string = self
            .todo_collection
            .iter()
            .map(|(key, todo)| format!("{key}-{}-{}", todo.resolved, todo.message))
            .collect::<Vec<String>>()
            .join("");

        write(path, todo_string.as_bytes())
            .await
            .map_err(TerminalError::StdoutErr)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::todo::mocks::_Factori_Builder_Todo;
    use crate::todos::tests::mocks::_Factori_Builder_Todos;

    pub mod mocks {
        use super::*;
        use std::collections::BTreeMap;

        fn create_collection() -> BTreeMap<u32, Todo> {
            let mut list = BTreeMap::<u32, Todo>::new();
            list.entry(1).or_insert(factori::create!(Todo));
            list.entry(2).or_insert(Todo::new("boo".to_string()));
            list
        }

        factori::factori!(Todos, {
            default {
                todo_collection = create_collection(),
                length = 2,
            }
        });
    }

    #[test]
    fn test_insert_and_get_todo_in_storage() {
        let mut storage = factori::create!(Todos);
        assert_eq!(storage.length, 2);
        storage.insert_todo(Todo::new("Lorem".to_string()));
        assert_eq!(storage.length, 3);
        assert_eq!(&storage.get_one_todo(1).unwrap().message, "foo");
        assert_eq!(&storage.get_one_todo(2).unwrap().message, "boo");
        assert_eq!(&storage.get_one_todo(3).unwrap().message, "Lorem");
        assert!(!storage.get_one_todo(3).unwrap().resolved);        
    }

    #[test]
    fn test_update_one_todo_in_storage() {
        let mut storage = factori::create!(Todos);        
        assert_eq!(&storage.get_one_todo(1).unwrap().message, "foo");
        storage.update(1, Todo::new("Lorem".to_string()));
        assert_eq!(&storage.get_one_todo(1).unwrap().message, "Lorem");
        assert!(!storage.get_one_todo(1).unwrap().resolved);     
    }

    #[test]
    fn test_remove_todo_in_storage() {
        let mut storage = factori::create!(Todos);       
        assert_eq!(storage.get_collection().len(), 2);
        assert_eq!(&storage.get_one_todo(1).unwrap().message, "foo");
        storage.remove(1);        
        assert_eq!(storage.get_collection().len(), 1);
    }

    #[test]
    fn test_resolve_one_todo_is_ok() {
        let mut storage = factori::create!(Todos);        
        assert_eq!(storage.get_one_todo(1).unwrap().resolved, false);
        storage.resolve_one_todo(1);
        assert_eq!(storage.get_one_todo(1).unwrap().resolved, true);
    }

    #[tokio::test]
    async fn test_parse_file_for_todos() {
        let mut storage = Todos::new();
        tokio::fs::write("test_todo.txt", "1-false-foo\n".as_bytes())
            .await
            .unwrap();
        assert_eq!(storage.length, 0);
        storage.parse_file_for_todos("test_todo.txt").await.unwrap();
        let todo = storage.get_one_todo(1).unwrap();
        assert!(!todo.resolved);
        assert_eq!(todo.message, "foo\n");
        assert_eq!(storage.length, 1);
        tokio::fs::remove_file("test_todo.txt").await.unwrap();
    }

    #[tokio::test]
    async fn test_parse_map_write_file() {
        let mut storage = factori::create!(Todos);        
        storage.parse_map_write_file("test_todo.txt").await.unwrap();
        let test_todo = tokio::fs::read_to_string("test_todo.txt").await.unwrap();
        assert!(test_todo.contains("1-false-foo"));
        assert!(test_todo.contains("2-false-boo"));        
        tokio::fs::remove_file("test_todo.txt").await.unwrap();
    }
}
