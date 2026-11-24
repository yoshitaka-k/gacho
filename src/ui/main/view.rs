use crate::{state, event};
use crate::ui::{setting, modal};
use crate::ui::main::{top, bottom, middle};


/// メインウィンドウを表示
/// * `ui` - UI
/// * `state` - 状態
/// * `pending_actions` - 待機中のアクション
pub(crate) fn view(
    ui: &mut egui::Ui,
    state: &mut state::State,
    pending_actions: &mut Vec<event::EventAction>,
) {
    // 上部パネルを表示
    top::view(
        ui,
        &mut state.files,
        pending_actions,
    );

    // 下部パネルを表示
    bottom::view(
        ui,
        &mut state.config,
        &mut state.files,
        pending_actions,
    );

    // 中央パネルを表示
    middle::view(
        ui,
        &mut state.config,
        &mut state.files,
        pending_actions,
        &mut state.error_token,
    );

    // 設定ウィンドウを表示
    if state.setting_token.open {
        setting::view(
            ui.ctx(),
            state,
            pending_actions,
        );
    }

    // エラーモーダルを表示
    if state.error_token.open {
        modal::error(
            ui.ctx(),
            &mut state.error_token,
        );
    }
}
