use rusqlite::Connection;

#[derive(Debug)]
pub struct User {
    pub id: i64,
    pub name: String,
}

pub fn get_user(conn: &Connection, name: &str) -> User {
    // récupérer l'id si existant
    let option: Option<i64> = conn
        .query_row(
            "SELECT id FROM users WHERE name = ?1",
            &[&name],
            |row| row.get(0),
        )
        .ok();

    match option {
        Some(id) => {
            return User { id, name: name.to_string()}
        }
        None => {
            return User {
                id: create_user(conn, name),
                name: name.to_string(),
            }
        }
    }
}

fn create_user(conn: &Connection, name: &str) -> i64 {
    conn.execute(
        "INSERT INTO users (name) VALUES (?1)",
        &[&name]
    ).expect("Erreur lors de la création d'un nouveau joueur");
    conn.last_insert_rowid()
}