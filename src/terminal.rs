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

    pub fn show_options(&mut self) -> Result<(), TerminalError> {
        self.write_styled(
            "\nEscolha uma opção para usar seu TODO LIST 🤔",
            Style::new().magenta(),
        )?;
        self.write_styled("\n1 - Para CRIAR um TODO \n2 - Para LISTAR todos os seus TODO's \n3 - Para ALTERAR um TODO existente \n4 - Para DELETAR um TODO \n5 - Para SAIR\n", Style::new().white())?;
        Ok(())
    }

    pub fn finish_todo(&mut self) -> Result<(), TerminalError> {
        self.write_styled(
            "\n😁 Ok!! Todo list finalizado! 🤠\n",
            Style::new().magenta(),
        )?;
        Ok(())
    }

    pub fn show_invalid_option(&mut self) -> Result<(), TerminalError> {
        self.clean()?;
        self.write_styled(
            "\n🙁 Desculpe esse comando não é válido para esse processo...",
            Style::new().blue(),
        )?;
        Ok(())
    }

    pub fn ask_for_new_todo(&mut self) -> Result<Todo, TerminalError> {
        self.write_styled("\nQual TODO deseja criar? 💬", Style::new().magenta())?;
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
        eprintln!("{}\n", style(error.message_err()).red().bold());
    }

    pub fn update_todo(&mut self, list_todos: &mut Todos) -> Result<(u32, Todo), TerminalError> {        
        list_todos.show_all_todos(true)?;
        self.write_styled(
            "\nDigite o número do TODO que deseja ALTERAR:\n",
            Style::new().blue().bold(),
        )?;
        let key = self.ask_which_todo(list_todos)?;
        let new_todo = self.ask_for_new_todo()?;
        Ok((key, new_todo))        
    }

    pub fn delete_todo(&mut self, list_todos: &mut Todos) -> Result<u32, TerminalError> {        
        list_todos.show_all_todos(true)?;
        self.write_styled(
            "\nDigite o número do TODO que deseja DELETAR: ❌\n",
            Style::new().blue().bold(),
        )?;
        let key = self.ask_which_todo(list_todos)?;
        Ok(key)        
    }

    pub fn write_feedback(&mut self, feedback: &str) -> Result<(), TerminalError> {
        self.clean()?;
        self.write_styled(feedback, Style::new().green().bold())?;
        Ok(())
    }

    pub fn clean(&mut self) -> Result<(), TerminalError> {
        self.output
            .clear_screen()
            .map_err(TerminalError::StdoutErr)?;
        Ok(())
    }

    fn ask_which_todo(&mut self, list_todos: &mut Todos) -> Result<u32, TerminalError> {
        let key = self.input()?.parse().map_err(TerminalError::ParseErr)?;
        let result = self.or_not_found(list_todos.get_one_todo(key))?;
        self.show_todo(result, "\n✅: ")?;

        Ok(key)
    }

    pub fn check_list_is_empty(&mut self, list: &mut Todos) -> bool {
        if list.len() < 1 {
            self.show_error(TerminalError::NotFound(
                "A sua coleção de TODOs esta vazia".to_string(),
            ));
            return false;
        }
        true
    }

    fn input(&mut self) -> Result<String, TerminalError> {
        let response = self.input.read_line().map_err(TerminalError::StdinErr)?;
        Ok(response)
    }

    fn write_styled(&mut self, message: &str, style: Style) -> Result<(), TerminalError> {
        let msg_styled = style.apply_to(message);
        self.output
            .write_line(&msg_styled.to_string())
            .map_err(TerminalError::StdoutErr)?;
        Ok(())
    }

    fn or_not_found<'a>(
        &mut self,
        maybe_todo: Option<&'a Todo>,
    ) -> Result<&'a Todo, TerminalError> {
        maybe_todo.ok_or(TerminalError::NotFound(
            "❗ O valor consultado não existe ❗".to_string(),
        ))
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
            Self::ParseErr(_err) => format!("O valor inserido precisa ser um número"),
            Self::NotFound(err) => format!("{err}"),
        }
    }
}
