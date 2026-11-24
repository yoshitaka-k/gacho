mod view;
mod top;
mod general;
mod layout;
mod memory;
mod about;

pub(crate) use view::view;

use crate::{lang, event, ui, update};
use crate::ui::assets::{self, icon, svg};

/// ヘッダーパネルを表示
/// * `ui` - UI
/// * `icon` - アイコン
/// * `label_width` - ラベルの幅
pub(crate) fn header_panel(
    ui: &mut egui::Ui,
    icon: egui::ImageSource<'static>,
    label: &str,
) {
    let spacing = ui.spacing().item_spacing.x;
    let icon_color = assets::icon_color(ui);

    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = ui::HEADER_ICON_SPACING;
        ui.add(egui::Image::new(icon).max_height(icon::SETTINGS_ICON_SIZE).tint(icon_color));
        ui.spacing_mut().item_spacing.x = spacing;
        ui.label(label);
    });
}

/// ヘッダーパネルを表示
/// * `ui` - UI
/// * `icon` - アイコン
pub(crate) fn about_header_panel(
    ui: &mut egui::Ui,
    icon: egui::ImageSource<'static>,
    label: &str,
    update_token: &update::UpdateToken,
    pending_actions: &mut Vec<event::EventAction>,
) {
    ui.horizontal(|ui| {
        header_panel(ui, icon, label);

        // アップデート確認ボタン
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.add_enabled_ui(!update_token.running, |ui| {
                if ui.button(lang::current().setting_about.check_for_updates).clicked() {
                    pending_actions.push(event::EventAction::CheckForUpdate);
                }
            });
        });
    });
}

/// ラベル後の残り幅に合わせてスライダーのレール幅を決める
/// * `ui` - UI
/// * `return` - スライダーのレール幅
pub(crate) fn remaining_slider_width(ui: &egui::Ui) -> f32 {
    let spacing = ui.spacing();
    (ui.available_width() - spacing.item_spacing.x - spacing.interact_size.x).max(0.0)
}

/// 警告ノートを表示
/// * `ui` - UI
/// * `text` - 警告ノートのテキスト
/// * `return` - 警告ノートのリッチテキスト
pub(crate) fn warning_note(ui: &mut egui::Ui, text: &str) {
    // デフォルトのスペースの幅を避けておく
    let spacing = ui.spacing().item_spacing.x;

    // 出力パスの注意書きを表示
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = ui::WARNING_ICON_SPACING;
        ui.add(egui::Image::new(svg::WARNING).max_height(icon::WARNING_ICON_SIZE).tint(assets::warning_color(ui)));
        ui.spacing_mut().item_spacing.x = spacing;
        ui.add(egui::Label::new(
            egui::RichText::new(text).weak(),
        ));
    });
}
