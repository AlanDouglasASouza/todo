mod command;
mod terminal;
mod todo;
mod todos;
use crate::command::UserCommand;
use crate::terminal::{Terminal, TerminalError};
use crate::todos::Todos;
use console::Style;

fn main() {
    let terminal = Terminal::new();
    let magenta = Style::new().magenta();
    println!("{} 😃", magenta.apply_to("Olá!").bold());

    if let Err(error) = run() {
        terminal.show_error(error)
    }
}

fn run() -> Result<(), TerminalError> {
    let mut list_todos = Todos::new();
    loop {
        let terminal = Terminal::new();
        let blue = Style::new().blue().bold();

        terminal.show_options()?;

        match terminal.get_user_command()? {
            UserCommand::Exit => {
                terminal.finish_todo()?;
                break;
            }
            UserCommand::Other => terminal.show_invalid_option()?,
            UserCommand::Insert => {
                terminal.clean()?;
                let todo = terminal.ask_for_new_todo()?;
                terminal.show_todo(&todo, "\n✅: ")?;
                list_todos.insert_todo(todo);
            }
            UserCommand::ShowTodos => {
                terminal.clean()?;
                println!("\n{}\n", blue.apply_to("📖 Os seus TODO's são:"));
                list_todos.show_all_todos(false)?;
            }
            UserCommand::Update => {
                terminal.clean()?;
                while terminal.check_list_is_empty(&mut list_todos) {
                    match terminal.get_todo_for_update(&list_todos) {
                        Ok((key, todo)) => {
                            list_todos.update(key, todo);
                            terminal.write_feedback("✅ TODO atualizado com sucesso! ✅")?;
                            break;
                        }
                        Err(error) => {
                            terminal.clean()?;
                            terminal.show_error(error)
                        }
                    }
                }
            }
            UserCommand::Delete => {
                terminal.clean()?;
                while terminal.check_list_is_empty(&mut list_todos) {
                    match terminal.get_id_todo_for_remove(&list_todos) {
                        Ok(key) => {
                            list_todos.remove(key);
                            terminal.write_feedback("❌ O TODO foi excluído com sucesso! ❌")?;
                            break;
                        }
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
