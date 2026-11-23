use crate::{config, lang, event, ui, update};
use crate::event::input;
use crate::ui::{setting, modal};
use crate::ui::setting::{top, general, layout, memory, about};

/// 設定ウィンドウを表示
/// * `ctx` - コンテキスト
/// * `config` - 設定
/// * `setting_token` - 設定トークン
/// * `update_token` - アップデートトークン
pub(crate) fn view(
    ctx: &egui::Context,
    config: &mut config::Config,
    setting_token: &mut ui::SettingToken,
    update_token: &mut update::UpdateToken,
    mut update_job: &mut update::UpdateJob,
    pending_actions: &mut Vec<event::EventAction>,
) {
    let window_id = egui::ViewportId::from_hash_of(setting::SETTING_WINDOW_ID);

    // 設定ウィンドウのオプションを設定
    let mut options = egui::ViewportBuilder::default()
        .with_title(lang::current().setting.heading)
        .with_inner_size([setting::WINDOW_WIDTH, setting::WINDOW_HEIGHT])
        .with_maximize_button(false)
        .with_resizable(false);

    // ウィンドウの表示位置を指定
    // take()で、取り出して None にする（ボタン押下時だけ位置更新と前面化）
    if let Some(pos) = setting_token.pos.take() {
        // ウィンドウの位置を指定
        options = options.with_position(pos);

        // 表示していたら、ウィンドウの位置を更新して前面に出す
        ctx.send_viewport_cmd_to(window_id, egui::ViewportCommand::OuterPosition(pos));
        ctx.send_viewport_cmd_to(window_id, egui::ViewportCommand::Focus);
    }

    // 設定ウィンドウを表示
    ctx.show_viewport_immediate(window_id, options, |ui, _class| {
        // 設定ウィンドウの上部パネルを描画
        top::view(ui, setting_token);

        // 設定ウィンドウの中央パネルを描画
        egui::CentralPanel::default().show(ui, |ui| {
            // タブに応じて表示内容を切り替え
            match setting_token.tab {
                ui::SettingTab::General => general::view(ui, config),
                ui::SettingTab::Layout => layout::view(ui, config),
                ui::SettingTab::Memory => memory::view(ui, config),
                ui::SettingTab::About => about::view(ui, &mut update_job, update_token),
            }
        });

        // Command + W キーが押されたら閉じる
        input::command_w(ui, pending_actions);

        // ウィンドウの閉じるボタンが押されたら閉じる
        if ui.input(|input| input.viewport().close_requested()) {
            pending_actions.push(event::EventAction::SettingClose);
        }

        // 更新モーダルを表示
        if update_token.open {
            modal::updated(ui, update_token);
        }
    });
}
