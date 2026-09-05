use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{Cursor, Read, BufReader};
use getset::{Getters, Setters};

use crate::file;

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

    /// アーカイブを展開する
    /// * `path` - アーカイブのパス
    /// * `return` - アーカイブを展開した結果
    pub fn unarchive(&mut self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();

        reader.read_to_end(&mut buffer)?;
        let mut archive = zip::ZipArchive::new(Cursor::new(buffer))?;

        // アーカイブのファイル数を取得する
        self.len = archive.len();

        // アーカイブのファイルを取得する
        // bytes 以外のデータを取得する（bytes はファイルを指定した時に取得する）
        for i in 0..self.len {
            let file = archive.by_index(i)?;
            if file.is_dir() {
                continue;
            }

            // zipファイル内のパスを取得する
            let path = file.enclosed_name().unwrap_or(PathBuf::new());

            // ファイルが隠しファイルかどうかをチェックする
            if file::is_hidden_entry(&path) {
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

            self.files.push(ArchiveFile {
                file_name,
                relative_path,
                bytes: Vec::new(),
            });
        }

        // ファイルを名前でソートする
        self.sort();

        // アーカイブを保存する
        self.archive = Some(archive);

        Ok(())
    }

    /// アーカイブのファイルを取得する
    /// * `relative_path` - アーカイブ内のファイルの相対パス
    /// * `return` - アーカイブのファイル
    pub fn get_zipfile(&mut self, relative_path: &str) -> Result<ArchiveFile, Box<dyn std::error::Error>> {
        let archive = self.archive.as_mut().ok_or("archive not loaded")?;
        let mut file = archive.by_name(relative_path)?;

        if file.is_dir() {
            return Err("file is a directory".into());
        }

        let path = file.enclosed_name().unwrap_or_default();
        let file_name = path.file_name()
            .ok_or("file name not found")?
            .to_string_lossy()
            .into_owned();
        let relative_path = file.name().to_string();

        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;

        Ok(ArchiveFile {
            file_name,
            relative_path,
            bytes,
        })
    }

    /// ファイルを名前でソートする
    fn sort(&mut self) {
        self.files.sort_by_key(|file| file.relative_path.clone());
    }
}
