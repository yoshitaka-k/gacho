use crate::{error, event, file, model, ui};
use crate::event::{open, button};

/// 左矢印キーが押されたら次に移動する
/// * `ui` - ウィジェットのUI
/// * `files` - 開いているファイル
/// * `pending_actions` - 処理予約
pub fn arrow_left(
    ui: &mut egui::Ui,
    pending_actions: &mut Vec<event::EventAction>,
) {
    if ui.input(|input| input.key_pressed(egui::Key::ArrowLeft)) {
        pending_actions.push(event::EventAction::Left);
    }
}

/// 右矢印キーが押されたら前に移動する
/// * `ui` - ウィジェットのUI
/// * `files` - 開いているファイル
/// * `pending_actions` - 処理予約
pub fn arrow_right(
    ui: &mut egui::Ui,
    pending_actions: &mut Vec<event::EventAction>,
) {
    if ui.input(|input| input.key_pressed(egui::Key::ArrowRight)) {
        pending_actions.push(event::EventAction::Right);
    }
}

/// ドロップされたファイルを処理する
/// * `ui` - ウィジェットのUI
/// * `open_file` - 開いているファイル
/// * `return` - ドロップがなければ `None`、あれば処理結果
pub fn drop(
    ui: &egui::Ui,
    open_file: &mut file::OpenFile,
    book_model: &mut model::Book,
    is_open_last_page: bool,
) -> Option<error::Result<bool>> {
    ui.ctx().input(|input| {
        let files = input.raw.dropped_files.clone();
        if files.is_empty() {
            None
        } else {
            Some(open::drop(&files, open_file, book_model, is_open_last_page))
        }
    })
}

/// Command + O キーが押されたらファイルダイアログを開く
/// * `ui` - ウィジェットのUI
/// * `open_dialog_token` - ダイアログトークン
pub fn command_open(
    ui: &mut egui::Ui,
    open_dialog_token: &mut ui::OpenDialogToken,
) {
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
/// * `pending_actions` - 処理予約
pub fn command_comma(
    ui: &mut egui::Ui,
    pending_actions: &mut Vec<event::EventAction>,
) {
    if ui.input(|input| {
        input.modifiers.matches_exact(egui::Modifiers::COMMAND)
        && input.key_pressed(egui::Key::Comma)
    }) {
        let pos = ui.ctx().input(|input| {
            input.viewport().outer_rect.map(|rect| rect.min)
        });
        pending_actions.push(event::EventAction::SettingOpen(pos));
    }
}

/// Command + W キーイベント
///
/// メインウィンドウでは開いている本を閉じる
/// メインウィンドウ以外では設定ウィンドウを閉じる
///
/// * `ui` - ウィジェットのUI
/// * `pending_actions` - 処理予約
pub fn command_w(
    ui: &mut egui::Ui,
    pending_actions: &mut Vec<event::EventAction>,
) {
    if ui.input(|input| {
        input.modifiers.matches_exact(egui::Modifiers::COMMAND)
        && input.key_pressed(egui::Key::W)
    }) {
        if ui.ctx().viewport_id() == egui::ViewportId::ROOT {
            pending_actions.push(event::EventAction::Close);
        } else {
            pending_actions.push(event::EventAction::SettingClose);
        }
    }
}
