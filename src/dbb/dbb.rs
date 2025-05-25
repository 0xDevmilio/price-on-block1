use anyhow::Result;
use duckdb::{params, Connection};

#[derive(Debug)]
pub struct DbHandler {
    conn: Connection,
}

impl DbHandler {
    pub fn new() -> Result<Self> {
        let conn = Connection::open("my_database.db")?;
        Ok(Self { conn })
    }

    pub fn create_tables(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS prices (
                token TEXT NOT NULL UNIQUE,
                price FLOAT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    pub fn insert_price(&self, token: &str, price: f32) -> Result<()> {
        self.conn.execute(
            "INSERT INTO prices (token, price) 
             VALUES (?, ?) 
             ON CONFLICT(token) DO UPDATE SET price = excluded.price",
            params![token, price],
        )?;
        Ok(())
    }

    pub fn get_prices(&self, token: &str) -> Result<Vec<f32>> {
        let mut stmt = self
            .conn
            .prepare("SELECT price FROM prices WHERE token = ?")?;
        let prices = stmt
            .query_map([token], |row| row.get(0))?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(prices)
    }
}

#[derive(Debug)]
pub struct Prices {
    token: String,
    price: f32,
}
