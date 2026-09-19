use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use getset::Getters;
use crate::{error, file};

/// Image の一意な ID を発行するカウンタ
static NEXT_ID: AtomicU64 = AtomicU64::new(1);

/// ライブラリのエントリ
#[derive(Getters)]
pub(crate) struct LibraryEntry {
    id: u64,

    #[getset(get = "pub")]
    path: PathBuf,

    file_name: String,
}

/// public methods
impl LibraryEntry {
    pub fn new(path: PathBuf, file_name: String) -> Self {
        // ファイルの一意な ID を発行
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);

        Self { id, path, file_name }
    }
}

/// ライブラリの構造体
pub struct Library {
    entries: Vec<LibraryEntry>,
}

impl Library {
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
    pub fn add_entry(&mut self, path: PathBuf) -> error::Result<()> {
        // ベースディレクトリを取得
        let base_dir = path.parent().unwrap_or(&path).to_path_buf();

        // ファイルが画像ファイルの場合は
        // ベースディレクトリのファイル名をライブラリに追加
        if file::is_image(&path) {
            // ベースディレクトリがすでにライブラリに追加されている場合はスキップ
            let index = self.get_index_by_path(&base_dir);
            if index.is_some() { return Ok(()); }

            let file_name = self.get_file_name(&base_dir);
            self.entries.push(LibraryEntry::new(base_dir.clone(), file_name));

            // ファイルをソート
            self.sort();

            return Ok(());
        }

        // ファイルがすでにライブラリに追加されている場合はスキップ
        let index = self.get_index_by_path(&path);
        if index.is_some() { return Ok(()); }

        // ファイルを探索
        self.find_file(&path, &base_dir)?;

        // ファイルをソート
        self.sort();

        Ok(())
    }
}

/// private methods
impl Library {
    /// ファイルをソート
    fn sort(&mut self) {
        self.entries.sort_by(|a, b| a.path.cmp(&b.path));
    }

    /// ファイルを探索
    /// * `path` - ファイルのパス
    /// * `base_dir` - ベースディレクトリのパス
    /// * `return` - 結果
    fn find_file(&mut self, path: &PathBuf, base_dir: &PathBuf) -> error::Result<()> {
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

                // ライブラリに追加
                self.entries.push(
                    LibraryEntry::new(entry.path().clone(), file_name)
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

                // ライブラリに追加
                self.entries.push(
                    LibraryEntry::new(entry.path().clone(), file_name)
                );
            }

            // ディレクトリで何も追加されていない場合は、ディレクトリのパスをライブラリに追加
            if self.entries.is_empty() {
                self.entries.push(
                    LibraryEntry::new(path.clone(), self.get_file_name(&path))
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
