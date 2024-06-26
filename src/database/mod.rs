use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use crate::hand::Hand;

pub struct Database {
    conn: SqliteConnection,
}

impl Database {
    pub fn new(database_url: &str) -> Self {
        let conn = SqliteConnection::establish(database_url)
            .expect(&format!("Error connecting to {}", database_url));
        Database { conn }
    }

    pub fn insert_hand(&self, _hand: &Hand) {
        // Implementation
    }
}