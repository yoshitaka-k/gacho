use crate::{error};
use eframe::egui::DroppedFileHandle;

/// ドロップされたファイルを処理
/// * `files` - ドロップされたファイル
pub(crate) fn files(
    files: &[DroppedFileHandle],
) -> error::Result<()> {
    if files.is_empty() {
        return Ok(());
    }

    for file in files {
        println!("file: {:?}", file.path());
    }

    Ok(())
}
