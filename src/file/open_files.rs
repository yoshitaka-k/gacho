use std::fs;
use std::path::{Path, PathBuf};
use getset::{Getters, Setters};

use crate::{file, error};

/// ドロップされたファイルを管理する構造体
#[derive(Getters, Setters)]
pub struct OpenFiles {
    /// ファイル一覧
    #[getset(get = "pub")]
    images: Vec<file::Image>,

    /// 選択されたファイルのインデックス
    #[getset(get = "pub", set = "pub")]
    selected_index: Option<usize>,
}

impl OpenFiles {
    /// 新しい OpenFiles を作成
    /// * `return` - OpenFiles のインスタンス
    pub fn new() -> Self {
        Self {
            images: vec![],
            selected_index: None,
        }
    }

    /// ファイルをクリア
    pub fn clear(&mut self) {
        self.images.clear();
        self.selected_index = None;
    }

    /// 選択されたファイルのパスを取得
    /// * `return` - 選択されたファイルのパス
    pub fn selected_index_file(&self) -> Option<&file::Image> {
        if let Some(index) = self.selected_index() {
            Some(&self.images[*index])
        } else {
            None
        }
    }

    /// 次のファイルを取得
    /// * `return` - 次のファイル
    pub fn selected_next_file(&mut self) -> Option<&file::Image> {
        let Some(index) = self.selected_index() else {
            return None;
        };

        if *index < self.images.len() - 1 {
            return Some(&self.images[*index + 1]);
        }

        None
    }

    /// 前のファイルを取得
    /// * `return` - 前のファイル
    pub fn selected_prev_file(&mut self) -> Option<&file::Image> {
        let Some(index) = self.selected_index() else {
            return None;
        };

        if *index > 0 {
            return Some(&self.images[*index - 1]);
        }

        None
    }

    /// 前のファイルを取得
    /// * `return` - 前のファイル
    pub fn prev(&mut self) -> Option<&file::Image> {
        if let Some(index) = self.selected_index() {
            if *index > 0 {
                self.selected_index = Some(*index - 1);
            }
        }

        self.selected_index_file()
    }

    /// 次のファイルを取得
    /// * `return` - 次のファイル
    pub fn next(&mut self) -> Option<&file::Image> {
        if let Some(index) = self.selected_index() {
            if *index < self.images.len() - 1 {
                self.selected_index = Some(*index + 1);
            }
        }

        self.selected_index_file()
    }

    /// ファイルをパス順にソート
    pub fn sort(&mut self) {
        self.images.sort_by_key(|file| file.path().clone());
    }

    /// パスを追加
    /// * `path` - ドロップされたファイルのパス
    /// * `return` - 結果
    pub fn add_path(&mut self, path: PathBuf) -> error::Result<()> {
        // parent() は path を借りるので、先に PathBuf にして借用を終わらせる
        let base_dir = path.parent().unwrap_or(&path).to_path_buf();

        // ファイルを検索
        self.find_file(path, &base_dir)?;

        // ファイルをパス順にソート
        self.sort();

        // TODO: とりあえず最初のファイルを選択するようにする
        if !self.images.is_empty() {
            self.selected_index = Some(0);
        }

        Ok(())
    }

    /// ファイルを検索
    /// * `path` - ドロップされたファイルのパス
    /// * `base_dir` - ドロップされたパスの親（相対パスの基準）
    /// * `return` - ファイルのパス
    fn find_file(&mut self,
        path: PathBuf,
        base_dir: &Path,
    ) -> error::Result<()> {
        let metadata = path.metadata().map_err(|e| error::GachoError::FileError(e.to_string(), path.clone()))?;

        if metadata.is_file() {
            if file::is_allowed_extension(&path) {
                // strip_prefix は path を借りるので、
                // 先に String にして into_owned()で所有権を移す
                let relative_path = path.strip_prefix(base_dir)
                    .unwrap_or(&path)
                    .to_string_lossy()
                    .into_owned();

                // ファイルがアーカイブかどうかを判断
                if file::is_archive(&path) {
                    // TODO: アーカイブ
                    let mut archive = file::Archive::new();

                    // TODO: アーカイブを展開
                    archive.unarchive(&path).map_err(|e| error::GachoError::FileError(e.to_string(), path.clone()))?;

                    for file in archive.files() {
                        let image_file = file::Image::new(
                            path.clone(),
                            file.relative_path().clone(),
                            Some(file.file_name().clone()),
                            Some(file.bytes().to_vec()),
                        )?;
                        self.images.push(image_file);
                    }

                    archive.sort();
                } else {
                    // ファイルを作成
                    let image_file = file::Image::new(path.clone(), relative_path, None, None)?;

                    // ファイルを追加
                    self.images.push(image_file);
                }
            }
        } else if metadata.is_dir() {
            // ディレクトリを再帰的に探索
            for entry in fs::read_dir(&path).map_err(|e| error::GachoError::FileError(e.to_string(), path.clone()))? {
                let entry = entry.map_err(|e| error::GachoError::FileError(e.to_string(), path.clone()))?;
                self.find_file(entry.path(), base_dir)?;
            }
        }

        Ok(())
    }
}
