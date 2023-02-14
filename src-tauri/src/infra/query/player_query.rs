use rusqlite::{Connection, Result};

use crate::services::{dto::player_dto::PlayerDTO, query::player_query_trait::PlayerQueryService};

pub struct SqlitePlayerQuery {
    conn: Connection,
}

impl SqlitePlayerQuery {
    pub fn new(conn: Connection) -> Self {
        SqlitePlayerQuery { conn }
    }
}

impl PlayerQueryService for SqlitePlayerQuery {
    fn fetch_by_id(&self, id: String) -> Result<Vec<PlayerDTO>> {
        let mut stmt = self.conn.prepare("SELECT * FROM D_PLAYER WHERE id = ?")?;
        let person_iter = stmt.query_map([id], |row| {
            Ok(PlayerDTO {
                id: row.get(0)?,
                name: row.get(1)?,
                role: row.get(2)?,
                height: row.get(3)?,
                weight: row.get(4)?,
                number: row.get(5)?,
                status: row.get(6)?,
            })
        })?;

        let mut result_vec = Vec::new();
        for person in person_iter {
            result_vec.push(person?);
        }

        Ok(result_vec)
    }
}
