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

    pub fn run(&mut self) -> Result<(), TerminalError> {
        self.user_interface
            .write_styled("Olá! 😃", Style::new().magenta())?;

        loop {
            self.user_interface.show_options()?;

            match self.user_interface.get_user_command()? {
                UserCommand::Insert => self.add_todo()?,
                UserCommand::ShowTodos => self.show_todos()?,
                UserCommand::Update => self.update_todo()?,
                UserCommand::Delete => self.delete_todo()?,
                UserCommand::Other => self.user_interface.show_invalid_option()?,
                UserCommand::Exit => {
                    self.user_interface.finish_todo()?;
                    break;
                }
            }
        }
        Ok(())
    }

    fn add_todo(&mut self) -> Result<(), TerminalError> {
        self.user_interface.clean()?;
        let todo = self.user_interface.ask_for_new_todo()?;
        self.user_interface.show_todo(&todo, "\n✅: ")?;
        self.todo_storage.insert_todo(todo);
        Ok(())
    }

    fn show_todos(&self) -> Result<(), TerminalError> {
        self.user_interface.clean()?;
        self.user_interface
            .write_styled("\n📖 Os seus TODO's são:\n", Style::new().blue().bold())?;
        self.show_all_todos(false)?;
        Ok(())
    }

    fn update_todo(&mut self) -> Result<(), TerminalError> {
        self.user_interface.clean()?;
        while self.check_list_is_empty(&*self.todo_storage) {
            self.show_all_todos(true)?;
            match self.user_interface.get_todo_for_update(&*self.todo_storage) {
                Ok((key, todo)) => {
                    self.todo_storage.update(key, todo);
                    self.user_interface
                        .write_feedback("✅ TODO atualizado com sucesso! ✅")?;
                    break;
                }
                Err(error) => {
                    self.user_interface.clean()?;
                    self.user_interface.show_error(error)
                }
            }
        }
        Ok(())
    }

    fn delete_todo(&mut self) -> Result<(), TerminalError> {
        self.user_interface.clean()?;
        while self.check_list_is_empty(&*self.todo_storage) {
            self.show_all_todos(true)?;
            match self
                .user_interface
                .get_id_todo_for_remove(&*self.todo_storage)
            {
                Ok(key) => {
                    self.todo_storage.remove(key);
                    self.user_interface
                        .write_feedback("❌ O TODO foi excluído com sucesso! ❌")?;
                    break;
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
        if list.len() < 1 {
            self.user_interface.show_error(TerminalError::NotFound(
                "A sua coleção de TODOs esta vazia".to_string(),
            ));
            return false;
        }
        true
    }

    fn show_all_todos(&self, show_keys: bool) -> Result<(), TerminalError> {
        for (key, todo) in self.todo_storage.get_collection() {
            if show_keys {
                self.user_interface
                    .show_todo(todo, format!("{key}: ").as_str())?;
            } else {
                self.user_interface.show_todo(todo, "✅: ")?;
            }
        }

        Ok(())
    }
}
