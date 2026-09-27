use rusqlite::Connection;
use rusqlite_migration::{Migrations, M};
use crate::error;

/// マイグレーション
pub(crate) struct Migration;
impl Migration {
    /// マイグレーションを実行
    /// * `conn`: データベース接続
    /// * `return`: 成功したかどうか
    pub fn migrate(conn: &mut Connection) -> error::Result<()> {
        let migrations = Migrations::new(vec![
            // v1.1.0 本のテーブルを作成
            M::up("CREATE TABLE IF NOT EXISTS `books` (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                `path` TEXT NOT NULL,
                `page` INTEGER NOT NULL,
                `created_at` TEXT NOT NULL,
                `updated_at` TEXT NOT NULL
            )"),
        ]);

        migrations.to_latest(conn).map_err(|e| {
            error::GachoError::database_error(e.to_string())
        })?;

        Ok(())
    }
}
