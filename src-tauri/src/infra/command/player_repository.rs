use rusqlite::{Connection, Result};



fn open_my_db() -> Result<Connection,rusqlite::Error> {
    let path = "";
    let con = Connection::open(&path)?;
    println!("{}", con.is_autocommit());
    Ok(con)
}