use crate::command::UserCommand;
use crate::todo::Todo;
use console::{style, Style, Term};
use std::io::Error;
use std::num::ParseIntError;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, Stdin, Stdout};

pub struct Terminal {
    input: BufReader<Stdin>,
    output: Stdout,
}

impl Terminal {
    pub fn new() -> Self {
        Self {
            input: BufReader::new(tokio::io::stdin()),
            output: tokio::io::stdout(),
        }
    }
}

#[async_trait::async_trait]
pub trait UserInterface {
    async fn get_user_command(&mut self) -> Result<UserCommand, TerminalError>;
    async fn show_options(&mut self) -> Result<(), TerminalError>;
    async fn finish_todo(&mut self) -> Result<(), TerminalError>;
    async fn show_invalid_option(&mut self) -> Result<(), TerminalError>;
    async fn ask_for_new_todo(&mut self) -> Result<Todo, TerminalError>;
    async fn show_todo(&mut self, todo: &Todo, msg_initial: &str) -> Result<(), TerminalError>;
    fn show_error(&self, error: TerminalError);
    async fn ask_key_todo_update(&mut self) -> Result<(), TerminalError>;
    async fn ask_key_todo_delete(&mut self) -> Result<(), TerminalError>;
    async fn write_feedback(&mut self, feedback: &str) -> Result<(), TerminalError>;
    fn clean(&self) -> Result<(), TerminalError>;
    async fn parse_user_option(&mut self) -> Result<u32, TerminalError>;
    async fn input(&mut self) -> Result<String, TerminalError>;
    async fn write_styled(&mut self, message: &str, style: Style) -> Result<(), TerminalError>;
    fn or_not_found<'a>(&self, maybe_todo: Option<&'a Todo>) -> Result<&'a Todo, TerminalError>;
    async fn get_key_todo_resolve(&mut self) -> Result<(), TerminalError>;
}

#[async_trait::async_trait]
impl UserInterface for Terminal {
    async fn get_user_command(&mut self) -> Result<UserCommand, TerminalError> {
        let response = self.input().await?;

        match response.trim() {
            "1" => Ok(UserCommand::Insert),
            "2" => Ok(UserCommand::ShowTodos),
            "3" => Ok(UserCommand::Resolve),
            "4" => Ok(UserCommand::Update),
            "5" => Ok(UserCommand::Delete),
            "0" => Ok(UserCommand::Exit),
            _ => Ok(UserCommand::Other),
        }
    }

    async fn show_options(&mut self) -> Result<(), TerminalError> {
        self.write_styled(
            "\nEscolha uma opção para usar seu TODO LIST 🤔\n",
            Style::new().magenta(),
        )
        .await?;
        self.write_styled(
            r"
    1 - Para CRIAR um TODO
    2 - Para LISTAR todos os seus TODO's
    3 - Para RESOLVER UM TODO
    4 - Para ALTERAR um TODO existente
    5 - Para DELETAR um TODO
    0 - Para SAIR
    ",
            Style::new().white(),
        )
        .await?;
        Ok(())
    }

    async fn finish_todo(&mut self) -> Result<(), TerminalError> {
        self.write_styled(
            "\n😁 Ok!! Todo list finalizado! 🤠\n",
            Style::new().magenta(),
        )
        .await?;
        Ok(())
    }

    async fn show_invalid_option(&mut self) -> Result<(), TerminalError> {
        self.clean()?;
        self.write_styled(
            "\n🙁 Desculpe esse comando não é válido para esse processo...\n",
            Style::new().blue(),
        )
        .await?;
        Ok(())
    }

    async fn ask_for_new_todo(&mut self) -> Result<Todo, TerminalError> {        
        let style = Style::new().magenta();
        println!("{} 💬\n", style.apply_to("\nQual TODO deseja criar?"));
        let new_todo = self.input().await?;

        Ok(Todo::new(new_todo))
    }

    async fn show_todo(&mut self, todo: &Todo, msg_initial: &str) -> Result<(), TerminalError> {
        let todo_msg = match todo.resolved {
            false => format!("{msg_initial}{}", &style(&*todo.message).yellow().italic()),
            true => format!("✅: {}", &style(&*todo.message).yellow().italic().dim()),
        };

        self.output
            .write(&todo_msg.as_bytes())
            .await
            .map_err(TerminalError::StdoutErr)?;
        Ok(())
    }

    fn show_error(&self, error: TerminalError) {
        eprintln!("{}\n", style(error.message_err()).red().bold());
    }

    async fn ask_key_todo_update(&mut self) -> Result<(), TerminalError> {
        self.write_styled(
            "\nDigite o número do TODO que deseja ALTERAR:\n",
            Style::new().blue().bold(),
        )
        .await?;
        
        Ok(())
    }

    async fn ask_key_todo_delete(&mut self) -> Result<(), TerminalError> {
        self.write_styled(
            "\nDigite o número do TODO que deseja DELETAR: ❌\n",
            Style::new().blue().bold(),
        )
        .await?;
        Ok(())
    }

    async fn write_feedback(&mut self, feedback: &str) -> Result<(), TerminalError> {
        self.clean()?;
        self.write_styled(feedback, Style::new().green().bold())
            .await?;
        Ok(())
    }

    fn clean(&self) -> Result<(), TerminalError> {
        Term::stdout()
            .clear_screen()
            .map_err(TerminalError::StdoutErr)?;

        Ok(())
    }

    async fn parse_user_option(&mut self) -> Result<u32, TerminalError> {       
        let user_input = self.input().await?;
        let key = user_input.trim().parse().map_err(TerminalError::ParseErr)?;
        
        Ok(key)
    }

    async fn input(&mut self) -> Result<String, TerminalError> {
        let mut response = String::new();
        self.input
            .read_line(&mut response)
            .await
            .map_err(TerminalError::StdinErr)?;
        Ok(response)
    }

    async fn write_styled(&mut self, message: &str, style: Style) -> Result<(), TerminalError> {
        let msg_styled = style.apply_to(message);
        self.output
            .write(&msg_styled.to_string().as_bytes())
            .await
            .map_err(TerminalError::StdoutErr)?;
        Ok(())
    }

    fn or_not_found<'a>(&self, maybe_todo: Option<&'a Todo>) -> Result<&'a Todo, TerminalError> {
        maybe_todo.ok_or(TerminalError::NotFound(
            "❗ O valor consultado não existe ❗".to_string(),
        ))
    }

    async fn get_key_todo_resolve(&mut self) -> Result<(), TerminalError> {
        self.write_styled(
            "\nDigite o número do TODO que deseja RESOLVER: ✅\n",
            Style::new().blue().bold(),
        )
        .await?;
        
        Ok(())
    }
}

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
