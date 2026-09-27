use std::path::PathBuf;

use eframe::egui::DroppedFileHandle;

use crate::{file, error, model};

/// ドロップされたファイルを処理
/// * `files` - ドロップされたファイル
pub(crate) fn files(
    files: &[DroppedFileHandle],
    open_file: &mut file::OpenFile,
    book_model: &mut model::Book,
    is_open_last_page: bool,
) -> error::Result<bool> {
    let Some(file) = files.first() else {
        return Ok(false);
    };

    path(file.path().to_path_buf(), open_file, book_model, is_open_last_page)
}

/// パスからファイルを開く
/// * `path` - ファイルのパス
/// * `open_file` - 開いているファイル
pub(crate) fn path(
    path: PathBuf,
    open_file: &mut file::OpenFile,
    book_model: &mut model::Book,
    is_open_last_page: bool,
) -> error::Result<bool> {
    let books = if is_open_last_page {
        book_model.get_all()?
    } else {
        Vec::new()
    };

    // ファイルを追加
    open_file.build_book(path, books)
}
