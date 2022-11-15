use std::io::{Stdin, Stdout, Write};
use std::io;

fn main() {
    println!("Olá! 😃");

    loop {
       let mut ask_todo = Terminal::new();
       let todo = ask_todo.ask_for_new_todo();       
       
       if let Err(_) = ask_todo.show_todo(&todo) {
        Terminal::show_error(&mut ask_todo, Err(TerminalError::WriteErr));
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

    fn should_ask_for_todo(&mut self) -> bool {
        let mut buf = String::new();

        if let Err(_) = self.stdin.read_line(&mut buf) {
            self.show_error(Err(TerminalError::StdoutErr));
        }

        buf.trim() == "s"
    }

    fn ask_for_new_todo(&mut self) -> Todo {          

        println!("\nVocê gostaria de adicionar um novo TODO? 🤔 (Digite: 's' para SIM ou qualquer outra tecla para NÃO)");
                
        if !self.should_ask_for_todo() {
            println!("\nTodo list finalizado! 🤠");
            std::process::exit(0);            
        } 

        println!("\nQual TODO deseja criar?");

        let mut new_todo = String::new();

        if let Err(_) = self.stdin.read_line(&mut new_todo) {
            self.show_error(Err(TerminalError::StdoutErr));
        }

        let todo_message = new_todo.trim().to_string();

        Todo { message: todo_message }
    }

    fn show_todo(&mut self, todo: &Todo) -> Result<(), io::Error> { 
        writeln!(self.stdout, "\n✅: {}", todo.message)?;
        Ok(())
    }

    fn show_error(&mut self, data: Result<(), TerminalError>) {

        if let Err(error) = data {
            println!("{}", TerminalError::message_err(error));
        }
       
    }
}

enum TerminalError {    
    WriteErr,
    StdoutErr,
}

impl TerminalError {

    fn message_err(msg: TerminalError) -> String {
        match msg {            
            Self::WriteErr => "Houve um erro ao tentar exibir mensagem".to_string(),
            Self::StdoutErr => "Houve um erro na entrada de dados".to_string(),      
        }
    }
}
