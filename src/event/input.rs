use crate::error;
use crate::event::drop;

/// ドロップされたファイルを処理する
/// * `ui` - ウィジェットのUI
/// * `app` - アプリケーション
/// * `open_files` - 開いているファイル
/// * return: エラーが発生した場合はエラーを返す
pub fn drop(ui: &egui::Ui) -> error::Result<()> {
    ui.ctx().input(|input| {
        let files = input.raw.dropped_files.clone();
        drop::files(
            &files,
        )
    })?;

    Ok(())
}
