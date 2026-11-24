use crate::{state, lang, ui};
use crate::ui::modal::heading_icon;
use crate::ui::assets::{icon, svg};

/// エラーモーダルを表示
/// * `ctx` - コンテキスト
/// * `error_token` - エラーモーダルを表示するためのトークン
pub(crate) fn error(
    ctx: &egui::Context,
    error_token: &mut state::ErrorToken,
) {
    if error_token.value.is_none() { return; }

    // モーダルを表示
    let modal = egui::Modal::new(egui::Id::new("modal_error")).show(ctx, |ui| {
        ui.set_width(ui::MODAL_WINDOW_WIDTH);

        // 見出し部分
        ui.horizontal(|ui| {
            ui.add(heading_icon(svg::ERROR, icon::MODAL_ERROR_ICON_SIZE, egui::Color32::RED));
            ui.heading(lang::current().modal_error.heading);
        });

        ui.separator();
        ui.add_space(ui::MODAL_WINDOW_SPACING);

        // エラー内容を表示
        if let Some(error) = &error_token.value {
            egui::Frame::default().inner_margin(ui::PANEL_INNER_MARGIN).show(ui, |ui| {
                ui.label(error.to_string());

                // エラーコードを表示
                let code = error.code();
                ui.add_space(ui::MODAL_WINDOW_SPACING);
                ui.label(egui::RichText::new(format!("Code: {}", code)).weak());
            });
        }

        ui.add_space(ui::MODAL_WINDOW_SPACING);
    });

    // モーダルを閉じたらモーダルを非表示にする
    if modal.should_close() {
        error_token.dismiss();
    }
}
