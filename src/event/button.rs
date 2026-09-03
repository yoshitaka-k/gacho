use crate::{app, file, ui};

/// ファイルダイアログを開く
/// * `ui` - UI
/// * `open_dialog_token` - ファイルダイアログを開くためのトークン
pub(crate) fn files_open(ui: &mut egui::Ui, open_dialog_token: &mut ui::OpenDialogToken) {
    // ファイルダイアログを開くタイミングをずらす
    open_dialog_token.file_dialog = true;

    // 再描画を要求
    ui.ctx().request_repaint();
}

/// フォルダダイアログを開く
/// * `ui` - UI
/// * `open_dialog_token` - フォルダダイアログを開くためのトークン
pub(crate) fn folder_open(ui: &mut egui::Ui, open_dialog_token: &mut ui::OpenDialogToken) {
    // フォルダダイアログを開くタイミングをずらす
    open_dialog_token.folder_dialog = true;

    // 再描画を要求
    ui.ctx().request_repaint();
}

/// 設定ダイアログを開く
/// * `ui` - UI
/// * `setting_token` - 設定ダイアログを開くためのトークン
pub(crate) fn setting_open(ui: &mut egui::Ui, setting_token: &mut ui::SettingToken) {
    // 設定ダイアログを開く
    setting_token.open = true;

    // 設定ダイアログの表示位置を設定
    setting_token.pos = ui.ctx().input(|input| {
        input.viewport().outer_rect.map(|rect| rect.min)
    });

    // 再描画を要求
    ui.ctx().request_repaint();
}

/// アップデートを確認する
/// * `update_job` - 更新ジョブ
/// * `updated_token` - 更新モーダルを表示するためのトークン
pub(crate) fn check_for_update(update_job: &mut app::UpdateJob) {
    update_job.run();
}

/// 次のファイルを表示する
/// * `ui` - UI
/// * `open_files` - 開いているファイル
pub(crate) fn next(ui: &mut egui::Ui, open_files: &mut file::OpenFiles) {
    open_files.next();

    // 再描画を要求
    ui.ctx().request_repaint();
}

/// 前のファイルを表示する
/// * `ui` - UI
/// * `open_files` - 開いているファイル
pub(crate) fn prev(ui: &mut egui::Ui, open_files: &mut file::OpenFiles) {
    open_files.prev();

    // 再描画を要求
    ui.ctx().request_repaint();
}
