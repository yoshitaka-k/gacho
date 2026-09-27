use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use getset::{Getters, Setters};
use crate::{error, file};

/// Image の一意な ID を発行するカウンタ
static NEXT_ID: AtomicU64 = AtomicU64::new(1);

/// ライブラリのエントリ
#[derive(Clone, Getters, Setters)]
pub(crate) struct LibraryEntry {
    /// ファイルの一意な ID
    #[allow(unused)]
    id: u64,

    /// ファイルのパス
    #[getset(get = "pub")]
    path: PathBuf,

    /// ファイルの名前
    #[allow(unused)]
    file_name: String,

    /// 最後に読んだページ
    #[getset(set = "pub", get = "pub")]
    last_page: Option<usize>,
}

/// public methods
impl LibraryEntry {
    /// 新しいライブラリのエントリを作成
    /// * `path` - ファイルのパス
    /// * `file_name` - ファイルの名前
    /// * `return` - ライブラリのエントリ
    pub fn new(path: PathBuf, file_name: String, last_page: Option<usize>) -> Self {
        // ファイルの一意な ID を発行
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);

        Self { id, path, file_name, last_page }
    }
}

/// ライブラリの構造体
pub struct Library {
    entries: Vec<LibraryEntry>,
}

impl Library {
    /// 新しいライブラリを作成
    /// * `return` - ライブラリ
    pub fn new() -> Self {
        Self { entries: vec![] }
    }

    /// ライブラリのエントリ数を取得
    /// * `return` - エントリ数
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// ライブラリをクリア
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// インデックスでエントリを取得
    /// * `index` - インデックス
    /// * `return` - エントリ
    pub fn get(&self, index: usize) -> Option<&LibraryEntry> {
        self.entries.get(index)
    }

    /// インデックスでエントリを取得
    /// * `index` - インデックス
    /// * `return` - エントリ
    pub fn get_mut(&mut self, index: usize) -> Option<&mut LibraryEntry> {
        self.entries.get_mut(index)
    }

    /// パスで index を取得
    /// * `path` - パス
    /// * `return` - index
    pub fn get_index_by_path(&self, path: &PathBuf) -> Option<usize> {
        // パスが一致するindexを取得
        self.entries.iter().position(|entry| {
            entry.path == *path
        })
    }

    /// ライブラリにエントリを追加
    /// * `path` - ファイルのパス
    /// * `books_last_page` - 本のリスト
    pub fn build_entries(&mut self, path: PathBuf, books_last_page: &Vec<(PathBuf, usize)>) -> error::Result<()> {
        // ベースディレクトリを取得
        let base_dir = path.parent().unwrap_or(&path).to_path_buf();

        // ファイルが画像ファイルの場合は
        // ベースディレクトリのファイル名をライブラリに追加
        if file::is_image(&path) {
            // ベースディレクトリがすでにライブラリに追加されている場合はスキップ
            let index = self.get_index_by_path(&base_dir);
            if index.is_some() { return Ok(()); }

            let file_name = self.get_file_name(&base_dir);
            self.entries.push(LibraryEntry::new(base_dir.clone(), file_name, None));

            // ファイルをソート
            self.sort();

            return Ok(());
        }

        // ファイルがすでにライブラリに追加されている場合はスキップ
        let index = self.get_index_by_path(&path);
        if index.is_some() { return Ok(()); }

        // ファイルを探索
        self.find_file(&path, &base_dir, books_last_page)?;

        // ファイルをソート
        self.sort();

        Ok(())
    }
}

/// private methods
impl Library {
    /// ファイルをソート
    fn sort(&mut self) {
        let mut temp_paths = Vec::new();
        for entry in &self.entries {
            temp_paths.push(entry.path.clone());
        }

        let sorted_paths = file::sort_path(&temp_paths);

        let mut sorted_entries: Vec<LibraryEntry> = Vec::new();
        for path in &sorted_paths {
            let entry = self.entries.iter().find(|e| *e.path() == *path).unwrap();
            sorted_entries.push(entry.clone());
        }

        self.entries = sorted_entries;
    }

    /// ファイルを探索
    /// * `path` - ファイルのパス
    /// * `base_dir` - ベースディレクトリのパス
    /// * `books_last_page` - 本のリスト
    /// * `return` - 結果
    fn find_file(&mut self, path: &PathBuf, base_dir: &PathBuf, books_last_page: &Vec<(PathBuf, usize)>) -> error::Result<()> {
        // メタデータを取得
        let metadata = path.metadata().map_err(|e| {
            error::GachoError::FileError(e.to_string(), path.clone())
        })?;

        // ファイルの場合は、同ディレクトリのファイルを探索してライブラリに追加
        if metadata.is_file() {
            // 同ディレクトリのファイルを探索
            for entry in std::fs::read_dir(&base_dir).map_err(|e| {
                error::GachoError::FileError(e.to_string(), base_dir.clone())
            })? {
                let entry = entry.map_err(|e| {
                    error::GachoError::FileError(e.to_string(), base_dir.clone())
                })?;

                // アーカイブ以外の場合はスキップ
                if !file::is_archive(&entry.path()) { continue; }

                // ファイルの名前を取得
                let file_name = self.get_file_name(&entry.path());

                // 最後に読んだページを取得
                let last_page = books_last_page.iter().find(|(p, _)| p == &entry.path()).map(|(_, p)| p).copied();

                // ライブラリに追加
                self.entries.push(
                    LibraryEntry::new(entry.path().clone(), file_name, last_page)
                );
            }

        // ディレクトリの場合は、ディレクトリの中のファイルを探索してライブラリに追加
        } else if metadata.is_dir() {
            // ディレクトリの中のファイルを探索
            for entry in std::fs::read_dir(&path).map_err(|e| {
                error::GachoError::FileError(e.to_string(), path.clone())
            })? {
                let entry = entry.map_err(|e| {
                    error::GachoError::FileError(e.to_string(), path.clone())
                })?;

                // アーカイブ以外の場合はスキップ
                if !file::is_archive(&entry.path()) { continue; }

                // ファイルの名前を取得
                let file_name = self.get_file_name(&entry.path());

                // 最後に読んだページを取得
                let last_page = books_last_page.iter().find(|(p, _)| p == &entry.path()).map(|(_, p)| p).copied();

                // ライブラリに追加
                self.entries.push(
                    LibraryEntry::new(entry.path().clone(), file_name, last_page)
                );
            }

            // ディレクトリで何も追加されていない場合は、ディレクトリのパスをライブラリに追加
            if self.entries.is_empty() {
                self.entries.push(
                    LibraryEntry::new(path.clone(), self.get_file_name(&path), None)
                );
            }
        }

        Ok(())
    }

    /// ファイルの名前を取得
    /// * `path` - ファイルのパス
    /// * `return` - ファイルの名前
    fn get_file_name(&self, path: &PathBuf) -> String {
        if let Some(file_name) = path.file_name() {
            file_name.to_string_lossy().to_string()
        } else {
            "".to_string()
        }
    }
}
