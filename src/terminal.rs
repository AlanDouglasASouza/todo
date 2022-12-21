use crate::command::UserCommand;
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

    pub fn get_user_command(&self) -> Result<UserCommand, TerminalError> {
        let response = self.input()?;

        match response.trim() {
            "1" => Ok(UserCommand::Insert),
            "2" => Ok(UserCommand::ShowTodos),
            "3" => Ok(UserCommand::Update),
            "4" => Ok(UserCommand::Delete),
            "5" => Ok(UserCommand::Exit),
            _ => Ok(UserCommand::Other),
        }
    }

    pub fn show_options(&self) -> Result<(), TerminalError> {
        self.write_styled(
            "\nEscolha uma opção para usar seu TODO LIST 🤔",
            Style::new().magenta(),
        )?;
        self.write_styled(
            r"
1 - Para CRIAR um TODO
2 - Para LISTAR todos os seus TODO's
3 - Para ALTERAR um TODO existente
4 - Para DELETAR um TODO
5 - Para SAIR
",
            Style::new().white(),
        )?;        
        Ok(())
    }

    pub fn finish_todo(&self) -> Result<(), TerminalError> {
        self.write_styled(
            "\n😁 Ok!! Todo list finalizado! 🤠\n",
            Style::new().magenta(),
        )?;
        Ok(())
    }

    pub fn show_invalid_option(&self) -> Result<(), TerminalError> {
        self.clean()?;
        self.write_styled(
            "\n🙁 Desculpe esse comando não é válido para esse processo...",
            Style::new().blue(),
        )?;
        Ok(())
    }

    pub fn ask_for_new_todo(&self) -> Result<Todo, TerminalError> {
        self.write_styled("\nQual TODO deseja criar? 💬", Style::new().magenta())?;
        let new_todo = self.input()?;

        Ok(Todo { message: new_todo })
    }

    pub fn show_todo(&self, todo: &Todo, msg_initial: &str) -> Result<(), TerminalError> {
        let todo_style = style(&*todo.message).yellow().italic();
        let todo_msg = msg_initial.to_owned() + &todo_style.to_string();

        self.output
            .write_line(&todo_msg.to_string())
            .map_err(TerminalError::StdoutErr)?;
        Ok(())
    }

    pub fn show_error(&self, error: TerminalError) {
        eprintln!("{}\n", style(error.message_err()).red().bold());
    }

    pub fn get_todo_for_update(&self, list_todos: &Todos) -> Result<(u32, Todo), TerminalError> {
        list_todos.show_all_todos(true)?;
        self.write_styled(
            "\nDigite o número do TODO que deseja ALTERAR:\n",
            Style::new().blue().bold(),
        )?;
        let key = self.ask_which_todo(list_todos)?;
        let new_todo = self.ask_for_new_todo()?;
        Ok((key, new_todo))
    }

    pub fn get_id_todo_for_remove(&self, list_todos: &Todos) -> Result<u32, TerminalError> {
        list_todos.show_all_todos(true)?;
        self.write_styled(
            "\nDigite o número do TODO que deseja DELETAR: ❌\n",
            Style::new().blue().bold(),
        )?;
        let key = self.ask_which_todo(list_todos)?;
        Ok(key)
    }

    pub fn write_feedback(&self, feedback: &str) -> Result<(), TerminalError> {
        self.clean()?;
        self.write_styled(feedback, Style::new().green().bold())?;
        Ok(())
    }

    pub fn clean(&self) -> Result<(), TerminalError> {
        self.output
            .clear_screen()
            .map_err(TerminalError::StdoutErr)?;
        Ok(())
    }

    fn ask_which_todo(&self, list_todos: &Todos) -> Result<u32, TerminalError> {
        let key = self.input()?.parse().map_err(TerminalError::ParseErr)?;
        let result = self.or_not_found(list_todos.get_one_todo(key))?;
        self.show_todo(result, "\n✅: ")?;

        Ok(key)
    }

    pub fn check_list_is_empty(&self, list: &Todos) -> bool {
        if list.len() < 1 {
            self.show_error(TerminalError::NotFound(
                "A sua coleção de TODOs esta vazia".to_string(),
            ));
            return false;
        }
        true
    }

    fn input(&self) -> Result<String, TerminalError> {
        let response = self.input.read_line().map_err(TerminalError::StdinErr)?;
        Ok(response)
    }

    fn write_styled(&self, message: &str, style: Style) -> Result<(), TerminalError> {
        let msg_styled = style.apply_to(message);
        self.output
            .write_line(&msg_styled.to_string())
            .map_err(TerminalError::StdoutErr)?;
        Ok(())
    }

    fn or_not_found<'a>(&self, maybe_todo: Option<&'a Todo>) -> Result<&'a Todo, TerminalError> {
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
