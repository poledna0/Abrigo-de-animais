use crate::model::estrutura_abrigo::*;


impl Animais{
    pub fn new(idade: u8, nome_animal: String, raca_animal: String, tutor:Option<Tutor>) -> Self{
        Animais {id: None, idade: idade, nome_animal, raca_animal, tutor:tutor }
    }
}

impl Tutor{
    pub fn new(nome: String, idade: u8) -> Self {
        Tutor {id: None, nome, idade}
    }
}

