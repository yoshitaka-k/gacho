use crate::event::button;
use crate::ui;
use crate::ui::assets::{self, icon, svg};

/// 上部パネル
pub(crate) fn view(
    ui: &mut egui::Ui,
    setting_token: &mut ui::SettingToken,
    open_dialog_token: &mut ui::OpenDialogToken,
) {
    // ボタンの色を設定
    let button_color = assets::button_icon_color(ui);

    // 上部パネルのスタイルを設定
    let top_panel_style = ui::panel_style(ui, ui::TOP_PANEL_INNER_MARGIN);

    // 上部パネルを表示
    egui::Panel::top("top_taskbar").frame(top_panel_style).show(ui, |ui| {
        // 開くボタンと設定ボタンを右寄せに配置
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            // 設定ボタン
            let settings_button = egui::Image::new(svg::SETTINGS).max_height(icon::BUTTON_SETTINGS_ICON_SIZE).tint(button_color);
            if ui.button(settings_button).on_hover_text("Settings").clicked() {
                button::setting_open(ui, setting_token);
            }

            // フォルダダイアログを開くボタン
            let hover_text = "Folder Open";
            let open_button = egui::Image::new(svg::FOLDER_OPEN).max_height(icon::BUTTON_OPEN_ICON_SIZE).tint(button_color);
            if ui.button(open_button).on_hover_text(hover_text).clicked() {
                button::folder_open(ui, open_dialog_token);
            }

            let hover_text = "Files Open";
            let open_button = egui::Image::new(svg::FILE_OPEN).max_height(icon::BUTTON_OPEN_ICON_SIZE).tint(button_color);
            if ui.button(open_button).on_hover_text(hover_text).clicked() {
                button::files_open(ui, open_dialog_token);
            }
        });
    });
}
