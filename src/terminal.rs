use crate::response::UserResponse;
use crate::todo::Todo;
use crate::todos::Todos;
use console::{style, Style, Term};
use std::io::Error;
use std::num::ParseIntError;

pub struct Terminal {
    input: Term,
    output: Term,
}

impl Terminal {
    pub fn new() -> Self {
        Terminal {
            input: Term::stdout(),
            output: Term::stdout(),
        }
    }

    pub fn should_ask_for_todo(&mut self) -> Result<UserResponse, TerminalError> {
        let response = self.input()?;
        match response.trim() {
            "1" => Ok(UserResponse::Insert),
            "2" => Ok(UserResponse::ShowTodos),
            "3" => Ok(UserResponse::Update),
            "4" => Ok(UserResponse::Delete),
            "5" => Ok(UserResponse::Exit),
            _ => Ok(UserResponse::Other),
        }
    }

    pub fn ask_for_new_todo(&mut self) -> Result<Todo, TerminalError> {
        let ask_todo = style("\nQual TODO deseja criar? 💬").magenta();

        self.output
            .write_line(&ask_todo.to_string())
            .map_err(TerminalError::StdoutErr)?;
        let new_todo = self.input()?;

        Ok(Todo { message: new_todo })
    }

    pub fn show_todo(&mut self, todo: &Todo, msg_initial: &str) -> Result<(), TerminalError> {
        let todo_style = style(&*todo.message).yellow().italic();
        let todo_msg = msg_initial.to_owned() + &todo_style.to_string();

        self.output
            .write_line(&todo_msg.to_string())
            .map_err(TerminalError::StdoutErr)?;

        Ok(())
    }

    pub fn show_error(&mut self, error: TerminalError) {
        eprintln!("{}", style(error.message_err()).red().bold());
    }

    pub fn ask_which_todo(&mut self, list_todos: &mut Todos) -> Result<(), TerminalError> {
        let blue = Style::new().blue().bold();
        let red = Style::new().red();
        let green = Style::new().green();
        
        self.clean()?;
        loop {
            list_todos.show_all_todos(true)?;
            println!(
                "\n{}\n",
                blue.apply_to("Digite o número do TODO que deseja alterar:")
            );

            let response = self.input()?.parse().map_err(TerminalError::ParseErr);

            match response {
                Ok(key) => {
                    if let Some(result) = list_todos.get_one_todo(key) {
                        self.show_todo(result, "\n✅: ")?;
                        let new_todo = self.ask_for_new_todo()?;
                        list_todos.update(key, new_todo);
                        self.clean()?;
                        println!(
                            "✅ {} ✅",
                            green.apply_to("TODO atualizado com sucesso!").bold()
                        );
                        break;
                    } else {
                        self.clean()?;
                        println!(
                            "❗ {} ❗\n",
                            red.apply_to("O TODO que você digitou não existe")
                        );
                    }
                }
                Err(_) => {
                    self.clean()?;
                    println!(
                        "❗ {} ❗\n",
                        red.apply_to("O identificador do TODO precisa ser um número!")
                    )
                }
            }
        }

        Ok(())
    }

    pub fn clean(&mut self) -> Result<(), TerminalError> {
        self.output
            .clear_screen()
            .map_err(TerminalError::StdoutErr)?;

        Ok(())
    }

    fn input(&mut self) -> Result<String, TerminalError> {
        let response = self.input.read_line().map_err(TerminalError::StdinErr)?;
        Ok(response)
    }

    //Errado
    pub fn or_not_found<'a>(
        &mut self,
        maybe_todo: Option<&'a Todo>,
    ) -> Result<&'a Todo, TerminalError> {
        match maybe_todo {
            Some(todo) => Ok(todo),
            None => Err(TerminalError::NotFound("Valor não encontrado".to_string())),
        }
    }
}

#[derive(Debug)]
pub enum TerminalError {
    StdoutErr(Error),
    StdinErr(Error),
    ParseErr(ParseIntError),
    NotFound(String),
}

impl TerminalError {
    fn message_err(self) -> String {
        match self {
            Self::StdoutErr(err) => format!("Houve um erro ao tentar exibir mensagem {}", err),
            Self::StdinErr(err) => format!("Houve um erro na entrada de dados {}", err),
            Self::ParseErr(err) => format!("Houve um erro ao analisar os seus dados {}", err),
            Self::NotFound(err) => format!("{err}"),
        }
    }

    /* pub fn or_not_found(maybe_todo: Option<&Todo>) -> Result<&Todo, TerminalError> {
        match maybe_todo {
            Some(todo) => Ok(todo),
            None => Err(Self::NotFound("Valor não encontrado".to_string()))
        }
    } */
}
