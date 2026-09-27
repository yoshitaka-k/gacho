use std::path::PathBuf;
use rusqlite::{Connection, OptionalExtension};
use crate::{error, model};

/// 本のモデル
pub(crate) struct Book {
    conn: Connection,
}

/// public methods
impl Book {
    /// 新しい本のモデルを作成
    pub fn new() -> Self {
        Self {
            conn: model::Model::connection(),
        }
    }

    /// 最後に読んだページを保存
    /// * `path`: 本のパス
    /// * `page`: 最後に読んだページ
    /// * `return`: 成功したかどうか
    pub fn save_last_page(&mut self, path: &PathBuf, page: usize) -> error::Result<bool> {
        let id = self.get_id(path)?;
        if id.is_none() {
            self.insert_page(path, page)
        } else {
            self.update_page(id.unwrap(), page)
        }
    }
}

/// private methods
impl Book {
    /// 最後に読んだページを取得
    /// * `path`: 本のパス
    /// * `return`: 本ID
    fn get_id(&self, path: &PathBuf) -> error::Result<Option<usize>> {
        let path = path.to_string_lossy().to_string();
        let mut stmt = self.conn
            .prepare("SELECT `id` FROM `books` WHERE `path` = ?")
            .map_err(|e| error::GachoError::DatabaseError(e.to_string()))?;

        // 該当行がなければ None にする
        let id: Option<i64> = stmt
            .query_row([&path], |row| row.get(0))
            .optional()
            .map_err(|e| error::GachoError::DatabaseError(e.to_string()))?;

        Ok(id.map(|id| id as usize))
    }

    /// 最後に読んだページを保存
    /// * `path`: 本のパス
    /// * `page`: 最後に読んだページ
    /// * `return`: 成功したかどうか
    fn insert_page(&mut self, path: &PathBuf, page: usize) -> error::Result<bool> {
        // パラメータを準備
        let path = path.to_string_lossy().to_string();
        let created_at = chrono::Local::now().to_string();
        let updated_at = chrono::Local::now().to_string();

        // SQL を準備
        let mut stmt = self.conn.prepare(
            "INSERT INTO `books` (
                `path`,
                `page`,
                `created_at`,
                `updated_at`
            ) VALUES (?, ?, ?, ?)"
        ).map_err(|e| error::GachoError::DatabaseError(e.to_string()))?;

        // 最後に読んだページを保存
        stmt.execute([
            &path,
            &page.to_string(),
            &created_at,
            &updated_at
        ]).map_err(|e| error::GachoError::DatabaseError(e.to_string()))?;

        // println!("{}", stmt.expanded_sql().unwrap_or_default());

        Ok(true)
    }

    /// 最後に読んだページを更新
    /// * `id`: 本のID
    /// * `page`: 最後に読んだページ
    /// * `return`: 成功したかどうか
    fn update_page(&mut self, id: usize, page: usize) -> error::Result<bool> {
        // パラメータを準備
        let updated_at = chrono::Local::now().to_string();

        // SQL を準備
        let mut stmt = self.conn.prepare(
            "UPDATE `books` SET `page` = ?, `updated_at` = ? WHERE `id` = ?"
        ).map_err(|e| error::GachoError::DatabaseError(e.to_string()))?;

        // 最後に読んだページを更新
        stmt.execute([&page.to_string(), &updated_at, &id.to_string()]).map_err(|e| {
            error::GachoError::DatabaseError(e.to_string())
        })?;

        // println!("{}", stmt.expanded_sql().unwrap_or_default());

        Ok(true)
    }
}
