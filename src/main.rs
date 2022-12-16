mod response;
mod terminal;
mod todo;
mod todos;
use crate::response::UserResponse;
use crate::terminal::{Terminal, TerminalError};
use crate::todos::Todos;
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
    let mut list_todos = Todos::new();
    loop {
        let mut terminal = Terminal::new();
        let blue = Style::new().blue().bold();

        terminal.show_options()?;

        match terminal.should_ask_for_todo()? {
            UserResponse::Exit => {
                terminal.finish_todo()?;
                break;
            }
            UserResponse::Other => terminal.show_invalid_option()?,
            UserResponse::Insert => {
                terminal.clean()?;
                let todo = terminal.ask_for_new_todo()?;
                terminal.show_todo(&todo, "\n✅: ")?;
                list_todos.insert_todo(todo);
            }
            UserResponse::ShowTodos => {
                terminal.clean()?;
                println!("\n{}\n", blue.apply_to("📖 Os seus TODO's são:"));
                list_todos.show_all_todos(false)?;
            }
            UserResponse::Update => {
                terminal.clean()?;
                loop {
                    match terminal.update_todo(&mut list_todos) {
                        Ok(()) => break,
                        Err(error) => {
                            terminal.clean()?;
                            terminal.show_error(error)
                        }
                    }
                }
            }
            UserResponse::Delete => {
                terminal.clean()?;
                loop {
                    match terminal.delete_todo(&mut list_todos) {
                        Ok(()) => break,
                        Err(error) => {
                            terminal.clean()?;
                            terminal.show_error(error)
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
