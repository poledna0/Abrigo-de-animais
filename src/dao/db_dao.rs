use rusqlite::{Connection, Result};
use crate::model::estrutura_abrigo::{Animais, Tutor};

pub struct AnimalDAO {
    pub conn: Connection,
}

impl AnimalDAO {
    pub fn new() -> Result<Self> {
        let conn: Connection = Connection::open("abrigo.db")?;

        // tabela tutor
        conn.execute(
            "CREATE TABLE IF NOT EXISTS tutor (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                nome TEXT NOT NULL,
                idade INTEGER NOT NULL
            )",
            [],
        )?;

        // tabela animal
        conn.execute(
            "CREATE TABLE IF NOT EXISTS animal (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                idade INTEGER NOT NULL,
                nome_animal TEXT NOT NULL,
                raca_animal TEXT NOT NULL,
                tutor_id INTEGER,
                FOREIGN KEY(tutor_id) REFERENCES tutor(id)
            )",
            [],
        )?;

        Ok(AnimalDAO { conn })
    }

    // inserir tutor e pegar ID automático
    pub fn inserir_tutor(&self, tutor: &mut Tutor) -> Result<()> {
        self.conn.execute(
            "INSERT INTO tutor (nome, idade) VALUES (?1, ?2)",
            (&tutor.nome, &tutor.idade),
        )?;
        tutor.id = Some(self.conn.last_insert_rowid() as u32);
        Ok(())
    }

    // inserir animal
    pub fn inserir_animal(&self, animal: &mut Animais) -> Result<()> {
        let tutor_id = animal.tutor.as_ref().and_then(|t| t.id);
        self.conn.execute(
            "INSERT INTO animal (idade, nome_animal, raca_animal, tutor_id)
             VALUES (?1, ?2, ?3, ?4)",
            (&animal.idade, &animal.nome_animal, &animal.raca_animal, &tutor_id),
        )?;
        animal.id = Some(self.conn.last_insert_rowid() as u32);
        Ok(())
    }

    // listar todos os animais
    pub fn listar_animais(&self) -> Result<Vec<Animais>> {
        let mut stmt = self.conn.prepare(
            "SELECT 
                a.id, a.idade, a.nome_animal, a.raca_animal,
                t.id, t.nome, t.idade
             FROM animal a
             LEFT JOIN tutor t ON a.tutor_id = t.id"
        )?;

        let animais_iter = stmt.query_map([], |row| {
            let tutor = match row.get::<usize, Option<u32>>(4)? {
                Some(tutor_id) => Some(Tutor {
                    id: Some(tutor_id),
                    nome: row.get(5)?,
                    idade: row.get(6)?,
                }),
                None => None,
            };

            Ok(Animais {
                id: row.get(0)?,
                idade: row.get(1)?,
                nome_animal: row.get(2)?,
                raca_animal: row.get(3)?,
                tutor,
            })
        })?;

        let mut lista = Vec::new();
        for a in animais_iter {
            lista.push(a?);
        }
        Ok(lista)
    }

    // listar apenas animais de um tutor
    pub fn listar_animais_do_tutor(&self, tutor_id: u32) -> Result<Vec<Animais>> {
        let mut stmt = self.conn.prepare(
            "SELECT 
                a.id, a.idade, a.nome_animal, a.raca_animal,
                t.id, t.nome, t.idade
             FROM animal a
             JOIN tutor t ON a.tutor_id = t.id
             WHERE t.id = ?1"
        )?;

        let animais_iter = stmt.query_map([tutor_id], |row| {
            Ok(Animais {
                id: row.get(0)?,
                idade: row.get(1)?,
                nome_animal: row.get(2)?,
                raca_animal: row.get(3)?,
                tutor: Some(Tutor {
                    id: row.get(4)?,
                    nome: row.get(5)?,
                    idade: row.get(6)?,
                }),
            })
        })?;

        let mut lista = Vec::new();
        for a in animais_iter {
            lista.push(a?);
        }
        Ok(lista)
    }

    // atualizar tutor de um animal
    pub fn atualizar_tutor_do_animal(&self, animal_id: u32, tutor_id: Option<u32>) -> Result<()> {
        self.conn.execute(
            "UPDATE animal SET tutor_id = ?1 WHERE id = ?2",
            (&tutor_id, &animal_id),
        )?;
        Ok(())
    }

    // editar dados de um animal
    pub fn editar_animal(&self, animal: &Animais) -> Result<()> {
        let tutor_id = animal.tutor.as_ref().and_then(|t| t.id);
        self.conn.execute(
            "UPDATE animal SET idade = ?1, nome_animal = ?2, raca_animal = ?3, tutor_id = ?4
             WHERE id = ?5",
            (&animal.idade, &animal.nome_animal, &animal.raca_animal, &tutor_id, &animal.id),
        )?;
        Ok(())
    }

    // remover animal
    pub fn remover_animal(&self, animal_id: u32) -> Result<()> {
        self.conn.execute(
            "DELETE FROM animal WHERE id = ?1",
            [animal_id],
        )?;
        Ok(())
    }
}