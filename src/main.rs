mod cli;
mod command;
mod terminal;
mod todo;
mod todos;
use crate::cli::TodoCli;
use crate::terminal::Terminal;
use crate::todos::Todos;

fn main() {
    let mut cli = TodoCli::new(Terminal::new(), Todos::new());

    if let Err(error) = cli.run() {
        cli.user_interface.show_error(error)
    }
}
