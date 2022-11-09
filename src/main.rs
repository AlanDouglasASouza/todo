use std::io::{Stdin, Stdout, Write};

fn main() {
    println!("Olá! 😃");

    loop {
       let mut ask_todo = Terminal::new();
       let todo = ask_todo.ask_for_new_todo();

       ask_todo.show_todo(&todo);
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

    fn ask_for_new_todo(&mut self) -> Todo {        

        println!("\nVocê gostaria de adicionar um novo TODO? 🤔 (Digite: 's' para SIM ou qualquer outra tecla para NÃO)");
    
        let mut buf = String::new();
        self.stdin.read_line(&mut buf).unwrap();               
        
        if buf.trim() == "s" {
            println!("\nQual TODO deseja criar?");

            let mut new_todo = String::new();
            self.stdin.read_line(&mut new_todo).unwrap();
            let todo_message = new_todo.trim().to_string();

            Todo { message: todo_message }
        } else {
            println!("\nTodo list finalizado! 🤠");
            std::process::exit(0);  
        }
    }

    fn show_todo(&mut self, todo: &Todo) {        
        writeln!(self.stdout, "\n✅: {}", todo.message).unwrap();
    }
}