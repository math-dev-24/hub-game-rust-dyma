mod database;
mod models;

use std::io::Error;
use rusqlite::Connection;
use crate::database::sqlite::initiate_db;
use crate::models::games::LIST_GAMES;
use crate::models::scores::add_score;

fn main() -> Result<(), Error> {
    let conn: Connection = Connection::open("scores.sqlite")
        .expect("Erreur lors de l'ouverture");

    initiate_db(&conn);


    println!("Hub de jeux ! by Mathieu");
    println!("Veuillez entrez votre alias :");
    let mut username: String = String::new();
    std::io::stdin().read_line(&mut username).expect("Failed to read line");
    let user = models::users::get_user(&conn, username.trim());

    loop {
        println!("Menu principal, {} choisi un jeux :", username);

        for game in LIST_GAMES.iter().enumerate() {
            println!("{} - {}", game.0 + 1, game.1);
        }

        println!("4. Affichage de mes scores");
        println!("5. Quitter");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).expect("Failed to read line");

        let selected = match choice.trim().parse::<i64>() {
            Ok(num) => num,
            Err(_) => {
                println!("Veuillez entrer un nombre valide !");
                continue;
            },
        };

        match selected {
            1..=3 => {
                let game_name = LIST_GAMES[selected as usize - 1];
                match models::games::play_game(&conn, game_name) {
                    Ok(tmp_score) => {
                        add_score(&conn, user.id, selected, tmp_score);
                    }
                    Err(e) => {
                        println!("{}", e);
                    }
                }
            }
            4 => {
                models::scores::print_score(&conn, &user).unwrap();
            }
            5 => {

                println!("Au revoir");
                break Ok(());
            }
            _ => {
                println!("Commande inconnu");
            }
        }
    }
}