use std::io::{Stdin, Stdout, Write, Error};
use std::io;

fn main() {
    println!("Olá! 😃");

    loop {
       let mut ask_todo = Terminal::new();
       let todo = ask_todo.ask_for_new_todo();

       match todo {
           Ok(new_todo) => {            
                if let Err(error) = ask_todo.show_todo(&new_todo) {
                    ask_todo.show_error(TerminalError::WriteErr(error))
                }        
           },
           Err(error) => ask_todo.show_error(error)
       }       
    }
}

#[derive(Debug, Clone)]
struct Todo {
    message: String
}

struct Terminal {
    stdin: Stdin,
    stdout: Stdout,
}

impl Terminal {
    fn new() -> Self {

        Terminal {
            stdin: std::io::stdin(), 
            stdout: std::io::stdout() 
        }
    }

    fn should_ask_for_todo(&mut self) -> Result<bool, TerminalError> {
        let mut buf = String::new();

        self.stdin.read_line(&mut buf).map_err(TerminalError::StdoutErr)?;

        Ok(buf.trim() == "s")
    }

    fn ask_for_new_todo(&mut self) -> Result<Todo, TerminalError> {

        println!("\nVocê gostaria de adicionar um novo TODO? 🤔 (Digite: 's' para SIM ou qualquer outra tecla para NÃO)");
                
        if let Ok(false) = self.should_ask_for_todo() {
            println!("\nTodo list finalizado! 🤠");
            std::process::exit(0);            
        } 

        println!("\nQual TODO deseja criar?");

        let mut new_todo = String::new();

        self.stdin.read_line(&mut new_todo).map_err(TerminalError::StdoutErr)?;

        let todo_message = new_todo.trim().to_string();

        Ok(Todo { message: todo_message })
       
    }

    fn show_todo(&mut self, todo: &Todo) -> Result<(), io::Error> { 
        writeln!(self.stdout, "\n✅: {}", todo.message)?;
        Ok(())
    }

    fn show_error(&mut self, error: TerminalError) { 
        
        eprintln!("{}", error.message_err());
        
    }
}

#[derive(Debug)]
enum TerminalError {    
    WriteErr(Error),
    StdoutErr(Error),
}

impl TerminalError {

    fn message_err(self) -> String {
        match self {            
            Self::WriteErr(err) => format!("Houve um erro ao tentar exibir mensagem {}", err),
            Self::StdoutErr(err) => format!("Houve um erro na entrada de dados {}", err)
        }
    }
}
