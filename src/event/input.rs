use crate::{file, event, error, ui};
use crate::event::{drop, button};

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

/// Command + O キーが押されたらファイルダイアログを開く
/// * `ui` - ウィジェットのUI
/// * `open_dialog_token` - ダイアログトークン
pub fn command_open(ui: &mut egui::Ui, open_dialog_token: &mut ui::OpenDialogToken) {
    // Command + O キーが押されたらファイルダイアログを開く
    if ui.input(|input| {
        input.modifiers.matches_exact(egui::Modifiers::COMMAND)
        && input.key_pressed(egui::Key::O)
    }) {
        button::files_open(ui, open_dialog_token);
    }

    // Command + Shift + O キーが押されたらフォルダダイアログを開く
    if ui.input(|input| {
        input.modifiers.matches_exact(egui::Modifiers::COMMAND | egui::Modifiers::SHIFT)
        && input.key_pressed(egui::Key::O)
    }) {
        button::folder_open(ui, open_dialog_token);
    }
}

/// Command + Comma キーが押されたら設定ウィンドウを開く
/// * `ui` - ウィジェットのUI
/// * `setting_token` - 設定ウィンドウトークン
pub fn command_comma(ui: &mut egui::Ui, setting_token: &mut ui::SettingToken) {
    if ui.input(|input| {
        input.modifiers.matches_exact(egui::Modifiers::COMMAND)
        && input.key_pressed(egui::Key::Comma)
    }) {
        button::setting_open(ui, setting_token);
    }
}
