use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use getset::{Getters, Setters};
use image::ImageReader;

use crate::{error, file};

/// Image の一意な ID を発行するカウンタ
static NEXT_ID: AtomicU64 = AtomicU64::new(1);

/// 画像ファイルを管理する構造体
#[derive(Clone, Getters, Setters)]
#[getset(get = "pub")]
pub struct Image {
    /// ファイルの一意な ID
    id: u64,

    /// ファイルのパス
    path: PathBuf,

    /// ファイルの相対パス
    relative_path: String,

    /// 画像ファイルの名前
    file_name: String,

    /// ファイルの拡張子
    extension: file::Extension,

    /// 画像ファイルの幅・高さ
    #[getset(set = "pub")]
    size: egui::Vec2,

    /// ファイルのバイト列
    #[getset(set = "pub")]
    bytes: Arc<[u8]>,

    /// アーカイブ内のファイルのインデックス
    archive_index: usize,
}

impl Image {
    /// 新しい Image を作成
    /// * `path` - ファイルのパス
    /// * `relative_path` - ドロップ基準からの相対パス
    /// * `file_name` - ファイルの名前
    /// * `return` - Image のインスタンス
    pub fn new(
        path: PathBuf,
        relative_path: String,
        file_name: Option<String>,
        bytes: Option<Vec<u8>>,
        archive_index: Option<usize>,
    ) -> Result<Self, error::GachoError> {
        // ファイル名を取得
        let file_name = if let Some(file_name) = file_name {
            file_name
        } else {
            if let Some(name) = path.file_name() {
                name.to_string_lossy().to_string()
            } else {
                return Err(error::GachoError::FileError(
                    "File name not found".to_string(), path.clone()
                ));
            }
        };

        // ファイル拡張子を取得
        let extension = if let Some(ext) = path.extension() {
            file::Extension::from_str(ext)
        } else {
            return Err(error::GachoError::FileError(
                "File extension not found".to_string(), path.clone()
            ));
        };

        // ファイルの幅・高さを取得
        let size = if file::is_image(&path) {
            let reader = ImageReader::open(&path).map_err(|e| {
                error::GachoError::FileError(e.to_string(), path.clone())
            })?;
            let (width, height) = reader.into_dimensions().map_err(|e| {
                error::GachoError::FileError(e.to_string(), path.clone())
            })?;

            egui::Vec2::new(width as f32, height as f32)
        } else {
            egui::Vec2::new(0.0, 0.0)
        };

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
            relative_path,
            file_name,
            extension,
            size,
            bytes,
            archive_index: archive_index.unwrap_or(0),
        })
    }

    /// ファイルがアーカイブかどうかを判断
    /// * `return` - ファイルがアーカイブかどうか
    pub fn is_archive(&self) -> bool {
        matches!(self.extension, file::Extension::Zip | file::Extension::Cbz)
    }

    /// ファイルのバイト列が空かどうかを判断
    /// * `return` - ファイルのバイト列が空かどうか
    pub fn is_empty_bytes(&self) -> bool {
        self.bytes.is_empty()
    }

    /// 指定した領域に収まる表示サイズを返す
    /// * `max` - 最大サイズ
    /// * `return` - 表示サイズ
    pub fn fit_to(&self, max: egui::Vec2) -> egui::Vec2 {
        if self.size.x <= 0.0 || self.size.y <= 0.0 {
            return max;
        }

        // 比率を計算
        let ratio = (max.x / self.size.x).min(max.y / self.size.y);

        // 比率が有限な場合は表示サイズを計算
        if ratio.is_finite() {
            self.size * ratio
        } else {
            max
        }
    }
}
