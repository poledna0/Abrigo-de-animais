use std::io::{self, Write};

pub fn run_cli() {

    loop {
        println!("\n=== Sistema de Abrigo de Animais ===");
        println!("1. Adicionar Tutor");
        println!("2. Adicionar Animal");
        println!("3. Listar Todos os Animais");
        println!("4. Listar Animais de um Tutor");
        println!("5. Atualizar Tutor de um Animal");
        println!("6. Editar Animal");
        println!("7. Remover Animal");
        println!("8. Sair");
        print!("Escolha uma opção: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => adicionar_tutor(&controller),
            "2" => adicionar_animal(&controller),
            "3" => listar_todos_animais(&controller),
            "4" => listar_animais_tutor(&controller),
            "5" => atualizar_tutor_animal(&controller),
            "6" => editar_animal(&controller),
            "7" => remover_animal(&controller),
            "8" => break,
            _ => println!("Opção inválida!"),
        }
    }
}
