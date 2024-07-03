use std::sync::Mutex;

use rusqlite::{params, Connection, Result};

#[derive(Clone, Default, PartialEq, Debug)]
pub struct UserDetails {
    pub user_id: String,
    pub integration_token: String,
    pub database_id: String,
}

pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = get_db(db_path).expect("cannot open database.");
        run_migrations(&conn)?;

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    pub fn register(&self, user_details: UserDetails) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        conn.execute(
            "INSERT OR REPLACE INTO user_details (user_id, integration_token, database_id)
             VALUES (?,?,?)",
            params![
                user_details.user_id,
                user_details.integration_token,
                user_details.database_id
            ],
        )?;

        Ok(())
    }

    pub fn get(&self, user_id: &str) -> Result<Option<UserDetails>> {
        let conn = self.conn.lock().unwrap();

        let mut stmt = conn.prepare(
            "SELECT user_id, integration_token, database_id
            FROM user_details
            WHERE user_id = ?1;",
        )?;

        let result = stmt.query_row(params![user_id], |row| {
            Ok(UserDetails {
                user_id: row.get(0)?,
                integration_token: row.get(1)?,
                database_id: row.get(2)?,
            })
        });

        match result {
            Ok(user_details) => Ok(Some(user_details)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }
}

fn get_db(path: &str) -> Result<Connection> {
    let db = Connection::open(path)?;
    run_migrations(&db)?;

    Ok(db)
}

fn run_migrations(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS user_details (
            user_id           TEXT NOT NULL,
            integration_token TEXT NOT NULL,
            database_id       TEXT NOT NULL,
            PRIMARY KEY (user_id)
        );",
        [],
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    use super::*;

    fn remove_db_if_exists(db_path: &str) {
        if Path::new(db_path).exists() {
            fs::remove_file(db_path).expect("Failed to remove old test database");
        }
    }

    #[test]
    fn test_db() -> Result<()> {
        let db_path = "test_db_1.sqlite";
        remove_db_if_exists(db_path);

        let db = Database::new(db_path)?;

        let user_details = UserDetails {
            user_id: "1".to_string(),
            integration_token: "2".to_string(),
            database_id: "3".to_string(),
        };

        db.register(user_details.clone())?;
        let result = db.get("1")?;

        remove_db_if_exists(db_path);

        assert_eq!(Some(user_details), result);
        Ok(())
    }

    #[test]
    fn no_record() -> Result<()> {
        let db_path = "test_db_2.sqlite";
        remove_db_if_exists(db_path);

        let db = Database::new(db_path)?;

        let result = db.get("1")?;

        remove_db_if_exists(db_path);

        assert_eq!(None, result);
        Ok(())
    }
}
