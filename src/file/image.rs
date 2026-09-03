use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use getset::{Getters, Setters};

use crate::{error, file};

/// Image の一意な ID を発行するカウンタ
static NEXT_ID: AtomicU64 = AtomicU64::new(1);

/// 画像ファイルを管理する構造体
#[derive(Clone, Getters, Setters)]
pub struct Image {
    /// ファイルの一意な ID
    #[getset(get = "pub")]
    id: u64,

    /// ファイルのパス
    #[getset(get= "pub")]
    path: PathBuf,

    /// 本の名前
    #[getset(get = "pub")]
    title: String,

    /// 画像ファイルの名前
    #[getset(get = "pub")]
    file_name: String,

    /// ファイルの拡張子
    #[getset(get = "pub")]
    extension: file::Extension,

    /// ファイルのバイト列
    #[getset(get = "pub")]
    bytes: Arc<[u8]>,
}

impl Image {
    /// 新しい Image を作成
    /// * `path` - ファイルのパス
    /// * `relative_path` - ドロップ基準からの相対パス
    /// * `file_name` - ファイルの名前
    /// * `return` - Image のインスタンス
    pub fn new(path: PathBuf, relative_path: String, file_name: Option<String>, bytes: Option<Vec<u8>>) -> Result<Self, error::GachoError> {
        // ファイル名を取得
        let file_name = if let Some(file_name) = file_name {
            file_name
        } else {
            if let Some(name) = path.file_name() {
                name.to_string_lossy().to_string()
            } else {
                return Err(error::GachoError::FileError("File name not found".to_string(), path.clone()));
            }
        };

        // ファイル拡張子を取得
        let extension = if let Some(ext) = path.extension() {
            file::Extension::from_str(ext)
        } else {
            return Err(error::GachoError::FileError("File extension not found".to_string(), path.clone()));
        };

        // 本の名前を取得
        let title = if matches!(extension, file::Extension::Zip | file::Extension::Cbz) {
            let mut path = path.clone();
            path.set_extension("");
            if let Some(name) = path.file_name() {
                name.to_string_lossy().to_string()
            } else {
                file_name.clone()
            }
        } else if path.is_file() {
            if let Some(parent) = path.parent() {
                if let Some(parent_name) = parent.file_name() {
                    parent_name.to_string_lossy().to_string()
                } else {
                    file_name.clone()
                }
            } else {
                file_name.clone()
            }
        } else {
            let replace_path = format!("/{}", file_name);
            relative_path.replace(&replace_path, "")
        };

        let title = file::extract_book_title(&title);

        // ファイルの内容を取得
        let bytes: Arc<[u8]> = if let Some(bytes) = bytes {
            Arc::from(bytes)
        } else {
            Arc::new([])
        };

        // ファイルの一意な ID を発行
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);

        Ok(Self {
            id,
            path,
            title,
            file_name,
            extension,
            bytes,
        })
    }

    /// ファイルがアーカイブかどうかを判断
    /// * `return` - ファイルがアーカイブかどうか
    pub fn is_archive(&self) -> bool {
        matches!(self.extension, file::Extension::Zip | file::Extension::Cbz)
    }
}
