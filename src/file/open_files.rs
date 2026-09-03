use std::fs;
use std::path::{Path, PathBuf};
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
    pub fn title(&mut self) -> &str {
        if let Some(image) = self.selected_index_file() {
            &image.title()
        } else {
            ""
        }
    }

    /// 選択されたファイルのパスを取得
    /// * `return` - 選択されたファイルのパス
    pub fn selected_index_file(&mut self) -> Option<&file::Image> {
        if let Some(index) = self.selected_index {
            self.get_image_by_index(index)
        } else {
            None
        }
    }

    /// 次のファイルを取得
    /// * `offset` - オフセット
    /// * `return` - 次のファイル
    pub fn selected_next_file(&mut self, offset: usize) -> Option<&file::Image> {
        let index = self.selected_index?;
        self.get_image_by_index(index.checked_add(offset)?)
    }

    /// 前のファイルを取得
    /// * `offset` - オフセット
    /// * `return` - 前のファイル
    pub fn selected_prev_file(&mut self, offset: usize) -> Option<&file::Image> {
        let index = self.selected_index?;
        self.get_image_by_index(index.checked_sub(offset)?)
    }

    /// インデックスからファイルを取得
    /// * `index` - インデックス
    /// * `return` - ファイル
    fn get_image_by_index(&mut self, index: usize) -> Option<&file::Image> {
        if index < self.images.len() {
            let image = &mut self.images[index];

            // アーカイブの場合は、アーカイブのファイルを取得
            if image.is_archive() && image.bytes().is_empty() {
                let archive = self.archive.as_mut().unwrap();
                let archive_file = archive.get_zipfile(&image.relative_path()).unwrap();

                image.set_bytes(archive_file.bytes().to_vec().into());
            }

            Some(image)
        } else {
            None
        }
    }

    /// 前のファイルを取得
    /// * `return` - 前のファイル
    pub fn prev(&mut self, app: &app::App) -> Option<&file::Image> {
        match app.read_from() {
            event::ReadFrom::RightToLeft => self.from_prev(),
            event::ReadFrom::LeftToRight => self.from_next(),
        }

        self.selected_index_file()
    }

    /// 次のファイルを取得
    /// * `return` - 次のファイル
    pub fn next(&mut self, app: &app::App) -> Option<&file::Image> {
        match app.read_from() {
            event::ReadFrom::RightToLeft => self.from_next(),
            event::ReadFrom::LeftToRight => self.from_prev(),
        }

        self.selected_index_file()
    }

    /// 前のファイルを取得
    fn from_prev(&mut self) {
        if let Some(index) = self.selected_index {
            if index > 0 {
                self.selected_index = Some(index - 1);
            }
        }
    }

    /// 次のファイルを取得
    fn from_next(&mut self) {
        if let Some(index) = self.selected_index {
            if index < self.images.len() - 1 {
                self.selected_index = Some(index + 1);
            }
        }
    }

    /// ファイルをパス順にソート
    pub fn sort(&mut self) {
        self.images.sort_by_key(|file| file.path().clone());
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
    pub fn is_same_path(&self, path: &Path) -> bool {
        self.images.iter().any(|f| *f.path() == *path)
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
            if self.is_same_path(&path) {
                return Ok(());
            }

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
            if let Some(file_name) = file_name {
                self.selected_index = self.get_index_by_filename(&file_name);
            } else {
                self.selected_index = Some(DEFAULT_SELECTED_INDEX);
            }
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
                    let mut archive = file::Archive::new();
                    archive.unarchive(&path).map_err(|e| {
                        error::GachoError::FileError(e.to_string(), path.clone())
                    })?;

                    // TODO: アーカイブのファイルを取得、一度に全部やらないようにしたい
                    for file in archive.files() {
                        let image_file = file::Image::new(
                            path.clone(),
                            file.relative_path().clone(),
                            Some(file.file_name().clone()),
                            Some(file.bytes().to_vec()),
                        )?;

                        self.images.push(image_file);
                    }

                    self.archive = Some(archive);
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
