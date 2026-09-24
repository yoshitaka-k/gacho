use std::path::PathBuf;

use eframe::egui::DroppedFileHandle;

use crate::{file, error};

/// ドロップされたファイルを処理
/// * `files` - ドロップされたファイル
pub(crate) fn files(
    files: &[DroppedFileHandle],
    open_file: &mut file::OpenFile,
) -> error::Result<bool> {
    let Some(file) = files.first() else {
        return Ok(false);
    };

    path(file.path().to_path_buf(), open_file)
}

/// パスからファイルを開く
/// * `path` - ファイルのパス
/// * `open_file` - 開いているファイル
pub(crate) fn path(
    path: PathBuf,
    open_file: &mut file::OpenFile,
) -> error::Result<bool> {
    // ファイルをクリア
    open_file.clear();

    // ファイルを追加
    open_file.open_book(path)
}
