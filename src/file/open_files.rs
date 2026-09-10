use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashSet;
use getset::{Getters, MutGetters, Setters};

use crate::{app, event, file, error};

const DEFAULT_SELECTED_INDEX: usize = 0;

/// ドロップされたファイルを管理する構造体
#[derive(Getters, MutGetters, Setters)]
pub struct OpenFiles {
    /// ファイル一覧
    #[getset(get = "pub")]
    images: Vec<file::Image>,

    /// アーカイブ
    archive: Option<file::Archive>,

    /// 選択されたファイルのインデックス
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    selected_index: Option<usize>,
}

impl OpenFiles {
    /// 新しい OpenFiles を作成
    /// * `return` - OpenFiles のインスタンス
    pub fn new() -> Self {
        Self {
            images: vec![],
            archive: None,
            selected_index: None,
        }
    }

    pub fn len(&self) -> usize {
        self.images.len()
    }

    /// ファイルをクリア
    pub fn clear(&mut self) {
        self.images.clear();
        self.archive = None;
        self.selected_index = None;
    }

    /// 本の名前を取得
    /// * `return` - 本の名前
    pub fn title(&mut self) -> error::Result<&str> {
        let Some(index) = self.selected_index else { return Ok(""); };
        if index >= self.images.len() { return Ok(""); }

        Ok(self.images[index].title())
    }

    /// ファイルのIDを取得
    /// * `return` - ファイルのIDリスト
    pub fn image_ids(&self) -> Vec<u64> {
        self.images.iter().map(|image| *image.id()).collect()
    }

    /// 選択されたファイルのパスを取得
    /// * `return` - 選択されたファイルのパス
    pub fn selected_index_files(&mut self, app: &app::App) -> error::Result<Vec<file::Image>> {
        let Some(index) = self.selected_index else { return Ok(vec![]); };

        let mut images = vec![];
        match app.page_layout() {
            event::PageLayout::Default | event::PageLayout::Single => {
                let image = self.ensure_image_by_index(index)?;
                if let Some(image) = image {
                    images.push(image);
                }
            }
            event::PageLayout::Spread => {
                for offset in 0..=app.page_layout().to_offset() {
                    let Some(add_index) = index.checked_add(offset) else { continue; };
                    let image = self.ensure_image_by_index(add_index)?;
                    if let Some(image) = image {
                        images.push(image);
                    }
                }

                // 重複を削除
                let mut seen = HashSet::new();
                let unique_images = images.into_iter()
                    .filter(|image| seen.insert(*image.id()))
                    .collect();

                images = unique_images;
            }
        }

        Ok(images)
    }

    /// 指定した次のインデックスを移動させてファイルを取得
    /// * `offset` - オフセット
    /// * `return` - 次のファイル
    pub fn selected_next_file(&mut self, offset: usize) -> error::Result<Option<file::Image>> {
        let Some(index) = self.selected_index else { return Ok(None); };
        let Some(add_index) = index.checked_add(offset) else { return Ok(None); };

        self.ensure_image_by_index(add_index)
    }

    /// 指定した前のインデックスを移動させてファイルを取得
    /// * `offset` - オフセット
    /// * `return` - 前のファイル
    pub fn selected_prev_file(&mut self, offset: usize) -> error::Result<Option<file::Image>> {
        let Some(index) = self.selected_index else { return Ok(None); };
        let Some(sub_index) = index.checked_sub(offset) else { return Ok(None); };

        self.ensure_image_by_index(sub_index)
    }

    /// 次のインデックス
    /// * `app` - アプリケーション
    /// * `return` - 次のインデックス
    pub fn next_index(&mut self, app: &app::App) -> error::Result<Option<usize>> {
        match app.read_from() {
            event::ReadFrom::RightToLeft => self.from_next(app.page_layout().to_offset()),
            event::ReadFrom::LeftToRight => self.from_prev(app.page_layout().to_offset()),
        }

        // 見開き
        if matches!(app.page_layout(), event::PageLayout::Spread) {
            if let Some(index) = self.selected_index {
                // 最後のページの調整
                if index % 2 != 0 {
                    self.selected_index = Some(index - 1);
                }
            }
        }

        Ok(self.selected_index)
    }

    /// 前のインデックス
    // 読み込み方向によって前後が変わる
    /// * `return` - 前のインデックス
    pub fn prev_index(&mut self, app: &app::App) -> error::Result<Option<usize>> {
        match app.read_from() {
            event::ReadFrom::RightToLeft => self.from_prev(app.page_layout().to_offset()),
            event::ReadFrom::LeftToRight => self.from_next(app.page_layout().to_offset()),
        }

        // 見開き
        if matches!(app.page_layout(), event::PageLayout::Spread) {
            if let Some(index) = self.selected_index {
                // 最後のページの調整
                if index % 2 != 0 {
                    self.selected_index = Some(index - 1);
                }
            }
        }

        Ok(self.selected_index)
    }

    /// パスを追加
    /// * `path` - ドロップされたファイルのパス
    /// * `return` - 結果
    pub fn add_path(&mut self, path: PathBuf) -> error::Result<()> {
        // parent() は path を借りるので、先に PathBuf にして借用を終わらせる
        let base_dir = path.parent().unwrap_or(&path).to_path_buf();

        // ファイル名の控えを用意
        let mut file_name = None;

        if file::is_image(&path) {
            // パスが同じかどうかを判断
            if self.is_same_path(&path) { return Ok(()); }

            // 画像ファイルの場合は、同ディレクトリの他の画像ファイルも検索する
            if let Some(parent) = path.parent() {
                self.find_file(parent.to_path_buf(), &base_dir)?;

                // 画像ファイルの場合は、ファイル名を控えておく
                file_name = if let Some(file_name) = path.file_name() {
                    Some(file_name.to_string_lossy().into_owned())
                } else {
                    None
                };
            }
        } else {
            // ファイルを検索
            self.find_file(path, &base_dir)?;
        }

        // ファイルをパス順にソート
        self.sort();

        if !self.images.is_empty() {
            // 画像ファイルから開かれたら
            if let Some(file_name) = file_name {
                self.selected_index = self.get_index_by_filename(&file_name);
            } else {
                self.selected_index = Some(DEFAULT_SELECTED_INDEX);
            }
        }

        Ok(())
    }

    /// ファイルをパス順にソート
    fn sort(&mut self) {
        self.images.sort_by_key(|file| file.path().clone());
    }

    /// インデックスからファイルを取得
    /// * `index` - インデックス
    /// * `return` - Image のインスタンス
    fn ensure_image_by_index(&mut self, index: usize) -> error::Result<Option<file::Image>> {
        // インデックスが範囲外の場合は None を返す
        if index >= self.images.len() { return Ok(None); }
        let image = self.images.get_mut(index).ok_or_else(|| {
            error::GachoError::IndexError(index)
        })?;

        // アーカイブの場合は、アーカイブからファイルのバイト列を取得
        if image.is_archive() && image.is_empty_bytes() {
            let archive = self.archive.as_mut().ok_or_else(|| {
                error::GachoError::ArchiveError(
                    format!("Archive not found: {}", image.relative_path())
                )
            })?;

            let archive_file = archive.get_zipfile(*image.archive_index()).map_err(|e| {
                error::GachoError::ArchiveError(
                    format!("{} : {}", e.to_string(), image.relative_path())
                )
            })?;

            image.set_bytes(archive_file.bytes().to_vec().into());
            image.set_size(*archive_file.size());
        }

        Ok(Some(image.clone()))
    }

    /// 次のファイルを取得
    fn from_next(&mut self, offset: usize) {
        if let Some(index) = self.selected_index {
            if index + offset < self.images.len() - 1 {
                self.selected_index = Some(index + offset + 1);
            } else {
                self.selected_index = Some(self.images.len() - 1);
            }
        }
    }

    /// 前のファイルを取得
    fn from_prev(&mut self, offset: usize) {
        if let Some(index) = self.selected_index {
            if (index as isize - offset as isize) > 0 {
                self.selected_index = Some(index - offset - 1);
            } else {
                self.selected_index = Some(0);
            }
        }
    }

    /// ファイル名でindexを取得
    /// * `file_name` - ファイル名
    /// * `return` - index
    fn get_index_by_filename(&self, name: &str) -> Option<usize> {
        // ファイル名が一致するindexを取得
        self.images.iter().position(|file| {
            let file_name = if let Some(file_name) = file.path().file_name() {
                file_name.to_string_lossy().into_owned()
            } else {
                return false;
            };
            file_name == name
        })
    }

    /// パスが同じかどうかを判断
    /// * `path` - パス
    /// * `return` - パスが同じかどうか
    fn is_same_path(&self, path: &Path) -> bool {
        self.images.iter().any(|f| *f.path() == *path)
    }

    /// ファイルを検索
    /// * `path` - ドロップされたファイルのパス
    /// * `base_dir` - ドロップされたパスの親（相対パスの基準）
    /// * `return` - ファイルのパス
    fn find_file(&mut self,
        path: PathBuf,
        base_dir: &Path,
    ) -> error::Result<()> {
        let metadata = path.metadata().map_err(|e| {
            error::GachoError::FileError(e.to_string(), path.clone())
        })?;

        if metadata.is_file() {
            // strip_prefix は path を借りるので、
            // 先に String にして into_owned()で所有権を移す
            let relative_path = path.strip_prefix(base_dir)
                .unwrap_or(&path)
                .to_string_lossy()
                .into_owned();

            // ファイルがアーカイブかどうかを判断
            if file::is_archive(&path) && self.images.is_empty() {
                let mut archive = file::Archive::new();
                archive.unarchive(&path).map_err(|e| {
                    error::GachoError::ArchiveError(e.to_string())
                })?;

                for file in archive.files() {
                    let image_file = file::Image::new(
                        path.clone(),
                        file.relative_path().clone(),
                        Some(file.file_name().clone()),
                        Some(file.bytes().to_vec()),
                        Some(*file.index()),
                    )?;

                    self.images.push(image_file);
                }

                self.archive = Some(archive);
            } else if file::is_image(&path) {
                // ファイルを作成
                let image_file = file::Image::new(
                    path.clone(),
                    relative_path,
                    None,
                    None,
                    None,
                )?;

                // ファイルを追加
                self.images.push(image_file);
            }
        } else if metadata.is_dir() {
            // ディレクトリの中のファイルを探索
            for entry in fs::read_dir(&path).map_err(|e| {
                error::GachoError::FileError(e.to_string(), path.clone())
            })? {
                let entry = entry.map_err(|e| {
                    error::GachoError::FileError(e.to_string(), path.clone())
                })?;

                let metadata = entry.path().metadata().map_err(|e| {
                    error::GachoError::FileError(e.to_string(), path.clone())
                })?;

                // ディレクトリの中のディレクトリはスキップ
                if metadata.is_dir() {
                    continue;
                }

                self.find_file(entry.path(), base_dir)?;
            }
        }

        Ok(())
    }
}
