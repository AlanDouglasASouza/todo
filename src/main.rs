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
        let magenta = Style::new().magenta();
        let blue = Style::new().blue().bold();
        let red = Style::new().red();
        let green = Style::new().green();

        println!(
            "\n{} 🤔\n \n1 - Para CRIAR um TODO \n2 - Para LISTAR todos os seus TODO's \n3 - Para ALTERAR um TODO existente \n4 - Para DELETAR um TODO \n5 - Para SAIR\n",
            magenta.apply_to("Escolha uma opção para usar seu TODO LIST")
        );

        match terminal.should_ask_for_todo()? {
            UserResponse::Exit => {
                println!(
                    "\n😁 {} 🤠\n",
                    magenta.apply_to("Ok!! Todo list finalizado!").bold()
                );
                break;
            }
            UserResponse::Other => {
                terminal.clean()?;
                println!(
                    "\n🙁 {}",
                    blue.apply_to("Desculpe esse comando não é válido para esse processo...")
                );
            }
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
                    list_todos.show_all_todos(true)?;
                    println!(
                        "\n{}\n",
                        blue.apply_to("Digite o número do TODO que deseja alterar:")
                    );

                    match terminal.ask_which_todo() {
                        Ok(key) => {
                            if let Some(result) = list_todos.get_one_todo(key) {
                                terminal.show_todo(result, "\n✅: ")?;
                                let new_todo = terminal.ask_for_new_todo()?;
                                list_todos.update(key, new_todo);
                                terminal.clean()?;
                                println!(
                                    "✅ {} ✅",
                                    green.apply_to("TODO atualizado com sucesso!").bold()
                                );
                                break;
                            } else {
                                terminal.clean()?;
                                println!(
                                    "❗ {} ❗\n",
                                    red.apply_to("O TODO que você digitou não existe")
                                );
                            }
                        }
                        Err(_) => {
                            terminal.clean()?;
                            println!(
                                "❗ {} ❗\n",
                                red.apply_to("O identificador do TODO precisa ser um número!")
                            )
                        }
                    }
                }
            }
            UserResponse::Delete => {
                terminal.clean()?;
                loop {
                    list_todos.show_all_todos(true)?;
                    println!(
                        "\n{}\n",
                        blue.apply_to("Digite o número do TODO que deseja deletar:")
                    );

                    match terminal.ask_which_todo() {
                        Ok(key) => {
                            if let Some(result) = list_todos.get_one_todo(key) {
                                terminal.clean()?;
                                terminal.show_todo(result, "\n❌ ")?;
                                list_todos.remove(key);
                                println!("❌ O TODO foi excluido com sucesso! ❌");
                                break;
                            } else {
                                println!(
                                    "❗ {} ❗\n",
                                    red.apply_to("O TODO que você digitou não existe")
                                );
                            }
                        }
                        Err(_) => {
                            terminal.clean()?;
                            println!(
                                "❗ {} ❗\n",
                                red.apply_to("O identificador do TODO precisa ser um número!")
                            )
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
