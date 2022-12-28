mod cli;
mod command;
mod terminal;
mod todo;
mod todos;
use crate::cli::TodoCli;

fn main() {
    let mut cli = TodoCli::new();

    if let Err(error) = cli.run() {
        cli.user_interface.show_error(error)
    }
}
