use crate::{file, ui, update};

/// 設定ダイアログを開く
/// * `setting_token` - 設定ダイアログを開くためのトークン
/// * `pos` - 設定ダイアログの表示位置
pub(crate) fn setting_open(
    setting_token: &mut ui::SettingToken,
    pos: Option<egui::Pos2>,
) {
    // 設定ダイアログのタブを設定
    setting_token.tab = ui::SettingTab::General;

    // 設定ダイアログの表示位置を設定
    setting_token.pos = pos;

    // 設定ダイアログを開く
    setting_token.open = true;
}

/// アップデートを確認する
/// * `update_job` - 更新ジョブ
/// * `update_token` - 更新モーダルを表示するためのトークン
pub(crate) fn check_for_update(update_job: &mut update::UpdateJob, update_token: &mut update::UpdateToken) {
    update_job.run(update_token);
}

/// 開いている本を閉じる
/// * `open_file` - 開いている本
pub(crate) fn close_open_file(open_file: &mut file::OpenFile) {
    open_file.clear();
}
