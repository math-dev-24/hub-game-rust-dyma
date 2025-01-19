use rusqlite::{Connection, Result};
use crate::models::users::User;

pub struct Score {
    id: i64,
    user_id: usize,
    game_id: usize,
    score: usize,
}

pub fn add_score(conn: &Connection, user_id: i64, game_id: i64, score: i64) {
    conn.execute(
        "INSERT INTO scores (user_id, game_id, score) VALUES (?1, ?2, ?3)",
        &[&user_id, &game_id, &score],
    ).expect("Une erreur est survenue lors de l'enregistrement du score !");
}

pub fn print_score(conn: &Connection, user: &User) -> Result<()> {
    println!("Voici vos score {} :", user.name);

    let mut stmt = conn.prepare(
        "SELECT games.name, scores.score
        FROM scores
        INNER JOIN games ON games.id = scores.game_id
        WHERE scores.user_id = ?1"
    )?;

    let mut rows = stmt.query(rusqlite::params![user.id])?;

    while let Some(row) = rows.next()? {
        let game: String = row.get(0)?;
        let score: i32 = row.get(1)?;
        println!("Jeu : {}, Score : {}", game, score);
    }
    Ok(())
}


