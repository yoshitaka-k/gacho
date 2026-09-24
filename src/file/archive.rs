use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{Cursor, Read, BufReader};
use getset::{Getters, Setters};
use image::ImageReader;

use crate::file;

// アーカイブヘッダーのバイト数
// JPEG の巨大 EXIF 用に大きめに設定
const HEADER_BYTES: u64 = 256 * 1024;

#[derive(Clone, Getters, Setters)]
#[getset(get = "pub")]
pub(crate) struct ArchiveFile {
    /// アーカイブ内のファイルのインデックス
    index: usize,

    /// ファイル名
    file_name: String,

    /// ファイルのパス
    relative_path: PathBuf,

    /// ファイルの幅・高さ
    #[getset(set = "pub")]
    size: egui::Vec2,

    /// ファイルのバイト列
    #[getset(set = "pub")]
    bytes: Vec<u8>,
}

#[derive(Getters, Setters)]
#[getset(get = "pub")]
pub(crate) struct Archive {
    /// アーカイブ
    archive: Option<zip::ZipArchive<Cursor<Vec<u8>>>>,

    /// アーカイブ内のファイル
    files: Vec<ArchiveFile>,

    /// アーカイブ内のファイル数
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
    /// アーカイブ内のファイルのbyte情報は展開した後に取得するため、ここでは取得しない
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
            let mut file = archive.by_index(i)?;
            if file.is_dir() { continue; }

            // zipファイル内の相対パス付きファイル名を取得する
            let relative_path = PathBuf::from(self.decode_raw(file.name_raw()));
            let path = relative_path.clone();

            // ファイルが隠しファイルかどうかをチェックする
            if file::is_hidden_entry(&path) { continue; }
            // ファイルが画像でない場合はスキップする
            if !file::is_image(&path.to_path_buf()) { continue; }

            // zipファイル内のファイル名を取得する
            let file_name = path.file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_else(|| {
                    relative_path.to_string_lossy().into_owned()
                });

            // アーカイブ内のファイルのヘッダーの HEADER_BYTES 数を取得する
            let mut header = Vec::new();
            (&mut file).take(HEADER_BYTES).read_to_end(&mut header)?;

            // ファイルのヘッダーのバイト列から画像の幅・高さを取得する
            let size = match ImageReader::new(Cursor::new(header)).with_guessed_format() {
                Ok(reader) => match reader.into_dimensions() {
                    Ok((width, height)) => egui::Vec2::new(width as f32, height as f32),
                    Err(_) => egui::Vec2::ZERO,
                },
                Err(_) => egui::Vec2::ZERO,
            };

            self.files.push(ArchiveFile {
                index: i,
                file_name,
                relative_path,
                size,
                bytes: Vec::new(),
            });
        }

        // ファイルを名前でソートする
        self.sort();

        // アーカイブを保存する
        self.archive = Some(archive);

        Ok(())
    }

    /// アーカイブファイルを取得する
    /// * `index` - アーカイブ内のファイルのインデックス
    /// * `return` - ArchiveFile のインスタンス
    pub fn get_zipfile(&mut self, index: usize) -> Result<&ArchiveFile, Box<dyn std::error::Error>> {
        // アーカイブファイルを取得する
        let archive_file = self.files.iter_mut()
            .find(|f| f.index == index)
            .ok_or("file not found")?;

        if !archive_file.bytes.is_empty() {
            return Ok(archive_file);
        }

        // アーカイブを取得する
        let archive = self.archive.as_mut().ok_or("archive not loaded")?;
        let mut file = archive.by_index(index)?;

        if file.is_dir() {
            return Err("file is a directory".into());
        }

        // アーカイブからファイルのバイト列を取得する
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;
        archive_file.set_bytes(bytes.clone());

        Ok(archive_file)
    }

    /// ファイルを名前でソートする
    fn sort(&mut self) {
        let mut temp_paths = Vec::new();
        for file in &self.files {
            temp_paths.push(file.relative_path.clone());
        }

        let sorted_paths = file::sort_path(&temp_paths);

        let mut sorted_files: Vec<ArchiveFile> = Vec::new();
        for path in &sorted_paths {
            let file = self.files.iter().find(|f| f.relative_path == *path).unwrap();
            sorted_files.push(file.clone());
        }

        self.files = sorted_files;
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
