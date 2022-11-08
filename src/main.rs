fn main() {
    println!("Olá! 😃");

    loop {        
        println!("\nVocê gostaria de adicionar um novo TODO? 🤔 (Digite: 's' para SIM ou qualquer outra tecla para NÃO)");

        if input() == "s" {
            create_todo();
        } else {           
            println!("\nTodo list finalizado! 🤠");
            break;
        }
    }
}

fn create_todo() {
    
    println!("\nQual TODO deseja criar?");
    let todo = input();

    println!("\n✅: {todo}");
}

fn input() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
 }
