use crate::dao::db_dao::*;
use crate::model::estrutura_abrigo::*;
use crate::view::*;

impl Animais {
    pub fn new(idade: u8, nome_animal: String, raca_animal: String, tutor: Option<Tutor>) -> Self {
        Animais {
            id: None,
            idade,
            nome_animal,
            raca_animal,
            tutor,
        }
    }
}

impl Tutor {
    pub fn new(nome: String, idade: u8) -> Self {
        Tutor {
            id: None,
            nome,
            idade,
        }
    }
}

pub fn adicionar_tutor() {
    let nome = cli_abrigo::read_input("Nome do tutor: ");
    let idade = cli_abrigo::read_input("Idade do tutor: ");

    let idade: u8 = match cli_abrigo::parse_u8(&idade) {
        Some(i) => i,
        None => {
            println!("Idade inválida.");
            return;
        }
    };

    let mut tutor = Tutor::new(nome, idade);
    match AnimalDAO::new() {
        Ok(dao) => match dao.inserir_tutor(&mut tutor) {
            Ok(_) => println!("Tutor inserido com sucesso (id: {:?}).", tutor.id),
            Err(err) => println!("Erro ao inserir tutor: {}", err),
        },
        Err(err) => println!("Erro ao abrir banco de dados: {}", err),
    }
}

pub fn adicionar_animal() {
    let nome_animal = cli_abrigo::read_input("Nome do animal: ");
    let idade = cli_abrigo::read_input("Idade do animal: ");
    let raca = cli_abrigo::read_input("Raça do animal: ");
    let tutor_id_txt = cli_abrigo::read_input("ID do tutor (deixe vazio para nenhum): ");

    let idade: u8 = match cli_abrigo::parse_u8(&idade) {
        Some(i) => i,
        None => {
            println!("Idade inválida.");
            return;
        }
    };

    let tutor = if tutor_id_txt.is_empty() {
        None
    } else if let Some(id) = cli_abrigo::parse_u32(&tutor_id_txt) {
        Some(Tutor {
            id: Some(id),
            nome: String::new(),
            idade: 0,
        })
    } else {
        println!("ID do tutor inválido.");
        return;
    };

    let mut animal = Animais::new(idade, nome_animal, raca, tutor);
    match AnimalDAO::new() {
        Ok(dao) => match dao.inserir_animal(&mut animal) {
            Ok(_) => println!("Animal inserido com sucesso (id: {:?}).", animal.id),
            Err(err) => println!("Erro ao inserir animal: {}", err),
        },
        Err(err) => println!("Erro ao abrir banco de dados: {}", err),
    }
}

pub fn listar_todos_animais() {
    match AnimalDAO::new() {
        Ok(dao) => match dao.listar_animais() {
            Ok(animais) => {
                if animais.is_empty() {
                    println!("Nenhum animal cadastrado.");
                    return;
                }
                for animal in animais {
                    let tutor_info = match animal.tutor {
                        Some(tutor) => format!(
                            "Tutor {{ id: {:?}, nome: {}, idade: {} }}",
                            tutor.id, tutor.nome, tutor.idade
                        ),
                        None => "Sem tutor".to_string(),
                    };
                    println!(
                        "ID: {:?}, Nome: {}, Idade: {}, Raça: {}, {}",
                        animal.id, animal.nome_animal, animal.idade, animal.raca_animal, tutor_info
                    );
                }
            }
            Err(err) => println!("Erro ao listar animais: {}", err),
        },
        Err(err) => println!("Erro ao abrir banco de dados: {}", err),
    }
}

pub fn listar_animais_tutor() {
    let tutor_id_txt = cli_abrigo::read_input("ID do tutor: ");
    let tutor_id: u32 = match cli_abrigo::parse_u32(&tutor_id_txt) {
        Some(i) => i,
        None => {
            println!("ID do tutor inválido.");
            return;
        }
    };

    match AnimalDAO::new() {
        Ok(dao) => match dao.listar_animais_do_tutor(tutor_id) {
            Ok(animais) => {
                if animais.is_empty() {
                    println!("Nenhum animal encontrado para esse tutor.");
                    return;
                }
                for animal in animais {
                    println!(
                        "ID: {:?}, Nome: {}, Idade: {}, Raça: {}",
                        animal.id, animal.nome_animal, animal.idade, animal.raca_animal
                    );
                }
            }
            Err(err) => println!("Erro ao listar animais do tutor: {}", err),
        },
        Err(err) => println!("Erro ao abrir banco de dados: {}", err),
    }
}

pub fn atualizar_tutor_animal() {
    let animal_id_txt = cli_abrigo::read_input("ID do animal: ");
    let tutor_id_txt = cli_abrigo::read_input("ID do novo tutor (deixe vazio para nenhum): ");

    let animal_id: u32 = match cli_abrigo::parse_u32(&animal_id_txt) {
        Some(i) => i,
        None => {
            println!("ID do animal inválido.");
            return;
        }
    };

    let tutor_id = if tutor_id_txt.is_empty() {
        None
    } else {
        match cli_abrigo::parse_u32(&tutor_id_txt) {
            Some(i) => Some(i),
            None => {
                println!("ID do tutor inválido.");
                return;
            }
        }
    };

    match AnimalDAO::new() {
        Ok(dao) => match dao.atualizar_tutor_do_animal(animal_id, tutor_id) {
            Ok(_) => println!("Tutor do animal atualizado com sucesso."),
            Err(err) => println!("Erro ao atualizar tutor do animal: {}", err),
        },
        Err(err) => println!("Erro ao abrir banco de dados: {}", err),
    }
}

pub fn editar_animal() {
    let animal_id_txt = cli_abrigo::read_input("ID do animal: ");
    let nome_animal = cli_abrigo::read_input("Nome do animal: ");
    let idade_txt = cli_abrigo::read_input("Idade do animal: ");
    let raca = cli_abrigo::read_input("Raça do animal: ");
    let tutor_id_txt = cli_abrigo::read_input("ID do tutor (deixe vazio para nenhum): ");

    let animal_id: u32 = match cli_abrigo::parse_u32(&animal_id_txt) {
        Some(i) => i,
        None => {
            println!("ID do animal inválido.");
            return;
        }
    };
    let idade: u8 = match cli_abrigo::parse_u8(&idade_txt) {
        Some(i) => i,
        None => {
            println!("Idade inválida.");
            return;
        }
    };

    let tutor = if tutor_id_txt.is_empty() {
        None
    } else if let Some(id) = cli_abrigo::parse_u32(&tutor_id_txt) {
        Some(Tutor {
            id: Some(id),
            nome: String::new(),
            idade: 0,
        })
    } else {
        println!("ID do tutor inválido.");
        return;
    };

    let animal = Animais {
        id: Some(animal_id),
        idade,
        nome_animal,
        raca_animal: raca,
        tutor,
    };

    match AnimalDAO::new() {
        Ok(dao) => match dao.editar_animal(&animal) {
            Ok(_) => println!("Animal atualizado com sucesso."),
            Err(err) => println!("Erro ao atualizar animal: {}", err),
        },
        Err(err) => println!("Erro ao abrir banco de dados: {}", err),
    }
}

pub fn remover_animal() {
    let animal_id_txt = cli_abrigo::read_input("ID do animal: ");
    let animal_id: u32 = match cli_abrigo::parse_u32(&animal_id_txt) {
        Some(i) => i,
        None => {
            println!("ID do animal inválido.");
            return;
        }
    };

    match AnimalDAO::new() {
        Ok(dao) => match dao.remover_animal(animal_id) {
            Ok(_) => println!("Animal removido com sucesso."),
            Err(err) => println!("Erro ao remover animal: {}", err),
        },
        Err(err) => println!("Erro ao abrir banco de dados: {}", err),
    }
}

pub fn remover_tutor() {
    let tutor_id_txt = cli_abrigo::read_input("ID do tutor: ");
    let tutor_id: u32 = match cli_abrigo::parse_u32(&tutor_id_txt) {
        Some(i) => i,
        None => {
            println!("ID do tutor inválido.");
            return;
        }
    };

    match AnimalDAO::new() {
        Ok(dao) => match dao.remover_tutor(tutor_id) {
            Ok(_) => println!("Tutor removido com sucesso."),
            Err(err) => println!("Erro ao remover tutor: {}", err),
        },
        Err(err) => println!("Erro ao abrir banco de dados: {}", err),
    }
}
