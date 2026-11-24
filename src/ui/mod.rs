mod assets;
mod main;
mod setting;
mod modal;

/// メイン描画関数を公開
pub(crate) use main::view as draw;

use crate::ui::assets::{font, svg};

/// ダークモードパネルの背景色
const DARK_MODE_PANEL_COLOR: egui::Color32 = egui::Color32::from_rgb(35, 35, 35);
/// ライトモードパネルの背景色
const LIGHT_MODE_PANEL_COLOR: egui::Color32 = egui::Color32::from_rgb(225, 225, 225);

// モーダルウィンドウ

/// モーダルウィンドウの幅
const MODAL_WINDOW_WIDTH: f32 = 320.0;
/// モーダルウィンドウのスペーシング
const MODAL_WINDOW_SPACING: f32 = 8.0;
/// モーダルのラベルの幅
const MODAL_LABEL_WIDTH: f32 = 130.0;

// メインウィンドウ

/// スライダーの幅
const SLIDER_WIDTH: f32 = 50.0;

// 設定ウィンドウ

/// 設定ウィンドウのID
const SETTING_WINDOW_ID: &str = "setting_window";

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

/// パネルの内側のマージン
pub(crate) const PANEL_INNER_MARGIN: egui::Margin = egui::Margin {
    left: 8, right: 8, top: 0, bottom: 0,
};

/// トップパネルの内側のマージン
pub(crate) const TOP_PANEL_INNER_MARGIN: egui::Margin = egui::Margin {
    left: 10, right: 10, top: 4, bottom: 4,
};

pub(crate) const BOTTOM_PANEL_INNER_MARGIN: egui::Margin = egui::Margin {
    left: 10, right: 10, top: 4, bottom: 6,
};

/// フォントと SVG ローダーを追加
/// * `ctx` - コンテキスト
pub(crate) fn install(ctx: &egui::Context) {
    font::install(ctx);
    svg::install(ctx);
}

/// パネルの背景色
/// * `ui` - UI
/// * `return` - パネルの背景色
fn panel_fill_color(ui: &egui::Ui) -> egui::Color32 {
    if ui.ctx().global_style().visuals.dark_mode {
        DARK_MODE_PANEL_COLOR
    } else {
        LIGHT_MODE_PANEL_COLOR
    }
}

/// パネルのスタイルを設定
/// * `ui` - UI
/// * `inner_margin` - パネルの内側のマージン
/// * `return` - パネルのスタイル
pub(crate) fn panel_style(ui: &mut egui::Ui, inner_margin: egui::Margin) -> egui::Frame {
    let panel_fill_color = panel_fill_color(ui);
    egui::Frame::default()
        .fill(panel_fill_color)
        .inner_margin(inner_margin)
}

/// ラベルの幅を取得
/// * `ui` - UI
/// * `label` - ラベルのテキスト
/// * `width` - ラベルの幅
/// * `return` - ラベルのレスポンス
pub(crate) fn add_label(ui: &mut egui::Ui, label: &str, width: f32) -> egui::Response {
    ui.allocate_ui_with_layout(
        egui::vec2(width, ui.spacing().interact_size.y),
        egui::Layout::right_to_left(egui::Align::Center),
        |ui| {
            ui.set_min_width(width);
            ui.label(label)
        }
    ).inner
}
