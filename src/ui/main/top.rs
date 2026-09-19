use crate::event::button;
use crate::{event, file, ui};
use crate::ui::assets::{self, icon, svg};

/// 上部パネル
/// * `ui` - UI
/// * `open_file` - 開いているファイル
/// * `setting_token` - 設定ダイアログのトークン
/// * `open_dialog_token` - ファイルダイアログのトークン
/// * `pending_actions` - 待機中のアクション
pub(crate) fn view(
    ui: &mut egui::Ui,
    open_file: &mut file::OpenFile,
    setting_token: &mut ui::SettingToken,
    open_dialog_token: &mut ui::OpenDialogToken,
    pending_actions: &mut Vec<event::EventAction>,
) {
    let top_panel_style = ui::panel_style(ui, ui::TOP_PANEL_INNER_MARGIN);
    let button_color = assets::button_icon_color(ui);

    // 上部パネルを表示
    egui::Panel::top("top_taskbar").frame(top_panel_style).show(ui, |ui| {
        // 左右分割のレイアウトで、左にタイトル、右にボタンを配置する
        egui::Sides::new().shrink_left().truncate().show(ui,
            |ui| {
                let title = open_file.book_title();
                ui.add(egui::Label::new(title).truncate());
            },
            |ui| {
                // 設定ボタン
                let hover_text = "Settings";
                let settings_button = egui::Image::new(svg::SETTINGS)
                    .tint(button_color);
                if ui.add_sized(icon::ICON_BUTTON_SIZE, egui::Button::image(settings_button))
                    .on_hover_text(hover_text).clicked()
                {
                    button::setting_open(ui, setting_token);
                }

                // 閉じるボタン
                let close_button = egui::Image::new(svg::CLEAR_ALL)
                    .tint(button_color);
                if ui.add_sized(icon::ICON_BUTTON_SIZE, egui::Button::image(close_button))
                    .on_hover_text("File Close").clicked()
                {
                    pending_actions.push(event::EventAction::Close);
                }

                // フォルダダイアログを開くボタン
                let hover_text = "Folder Open";
                let open_button = egui::Image::new(svg::FOLDER_OPEN)
                    .tint(button_color);
                if ui.add_sized(icon::ICON_BUTTON_SIZE, egui::Button::image(open_button))
                    .on_hover_text(hover_text).clicked()
                {
                    button::folder_open(ui, open_dialog_token);
                }

                let hover_text = "Files Open";
                let open_button = egui::Image::new(svg::FILE_OPEN)
                    .tint(button_color);
                if ui.add_sized(icon::ICON_BUTTON_SIZE, egui::Button::image(open_button))
                    .on_hover_text(hover_text).clicked()
                {
                    button::files_open(ui, open_dialog_token);
                }
            },
        );
    });
}
