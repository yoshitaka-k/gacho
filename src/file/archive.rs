use std::path::Path;
use std::fs::File;
use std::io::{Cursor, Read, BufReader};
use getset::{Getters, Setters};

use crate::file;

#[derive(Getters, Setters)]
pub(crate) struct ArchiveFile {
    /// アーカイブ内のファイルのインデックス
    #[getset(get = "pub")]
    index: usize,

    /// ファイル名
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

            // zipファイル内の相対パス付きファイル名を取得する
            let relative_path = self.decode_raw(file.name_raw());

            // ファイルが隠しファイルかどうかをチェックする
            let path = Path::new(&relative_path);
            if file::is_hidden_entry(&path) {
                continue;
            }

            // zipファイル内のファイル名を取得する
            let file_name = path.file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_else(|| relative_path.clone());

            self.files.push(ArchiveFile {
                index: i,
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
    pub fn get_zipfile(&mut self, index: usize) -> Result<ArchiveFile, Box<dyn std::error::Error>> {
        // アーカイブのファイルからファイル名と相対パスを取得する
        let (file_name, relative_path) = self.files.iter()
            .find(|f| f.index == index)
            .map(|f| (f.file_name.clone(), f.relative_path.clone()))
            .ok_or("file not found")?;

        // アーカイブファイルを取得する
        let archive = self.archive.as_mut().ok_or("archive not loaded")?;
        let mut file = archive.by_index(index)?;

        if file.is_dir() {
            return Err("file is a directory".into());
        }

        // アーカイブからファイルのバイト列を取得する
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;

        Ok(ArchiveFile {
            index,
            file_name,
            relative_path,
            bytes,
        })
    }

    /// ファイルを名前でソートする
    fn sort(&mut self) {
        self.files.sort_by_key(|file| file.relative_path.clone());
    }

    /// テキストをデコードする
    /// * `raw` - デコードするテキストのバイト列
    /// * `return` - デコードしたテキスト
    fn decode_raw(&self, raw: &[u8]) -> String {
        // utf-8 でデコードできる場合は utf-8 でデコード
        if let Ok(name) = str::from_utf8(raw) {
            return name.to_string();
        }

        // utf-8 でデコードできない場合は Shift-JIS でデコード
        let (decoded, _, errors) = encoding_rs::SHIFT_JIS.decode(raw);
        if !errors {
            return decoded.to_string();
        }

        // utf-8 でも、Shift-JIS でもデコードできない場合はバイト列をそのまま返す
        String::from_utf8_lossy(raw).to_string()
    }
}
