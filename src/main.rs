mod terminal;
mod todo;
use terminal::{Terminal, TerminalError, UserResponse};

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
