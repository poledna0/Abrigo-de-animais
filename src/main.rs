mod model;
mod controller;
mod dao;
mod view;

use view::cli_abrigo::run_cli;

fn main() {

    let is_file = std::path::Path::new("abrigo.db").is_file();

    match is_file{
        
        true => { println!("Banco já criado"); },

        false => { let _ = crate::dao::db_dao::AnimalDAO::new().unwrap(); }
    }



    run_cli();
}
