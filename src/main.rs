fn main() {
    println!("Olá! 😃");

    loop {
        println!();
        println!("Você gostaria de adicionar um novo TODO? 🤔 (Digite: 's' para SIM ou qualquer outra tecla para NÃO)");

        if input() == "s" {
            create_todo();
        } else {
            println!();
            println!("Todo list finalizado! 🤠");
            break;
        }        
    }
}

fn create_todo() {
    println!();

    println!("Qual TODO deseja criar?");
    let todo = input();

    println!();

    println!("✅: {todo}");
}

fn input() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
 }
