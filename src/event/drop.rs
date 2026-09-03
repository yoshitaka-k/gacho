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

    // ファイルをクリア
    open_files.clear();

    // ドロップされたファイルを追加
    // 最初のファイルのみを追加
    let path = files[0].path().to_path_buf();
    open_files.add_path(path)?;

    Ok(())
}
