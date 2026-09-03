use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{Cursor, Read, BufReader};
use getset::{Getters, Setters};

#[derive(Getters, Setters)]
pub(crate) struct ArchiveFile {
    #[getset(get = "pub")]
    file_name: String,

    /// ファイルのパス
    #[getset(get = "pub")]
    relative_path: String,

    /// ファイルのバイト列
    #[getset(get = "pub")]
    bytes: Vec<u8>,
}

#[derive(Getters, Setters)]
pub(crate) struct Archive {
    #[getset(get = "pub")]
    archive: Option<zip::ZipArchive<Cursor<Vec<u8>>>>,

    #[getset(get = "pub")]
    files: Vec<ArchiveFile>,

    #[getset(get = "pub")]
    len: usize,
}

impl Archive {
    pub(crate) fn new() -> Self {
        Self {
            archive: None,
            files: Vec::new(),
            len: 0,
        }
    }

    /// ファイルを名前でソートする
    pub fn sort(&mut self) {
        self.files.sort_by_key(|file| file.relative_path.clone());
    }

    /// アーカイブを展開する
    /// * `path` - アーカイブのパス
    /// * `return` - アーカイブを展開した結果
    pub fn unarchive(&mut self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();

        reader.read_to_end(&mut buffer)?;
        let mut archive = zip::ZipArchive::new(Cursor::new(buffer))?;

        // アーカイブを保存する
        self.archive = Some(archive.clone());

        // アーカイブのファイル数を取得する
        self.len = archive.len();

        // アーカイブのファイルを取得する
        for i in 0..self.len {
            let mut file = archive.by_index(i)?;
            if file.is_dir() {
                continue;
            }

            // zipファイル内のパスを取得する
            let path = file.enclosed_name().unwrap_or(PathBuf::new());

            // ファイルが隠しファイルかどうかをチェックする
            if self.is_hidden_zip_entry(&path) {
                continue;
            }

            // zipファイル内のファイル名を取得する
            let file_name = if let Some(file_name) = path.file_name() {
                file_name.to_string_lossy().to_string()
            } else {
                continue;
            };

            // zipファイル内の相対パス付きファイル名を取得する
            let relative_path = file.name().to_string();

            let mut bytes = Vec::new();
            file.read_to_end(&mut bytes)?;

            self.files.push(ArchiveFile {
                file_name,
                relative_path,
                bytes,
            });
        }

        // ファイルを名前でソートする
        self.sort();

        Ok(())
    }

    /// ファイルが隠しファイルかどうかをチェックする
    /// * `path` - ファイルのパス
    /// * `return` - ファイルが隠しファイルかどうか
    fn is_hidden_zip_entry(&self, path: &Path) -> bool {
        path.components().any(|c| {
            let s = c.as_os_str().to_string_lossy();
            s.starts_with('.') || s == "__MACOSX"
        })
    }
}
