use std::path::PathBuf;

use eframe::egui::DroppedFileHandle;

use crate::{file, error};

/// ドロップされたファイルを処理
/// * `files` - ドロップされたファイル
pub(crate) fn files(
    files: &[DroppedFileHandle],
    open_files: &mut file::OpenFiles,
) -> error::Result<()> {
    let Some(file) = files.first() else {
        return Ok(());
    };

    path(file.path().to_path_buf(), open_files)
}

/// パスからファイルを開く
/// * `path` - ファイルのパス
/// * `open_files` - 開いているファイル
pub(crate) fn path(
    path: PathBuf,
    open_files: &mut file::OpenFiles,
) -> error::Result<()> {
    open_files.clear();
    open_files.add_path(path)
}
