use crate::terminal::{Terminal, TerminalError, UserResponse};

fn main() {
    let mut terminal = Terminal::new();
    println!("Olá! 😃");

    if let Err(error) = run() {
        terminal.show_error(error)
    }
}

fn run() -> Result<(), TerminalError> {
    loop {
        let mut ask_todo = Terminal::new();
        println!(
            "\nVocê gostaria de adicionar um novo TODO? 🤔 (Digite: 's' para SIM ou 'n' para NÃO)"
        );

        match ask_todo.should_ask_for_todo()? {
            UserResponse::No => {
                println!("\n😁 Ok!! Todo list finalizado! 🤠\n");
                break;
            }
            UserResponse::Other => {
                println!("\n🙁 Desculpe esse comando não é válido para esse processo...");
            }
            UserResponse::Yes => {
                let todo = ask_todo.ask_for_new_todo()?;
                ask_todo.show_todo(&todo)?;
            }
        }
    }
    Ok(())
}

/* mod todo {
    #[derive(Debug, Clone)]
    pub struct Todo {
        pub message: String,
    }
} */

mod todo;

mod terminal {
    //use crate::todo::Todo;
    use crate::todo::todo::Todo;    
    use std::io::{Error, Stdin, Stdout, Write};
    pub struct Terminal {
        stdin: Stdin,
        stdout: Stdout,
    }

    impl Terminal {
        pub fn new() -> Self {
            Terminal {
                stdin: std::io::stdin(),
                stdout: std::io::stdout(),
            }
        }

        pub fn should_ask_for_todo(&mut self) -> Result<UserResponse, TerminalError> {
            let mut buf = String::new();
            self.stdin
                .read_line(&mut buf)
                .map_err(TerminalError::StdinErr)?;

            match buf.trim() {
                "s" => Ok(UserResponse::Yes),
                "n" => Ok(UserResponse::No),
                _ => Ok(UserResponse::Other),
            }
        }

        pub fn ask_for_new_todo(&mut self) -> Result<Todo, TerminalError> {
            println!("\nQual TODO deseja criar?");
            let mut new_todo = String::new();
            self.stdin
                .read_line(&mut new_todo)
                .map_err(TerminalError::StdinErr)?;

            let todo_message = new_todo.trim().to_string();

            Ok(Todo {
                message: todo_message,
            })
        }

        pub fn show_todo(&mut self, todo: &Todo) -> Result<(), TerminalError> {
            writeln!(self.stdout, "\n✅: {}", todo.message).map_err(TerminalError::StdoutErr)?;
            Ok(())
        }

        pub fn show_error(&mut self, error: TerminalError) {
            eprintln!("{}", error.message_err());
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
}
