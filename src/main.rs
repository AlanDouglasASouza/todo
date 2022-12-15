mod terminal;
mod todo;
use crate::terminal::{Terminal, TerminalError, UserResponse};
use console::Style;

fn main() {
    let mut terminal = Terminal::new();
    let magenta = Style::new().magenta();
    println!("{} 😃", magenta.apply_to("Olá!").bold());

    if let Err(error) = run() {
        terminal.show_error(error)
    }
}

fn run() -> Result<(), TerminalError> {
    loop {
        let mut ask_todo = Terminal::new();
        let magenta = Style::new().magenta();
        let blue = Style::new().blue().bold();

        println!(
            "\n{} 🤔 (Digite: 's' para SIM ou 'n' para NÃO)",            
            magenta.apply_to("Você gostaria de adicionar um novo TODO?")
        );

        match ask_todo.should_ask_for_todo()? {
            UserResponse::No => {
                println!("\n😁 {} 🤠\n", magenta.apply_to("Ok!! Todo list finalizado!").bold());
                break;
            }
            UserResponse::Other => {
                println!("\n🙁 {}", 
                blue.apply_to("Desculpe esse comando não é válido para esse processo..."));
            }
            UserResponse::Yes => {
                let todo = ask_todo.ask_for_new_todo()?;
                ask_todo.show_todo(&todo)?;
            }
        }
    }
    Ok(())
}
