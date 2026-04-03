use crate::controller::*;
use crate::dao::db_dao::AnimalDAO;
use std::io::{self, Write};

pub fn run_cli() {
    let dao = match AnimalDAO::new() {
        Ok(d) => d,
        Err(e) => {
            println!("Erro ao abrir banco de dados: {}", e);
            return;
        }
    };

    loop {
        println!("\n=== Sistema de Abrigo de Animais ===");
        println!("1. Adicionar Tutor");
        println!("2. Adicionar Animal");
        println!("3. Listar Todos os Animais");
        println!("4. Listar Todos os Tutores");
        println!("5. Listar Animais de um Tutor");
        println!("6. Atualizar Tutor de um Animal");
        println!("7. Editar Animal");
        println!("8. Remover Animal");
        println!("9. Remover Tutor");
        println!("0. Sair");
        print!("Escolha uma opção: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => abrigo_controller::adicionar_tutor(&dao),
            "2" => abrigo_controller::adicionar_animal(&dao),
            "3" => abrigo_controller::listar_todos_animais(&dao),
            "4" => abrigo_controller::listar_todos_tutores(&dao),
            "5" => abrigo_controller::listar_animais_tutor(&dao),
            "6" => abrigo_controller::atualizar_tutor_animal(&dao),
            "7" => abrigo_controller::editar_animal(&dao),
            "8" => abrigo_controller::remover_animal(&dao),
            "9" => abrigo_controller::remover_tutor(&dao),
            "0" => break,
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
