pub struct Animais {
    pub id: Option<u32>,
    pub idade: u8,
    pub nome_animal: String,
    pub raca_animal: String,
    pub tutor: Option<Tutor>,
}

pub struct Tutor {
    pub id: Option<u32>,
    pub nome: String,
    pub idade: u8,
}