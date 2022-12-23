mod cli;
mod command;
mod terminal;
mod todo;
mod todos;
use crate::cli::TodoCli;
use console::Style;

fn main() {
    let mut cli = TodoCli::new();
    let magenta = Style::new().magenta();
    println!("{} 😃", magenta.apply_to("Olá!").bold());

    if let Err(error) = cli.run() {
        cli.user_interface.show_error(error)
    }
}
