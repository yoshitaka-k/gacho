use crate::{state};

/// 設定ダイアログを開く
/// * `setting_token` - 設定ダイアログを開くためのトークン
/// * `pos` - 設定ダイアログの表示位置
pub(crate) fn setting_open(
    setting_token: &mut state::SettingToken,
    pos: Option<egui::Pos2>,
) {
    // 設定ダイアログのタブを設定
    setting_token.tab = state::SettingTab::General;

    // 設定ダイアログの表示位置を設定
    setting_token.pos = pos;

    // 設定ダイアログを開く
    setting_token.open = true;
}
