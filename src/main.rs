fn input() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
 }

fn main() {
     
    let mut response: String = "s".to_string();

    println!("Olá! 😃");

    while response == "s" {
        println!("");
        println!("Você gostaria de adicionar um novo TODO? 🤔 (Digite: 's' para SIM ou qualquer outra tecla para NÃO)");

        response = input();

        if response == "s" {
            create_todo();
        } else {
            println!("");
            println!("Todo list finalizado! 🤠");
        }  
    }
}

fn create_todo() {
    println!("");

    println!("Qual TODO deseja criar?");
    let todo = input();

    println!("");

    println!("✅: {}", todo);
}
