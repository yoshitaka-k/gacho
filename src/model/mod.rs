mod migration;
mod book;

pub(crate) use book::Book;


use rusqlite::Connection;
use crate::APP_NAME;

const DB_NAME: &str = "data.db";

pub(crate) struct Model;
impl Model {
    /// 新しいモデルを作成
    pub fn connection() -> Connection {
        // データベースファイルのパスを取得
        let dir = eframe::storage_dir(APP_NAME).expect("Failed to get storage directory");
        std::fs::create_dir_all(&dir).expect("Failed to create storage directory");
        let path = dir.join(DB_NAME);

        // sqlite 接続
        let mut conn = Connection::open(path).expect("Failed to open SQLite connection");

        // マイグレーション
        migration::Migration::migrate(&mut conn).expect("Failed to migrate");

        conn
    }
}
