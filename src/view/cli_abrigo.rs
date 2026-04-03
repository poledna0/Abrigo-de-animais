use std::io::{self, Write};
use crate::controller::*;


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
        println!("8. Remover Tutor");
        println!("9. Sair");
        print!("Escolha uma opção: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => abrigo_controller::adicionar_tutor(),
            "2" => abrigo_controller::adicionar_animal(),
            "3" => abrigo_controller::listar_todos_animais(),
            "4" => abrigo_controller::listar_animais_tutor(),
            "5" => abrigo_controller::atualizar_tutor_animal(),
            "6" => abrigo_controller::editar_animal(),
            "7" => abrigo_controller::remover_animal(),
            "8" => abrigo_controller::remover_tutor(),
            "9" => break,
            _ => println!("Opção inválida!"),
        }
    }
}

pub fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_owned()
}

pub fn parse_u8(value: &str) -> Option<u8> {
    value.trim().parse::<u8>().ok()
}

pub fn parse_u32(value: &str) -> Option<u32> {
    value.trim().parse::<u32>().ok()
}

