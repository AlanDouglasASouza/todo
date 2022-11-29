use crate::todo::Todo;
use console::{style, Term};
use std::io::Error;

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
        let response = self.input.read_line().map_err(TerminalError::StdinErr)?;
        match response.trim() {
            "s" => Ok(UserResponse::Yes),
            "n" => Ok(UserResponse::No),
            _ => Ok(UserResponse::Other),
        }
    }

    pub fn ask_for_new_todo(&mut self) -> Result<Todo, TerminalError> {
        self.output.clear_screen().map_err(TerminalError::StdoutErr)?;
        let ask_todo = style("\nQual TODO deseja criar?").magenta();

        self.output
            .write_line(&ask_todo.to_string())
            .map_err(TerminalError::StdoutErr)?;
        let new_todo = self.input.read_line().map_err(TerminalError::StdinErr)?;

        Ok(Todo { message: new_todo })
    }

    pub fn show_todo(&mut self, todo: &Todo) -> Result<(), TerminalError> {
       
        let todo_style = style(&*todo.message).yellow().italic();
        let todo_msg =  "\n✅: ".to_owned() + &todo_style.to_string();

        self.output
            .write_line(&todo_msg.to_string())
            .map_err(TerminalError::StdoutErr)?;

        Ok(())
    }

    pub fn show_error(&mut self, error: TerminalError) {
        eprintln!("{}", style(error.message_err()).red().bold());
    }
}

#[derive(Debug)]
pub enum TerminalError {
    StdoutErr(Error),
    StdinErr(Error),
}

impl TerminalError {
    fn message_err(self) -> String {
        match self {
            Self::StdoutErr(err) => format!("Houve um erro ao tentar exibir mensagem {}", err),
            Self::StdinErr(err) => format!("Houve um erro na entrada de dados {}", err),
        }
    }
}

pub enum UserResponse {
    Yes,
    No,
    Other,
}
