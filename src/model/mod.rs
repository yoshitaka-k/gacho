mod book;

pub(crate) use book::Book;

use rusqlite::Connection;

use crate::APP_NAME;
const DB_NAME: &str = "data.db";

/// モデル
pub(crate) struct Model {
    conn: Connection,
}

impl Model {
    /// 新しいモデルを作成
    pub fn new() -> Self {
        let dir = eframe::storage_dir(APP_NAME).expect("Failed to get storage directory");
        std::fs::create_dir_all(&dir).expect("Failed to create storage directory");
        let path = dir.join(DB_NAME);
        let conn = Connection::open(path).expect("Failed to open SQLite connection");

        conn.execute(
            "CREATE TABLE IF NOT EXISTS books (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                path TEXT NOT NULL,
                page INTEGER NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
            )",
            [],
        );

        Self { conn }
    }
}
