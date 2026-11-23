mod view;
mod top;
mod general;
mod layout;
mod memory;
mod about;

pub(crate) use view::view;

use crate::{lang, update};
use crate::event::button;
use crate::ui::assets::{self, icon, svg};

/// 設定ウィンドウのID
pub(crate) const SETTING_WINDOW_ID: &str = "setting_window";

/// 設定ウィンドウの幅
const WINDOW_WIDTH: f32 = 420.0;
/// 設定ウィンドウの高さ
const WINDOW_HEIGHT: f32 = 240.0;

/// 追加のスペースの幅
const SETTING_ADD_SPACING: f32 = 4.0;

/// ヘッダーのアイコンのスペースの幅
const HEADER_ICON_SPACING: f32 = 4.0;
/// ヘッダーの下のスペースの幅
const HEADER_BOTTOM_SPACING: f32 = 2.0;
/// 警告ノートのアイコンのスペースの幅
const WARNING_ICON_SPACING: f32 = 3.0;

// レイアウトのラベルの幅

/// 読み込み方向の英語のラベルの幅
const READFROM_EN_LABEL_WIDTH: f32 = 80.0;
/// 読み込み方向の日本語のラベルの幅
const READFROM_JA_LABEL_WIDTH: f32 = 60.0;

/// レイアウトの英語のラベルの幅
const LAYOUT_EN_LABEL_WIDTH: f32 = 80.0;
/// レイアウトの日本語のラベルの幅
const LAYOUT_JA_LABEL_WIDTH: f32 = 110.0;

// メモリのラベルの幅

/// メモリの英語のラベルの幅
const MEMORY_EN_LABEL_WIDTH: f32 = 140.0;
/// メモリの日本語のラベルの幅
const MEMORY_JA_LABEL_WIDTH: f32 = 110.0;

// 前処理数のラベルの幅

/// 前処理数の英語のラベルの幅
const PRELOADING_EN_LABEL_WIDTH: f32 = 70.0;
/// 前処理数の日本語のラベルの幅
const PRELOADING_JA_LABEL_WIDTH: f32 = 110.0;

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
        ui.spacing_mut().item_spacing.x = HEADER_ICON_SPACING;
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
    update_job: &mut update::UpdateJob,
    update_token: &mut update::UpdateToken,
) {
    ui.horizontal(|ui| {
        header_panel(ui, icon, label);

        // アップデート確認ボタン
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.add_enabled_ui(!update_token.running, |ui| {
                if ui.button(lang::current().setting_about.check_for_updates).clicked() {
                    button::check_for_update(update_job, update_token);
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
        ui.spacing_mut().item_spacing.x = WARNING_ICON_SPACING;
        ui.add(egui::Image::new(svg::WARNING).max_height(icon::WARNING_ICON_SIZE).tint(assets::warning_color(ui)));
        ui.spacing_mut().item_spacing.x = spacing;
        ui.add(egui::Label::new(
            egui::RichText::new(text).weak(),
        ));
    });
}
