use rusqlite::Connection;

pub const LIST_GAMES: [&str; 3] = ["Jeux motus","juste nombre", "Tester vos réflexe"];


pub struct Game {
    pub id: i64,
    pub name: String,
}

pub fn add_game(conn: &Connection, name_game: &str) {
    if get_game_by_name(conn, name_game).is_none() {
        conn.execute(
            "INSERT INTO games (name) VALUES (?1)",
            &[&name_game.to_string()],
        ).expect("Une erreur est survenu sur la création du jeux");
    }
}

pub fn get_game_by_name(conn: &Connection, name: &str) -> Option<Game> {
    conn.query_row(
        "SELECT id, name FROM games WHERE name = $1",
        &[&name],
        |row| {
            Ok(Game {
                id: row.get(0)?,
                name: row.get(1)?
            })
        }
    ).ok()
}


pub fn play_game(conn: &Connection, game_name: &str) -> Result<i64, &'static str> {
    if let Some(game) = get_game_by_name(conn, game_name) {
        println!("Lancement du jeux : {}", game.name);

        let mut score: i64 = 0;
        match game.name.as_str() {
            "Jeux motus" => {
                score += game1::play_game();
            }
            "Jeux nombre" => {
                score = 10
            }
            "Tester vos réflexe" => {
                score += game2::play_game();
            }
            _ => {
                println!("Lancement du jeux : {}", game.name);
            }
        }
        Ok(score)
    }else {
        Err("Jeux non trouvé !")
    }
}