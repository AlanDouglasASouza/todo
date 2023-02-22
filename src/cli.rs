use crate::command::UserCommand;
use crate::terminal::{TerminalError, UserInterface};
use crate::todos::TodoStorage;
use console::Style;

pub struct TodoCli {
    pub user_interface: Box<dyn UserInterface>,
    todo_storage: Box<dyn TodoStorage>,
}

impl TodoCli {
    pub fn new<U: UserInterface + 'static, S: TodoStorage + 'static>(ui: U, storage: S) -> Self {
        Self {
            user_interface: Box::new(ui),
            todo_storage: Box::new(storage),
        }
    }

    pub async fn run(&mut self) -> Result<(), TerminalError> {
        self.todo_storage
            .parse_file_for_todos("todo_list.txt")
            .await?;
        self.user_interface
            .write_styled("Olá! 😃\n", Style::new().magenta())
            .await?;

        loop {
            self.user_interface.show_options().await?;

            match self.user_interface.get_user_command().await? {
                UserCommand::Insert => self.add_todo().await?,
                UserCommand::ShowTodos => self.show_todos().await?,
                UserCommand::Resolve => self.resolve_todo().await?,
                UserCommand::Update => self.update_todo().await?,
                UserCommand::Delete => self.delete_todo().await?,
                UserCommand::Other => self.user_interface.show_invalid_option().await?,
                UserCommand::Exit => {
                    self.user_interface.finish_todo().await?;
                    return Ok(());
                }
            }
        }
    }

    async fn add_todo(&mut self) -> Result<(), TerminalError> {
        self.user_interface.clean()?;
        let todo = self.user_interface.ask_for_new_todo().await?;
        self.user_interface.show_todo(&todo, "\n✅: ").await?;
        self.todo_storage.insert_todo(todo);
        self.todo_storage
            .parse_map_write_file("todo_list.txt")
            .await?;

        Ok(())
    }

    async fn show_todos(&mut self) -> Result<(), TerminalError> {
        self.user_interface.clean()?;
        self.user_interface
            .write_styled("\nOs seus TODO's são: 📖\n\n", Style::new().blue().bold())
            .await?;
        self.show_all_todos(false).await?;
        Ok(())
    }

    async fn update_todo(&mut self) -> Result<(), TerminalError> {
        self.user_interface.clean()?;
        while self.check_list_is_empty(&*self.todo_storage) {
            self.show_all_todos(true).await?;
            self.user_interface.ask_key_todo_update().await?;

            match self.user_interface.parse_user_option().await {
                Ok(key) => {
                    if self.todo_is_found(key, "").await? {
                        let todo = self.user_interface.ask_for_new_todo().await?;
                        self.todo_storage.update(key, todo);
                        self.todo_storage
                            .parse_map_write_file("todo_list.txt")
                            .await?;
                        self.user_interface
                            .write_feedback("\n✅ TODO atualizado com sucesso! ✅\n")
                            .await?;
                        return Ok(());
                    }
                }
                Err(error) => {
                    self.user_interface.clean()?;
                    self.user_interface.show_error(error)
                }
            }
        }
        Ok(())
    }

    async fn delete_todo(&mut self) -> Result<(), TerminalError> {
        self.user_interface.clean()?;
        while self.check_list_is_empty(&*self.todo_storage) {
            self.show_all_todos(true).await?;
            self.user_interface.ask_key_todo_delete().await?;

            match self.user_interface.parse_user_option().await {
                Ok(key) => {
                    if self
                        .todo_is_found(key, "\n❌ O TODO foi excluído com sucesso! ❌\n")
                        .await?
                    {
                        self.todo_storage.remove(key);
                        self.todo_storage
                            .parse_map_write_file("todo_list.txt")
                            .await?;
                        return Ok(());
                    }
                }
                Err(error) => {
                    self.user_interface.clean()?;
                    self.user_interface.show_error(error)
                }
            }
        }
        Ok(())
    }

    fn check_list_is_empty(&self, list: &dyn TodoStorage) -> bool {
        if list.is_empty() < 1 {
            self.user_interface.show_error(TerminalError::NotFound(
                "A sua coleção de TODOs esta vazia".to_string(),
            ));
            return false;
        }
        true
    }

    async fn show_all_todos(&mut self, show_keys: bool) -> Result<(), TerminalError> {
        for (key, todo) in self.todo_storage.get_collection() {
            if show_keys {
                self.user_interface
                    .show_todo(todo, format!("{key}: ").as_str())
                    .await?;
            } else {
                self.user_interface.show_todo(todo, "⏳: ").await?;
            }
        }

        Ok(())
    }

    async fn resolve_todo(&mut self) -> Result<(), TerminalError> {
        self.user_interface.clean()?;
        while self.check_list_is_empty(&*self.todo_storage) {
            self.show_all_todos(true).await?;
            self.user_interface.get_key_todo_resolve().await?;

            match self.user_interface.parse_user_option().await {
                Ok(key) => {
                    if self
                        .todo_is_found(key, "\n✅ TODO resolvido com sucesso! ✅\n")
                        .await?
                    {
                        self.todo_storage.resolve_one_todo(key);
                        self.todo_storage
                            .parse_map_write_file("todo_list.txt")
                            .await?;
                        return Ok(());
                    }
                }
                Err(error) => {
                    self.user_interface.clean()?;
                    self.user_interface.show_error(error)
                }
            }
        }
        Ok(())
    }

    async fn todo_is_found(&mut self, key: u32, feedback: &str) -> Result<bool, TerminalError> {
        let result = self
            .user_interface
            .or_not_found(self.todo_storage.get_one_todo(key));
        match result {
            Ok(todo) => {
                self.user_interface.show_todo(&todo, "\n✅ ").await?;
                self.user_interface.write_feedback(feedback).await?;
                Ok(true)
            }
            Err(error) => {
                self.user_interface.clean()?;
                self.user_interface.show_error(error);
                Ok(false)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::todo::*;
    use crate::todo::mocks::_Factori_Builder_Todo;
    use crate::{terminal::MockUserInterface, todos::MockTodoStorage};
    use std::collections::BTreeMap;

    fn create_mocks() -> (MockUserInterface, MockTodoStorage) {
        let mut list = BTreeMap::<u32, Todo>::new();
        list.entry(1).or_insert(factori::create!(Todo));

        let mut mock_user_interface = MockUserInterface::new();
        mock_user_interface.expect_clean().returning(|| Ok(()));
        mock_user_interface
            .expect_ask_key_todo_update()            
            .return_once(move || Ok(()));
        mock_user_interface
            .expect_parse_user_option()
            .returning(|| Ok(1));
        mock_user_interface
            .expect_write_feedback()
            .returning(|_| Ok(()));
        mock_user_interface
            .expect_or_not_found()
            .returning(|_| Ok(Todo::new("boo".to_string())));
        mock_user_interface
            .expect_get_key_todo_resolve()
            .returning(|| Ok(()));
        mock_user_interface
            .expect_ask_key_todo_delete()
            .return_once(|| Ok(()));

        let mut mock_storage = MockTodoStorage::new();
        mock_storage
            .expect_parse_map_write_file()
            .returning(|_| Ok(()));
        mock_storage.expect_insert_todo().return_once(|_| ());
        mock_storage.expect_get_collection().return_const(list);
        mock_storage
            .expect_update()
            .withf(|key, todo| key == &1 && todo.message == "boo")
            .return_once(|_, _| true);
        mock_storage.expect_is_empty().returning(|| 1);
        mock_storage
            .expect_get_one_todo()
            .returning(|_| Some(Todo::new("boo".to_string())));
        mock_storage.expect_resolve_one_todo().return_once(|_| true);
        mock_storage.expect_remove().return_once(|_| ());
        mock_storage.expect_parse_file_for_todos().withf(|path| path == "todo_list.txt").return_once(|_| Ok(()));

        (mock_user_interface, mock_storage)
    }

    #[tokio::test]
    async fn test_cli_run() {
        let (mut mock_user_interface, mock_storage) = create_mocks();
       
        mock_user_interface
            .expect_show_options()           
            .returning(|| Ok(()));
        mock_user_interface
            .expect_write_styled()           
            .returning(|_,_| Ok(()));
        
        mock_user_interface
            .expect_get_user_command()       
            .return_once(|| Ok(UserCommand::Exit));
        mock_user_interface
            .expect_finish_todo()       
            .returning(|| Ok(())); 


        let mut cli = TodoCli {
            user_interface: Box::new(mock_user_interface),
            todo_storage: Box::new(mock_storage),
        };

        cli.run().await.unwrap();        
    }

    #[tokio::test]
    async fn add_todo_and_verify_if_exist_with_show_todo() {
        let (mut mock_user_interface, mock_storage) = create_mocks();

        mock_user_interface
            .expect_ask_for_new_todo()
            .times(1)
            .return_once(|| Ok(factori::create!(Todo)));
        mock_user_interface
            .expect_show_todo()            
            .withf(|todo, _| todo.message == "foo")
            .returning(|_, _| Ok(()));

        let mut cli = TodoCli {
            user_interface: Box::new(mock_user_interface),
            todo_storage: Box::new(mock_storage),
        };

        cli.add_todo().await.unwrap();
        cli.show_all_todos(true).await.unwrap();
    }

    #[tokio::test]
    async fn test_update_todo() {
        let (mut mock_user_interface, mock_storage) = create_mocks();
        mock_user_interface
            .expect_ask_for_new_todo()
            .times(1)
            .return_once(|| Ok(Todo::new("boo".to_string())));
        mock_user_interface
            .expect_show_todo()
            .withf(|todo, _| todo.message == "foo")
            .return_once(|_, _| Ok(()));
        mock_user_interface
            .expect_show_todo()
            .withf(|todo, _| todo.message == "boo")
            .return_once(|_, _| Ok(()));

        let mut cli = TodoCli {
            user_interface: Box::new(mock_user_interface),
            todo_storage: Box::new(mock_storage),
        };
        cli.update_todo().await.unwrap();
    }

    #[tokio::test]
    async fn test_resolve_and_delete_todo() {
        let (mut mock_user_interface, mock_storage) = create_mocks();

        mock_user_interface
            .expect_show_todo()
            .withf(|todo, _| todo.message.contains("oo"))
            .returning(|_, _| Ok(()));

        let mut cli = TodoCli {
            user_interface: Box::new(mock_user_interface),
            todo_storage: Box::new(mock_storage),
        };

        cli.resolve_todo().await.unwrap();
        cli.delete_todo().await.unwrap();
    }
}
