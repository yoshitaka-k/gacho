use crate::{file, error};
use eframe::egui::DroppedFileHandle;

/// ドロップされたファイルを処理
/// * `files` - ドロップされたファイル
pub(crate) fn files(
    files: &[DroppedFileHandle],
    open_files: &mut file::OpenFiles,
) -> error::Result<()> {
    if files.is_empty() {
        return Ok(());
    }

    // ドロップされたファイルを追加
    for file in files {
        open_files.add_path(file.path().to_path_buf())?;
    }

    Ok(())
}
