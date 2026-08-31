use crate::{file, event, error};
use crate::event::drop;

/// 左矢印キーが押されたら次に移動する
/// * `ui` - ウィジェットのUI
/// * `files` - 開いているファイル
/// * `pending_actions` - 処理予約
pub fn arrow_left(ui: &mut egui::Ui, pending_actions: &mut Vec<event::EventAction>) {
    if ui.input(|input| input.key_pressed(egui::Key::ArrowLeft)) {
        pending_actions.push(event::EventAction::Left);
    }
}

/// 右矢印キーが押されたら前に移動する
/// * `ui` - ウィジェットのUI
/// * `files` - 開いているファイル
/// * `pending_actions` - 処理予約
pub fn arrow_right(ui: &mut egui::Ui, pending_actions: &mut Vec<event::EventAction>) {
    if ui.input(|input| input.key_pressed(egui::Key::ArrowRight)) {
        pending_actions.push(event::EventAction::Right);
    }
}

/// ドロップされたファイルを処理する
/// * `ui` - ウィジェットのUI
/// * `app` - アプリケーション
/// * `open_files` - 開いているファイル
/// * return: エラーが発生した場合はエラーを返す
pub fn drop(ui: &egui::Ui, open_files: &mut file::OpenFiles) -> error::Result<()> {
    ui.ctx().input(|input| {
        let files = input.raw.dropped_files.clone();
        drop::files(
            &files,
            open_files,
        )
    })?;

    Ok(())
}
