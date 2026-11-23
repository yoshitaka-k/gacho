use crate::ui;
use crate::ui::assets;
use crate::ui::assets::{icon, svg};

/// 設定ウィンドウの上部パネルを描画
/// * `ui` - UI
/// * `setting_token` - 設定トークン
pub(crate) fn view(
    ui: &mut egui::Ui,
    setting_token: &mut ui::SettingToken,
) {
    // パネルのスタイルを設定
    let style = ui::panel_style(ui, ui::TOP_PANEL_INNER_MARGIN);
    // アイコンの色を取得
    let icon_color = assets::icon_color(ui);

    // タブを表示
    egui::Panel::top("setting_top_taskbar").frame(style).show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.add(egui::Image::new(svg::SETTINGS).max_height(icon::TOP_MENU_SETTINGS_ICON_SIZE).tint(icon_color));

            // タブの選択時の背景色を保存
            let selection_bg_fill = ui.style_mut().visuals.selection.bg_fill;

            // タブを表示
            ui.selectable_value(&mut setting_token.tab, ui::SettingTab::General, ui::SettingTab::General.to_string());
            ui.selectable_value(&mut setting_token.tab, ui::SettingTab::Layout, ui::SettingTab::Layout.to_string());
            ui.selectable_value(&mut setting_token.tab, ui::SettingTab::Memory, ui::SettingTab::Memory.to_string());
            ui.selectable_value(&mut setting_token.tab, ui::SettingTab::About, ui::SettingTab::About.to_string());

            // タブの選択時の背景色をリセット
            ui.style_mut().visuals.selection.bg_fill = selection_bg_fill;
        });
    });
}
