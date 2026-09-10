pub(crate) mod view;
mod general;
mod about;

use crate::app;
use crate::event::button;
use crate::ui::assets::{self, icon};

// ウィンドウのID
pub(crate) const SETTING_WINDOW_ID: &str = "setting_window";

/// ウィンドウのタイトル
const WINDOW_TITLE: &str = "Settings";

// ウィンドウのサイズ
const WINDOW_WIDTH: f32 = 460.0;
const WINDOW_HEIGHT: f32 = 240.0;

// ヘッダーのスペースの幅
const HEADER_ICON_SPACING: f32 = 4.0;
const HEADER_BOTTOM_SPACING: f32 = 2.0;

// ラベルの幅
pub(crate) const GENERAL_LABEL_WIDTH: f32 = 76.0;

// 追加のスペースの幅
pub(crate) const SETTING_ADD_SPACING: f32 = 4.0;

/// ヘッダーパネルを表示
/// * `ui` - UI
/// * `icon` - アイコン
pub(crate) fn header_panel(
    ui: &mut egui::Ui,
    icon: egui::ImageSource<'static>,
    label: &str,
    update_job: Option<&mut app::UpdateJob>,
) {
    let spacing = ui.spacing().item_spacing.x;

    // ボタンの色を設定
    let icon_color = assets::icon_color(ui);

    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = HEADER_ICON_SPACING;
        ui.add(egui::Image::new(icon).max_height(icon::SETTINGS_ICON_SIZE).tint(icon_color));
        ui.spacing_mut().item_spacing.x = spacing;
        ui.label(label);

        // アップデート確認ボタン
        if let Some(update_job) = update_job {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("Check for updates.").clicked() {
                    button::check_for_update(update_job);
                }
            });
        }
    });
}

/// ラベル後の残り幅に合わせてスライダーのレール幅を決める
/// * `ui` - UI
/// * `return` - スライダーのレール幅
pub(crate) fn remaining_slider_width(ui: &egui::Ui) -> f32 {
    let spacing = ui.spacing();
    (ui.available_width() - spacing.item_spacing.x - spacing.interact_size.x).max(0.0)
}
