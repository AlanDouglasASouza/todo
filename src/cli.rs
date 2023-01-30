use crate::command::UserCommand;
use crate::terminal::{TerminalError, UserInterface};
use crate::todos::TodoStorage;
use console::Style;

pub(crate) struct TodoCli {
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
        self.todo_storage.parse_file_for_todos().await?;
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
        self.todo_storage.insert_todo(todo).await?;
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
                        self.todo_storage.update(key, todo).await?;
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
                        self.todo_storage.remove(key).await?;
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
                        self.todo_storage.resolve_one_todo(key).await?;
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
                self.user_interface.show_todo(todo, "\n✅ ").await?;
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
