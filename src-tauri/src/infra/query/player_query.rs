use crate::{
    models::player::player_entity::Player,
    services::{dto::player_dto::PlayerDTO, query::player_query_trait::PlayerQueryService},
};

use rusqlite::{params, Connection, OptionalExtension};

pub struct SqlitePlayerQuery {
    conn: Connection,
}

impl SqlitePlayerQuery {
    pub fn new(conn: Connection) -> Self {
        SqlitePlayerQuery { conn }
    }
}

impl PlayerQueryService for SqlitePlayerQuery {
    fn fetch_by_id(&self, id: String) -> Vec<PlayerDTO> {
        let players = self
            .conn
            .query_row(
                "SELECT id, name FROM players WHERE id = ?1",
                params![id],
                |row| Ok(Player::new(row.get(0)?, row.get(1)?)),
            )
            .optional()?;
        Ok(players)
    }
}
