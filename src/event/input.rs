use crate::event;

/// ドロップされたファイルを処理する
/// * `ui` - ウィジェットのUI
/// * `pending_actions` - 処理予約
pub fn drop(
    ui: &egui::Ui,
    pending_actions: &mut Vec<event::EventAction>,
) {
    let files: Vec<std::path::PathBuf> = ui.ctx().input(|input| {
        input.raw.dropped_files.iter().map(|f| f.path().to_path_buf()).collect()
    });

    if files.is_empty() { return; }

    pending_actions.push(event::EventAction::DropOpen(files));
}

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

/// Command + O キーが押されたらファイルダイアログを開く
/// * `ui` - ウィジェットのUI
/// * `pending_actions` - 処理予約
pub fn command_open(
    ui: &mut egui::Ui,
    pending_actions: &mut Vec<event::EventAction>,
) {
    // Command + O キーが押されたらファイルダイアログを開く
    if ui.input(|input| {
        input.modifiers.matches_exact(egui::Modifiers::COMMAND)
        && input.key_pressed(egui::Key::O)
    }) {
        pending_actions.push(event::EventAction::FilesOpen);
    }

    // Command + Shift + O キーが押されたらフォルダダイアログを開く
    if ui.input(|input| {
        input.modifiers.matches_exact(egui::Modifiers::COMMAND | egui::Modifiers::SHIFT)
        && input.key_pressed(egui::Key::O)
    }) {
        pending_actions.push(event::EventAction::FolderOpen);
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
