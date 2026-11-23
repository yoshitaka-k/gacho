use crate::{lang, file, event, ui};
use crate::event::button;
use crate::ui::assets::{self, icon, svg};

/// 上部パネル
/// * `ui` - UI
/// * `open_file` - 開いているファイル
/// * `open_dialog_token` - ファイルダイアログのトークン
/// * `pending_actions` - 待機中のアクション
pub(crate) fn view(
    ui: &mut egui::Ui,
    open_file: &mut file::OpenFile,
    open_dialog_token: &mut ui::OpenDialogToken,
    pending_actions: &mut Vec<event::EventAction>,
) {
    let style = ui::panel_style(ui, ui::TOP_PANEL_INNER_MARGIN);
    let icon_color = assets::button_icon_color(ui);

    // 上部パネルを表示
    egui::Panel::top("top_taskbar").frame(style).show(ui, |ui| {
        // 左右分割のレイアウトで、左にタイトル、右にボタンを配置する
        egui::Sides::new().shrink_left().truncate().show(ui,
            |ui| {
                let title = open_file.book_title();
                ui.add(egui::Label::new(title).truncate());
            },
            |ui| {
                // 設定ボタン
                let hover_text = lang::current().main_top.settings;
                let settings_button_image = egui::Image::new(svg::SETTINGS).tint(icon_color);
                if ui.add_sized(icon::ICON_BUTTON_SIZE, egui::Button::image(settings_button_image))
                    .on_hover_text(hover_text).clicked()
                {
                    let pos = ui.ctx().input(|input| {
                        input.viewport().outer_rect.map(|rect| rect.min)
                    });
                    pending_actions.push(event::EventAction::SettingOpen(pos));
                }

                // 閉じるボタン
                let hover_text = lang::current().main_top.close;
                let close_button_image = egui::Image::new(svg::CLEAR_ALL).tint(icon_color);
                if ui.add_enabled_ui(!open_file.book_is_empty(), |ui| {
                    ui.add_sized(icon::ICON_BUTTON_SIZE, egui::Button::image(close_button_image))
                }).inner.on_hover_text(hover_text).on_disabled_hover_text(hover_text).clicked()
                {
                    pending_actions.push(event::EventAction::Close);
                }

                // フォルダダイアログを開くボタン
                let hover_text = lang::current().main_top.folder_open;
                let open_button_image = egui::Image::new(svg::FOLDER_OPEN).tint(icon_color);
                if ui.add_sized(icon::ICON_BUTTON_SIZE, egui::Button::image(open_button_image))
                    .on_hover_text(hover_text).clicked()
                {
                    button::folder_open(ui, open_dialog_token);
                }

                // ファイルダイアログを開くボタン
                let hover_text = lang::current().main_top.files_open;
                let open_button_image = egui::Image::new(svg::FILE_OPEN).tint(icon_color);
                if ui.add_sized(icon::ICON_BUTTON_SIZE, egui::Button::image(open_button_image))
                    .on_hover_text(hover_text).clicked()
                {
                    button::files_open(ui, open_dialog_token);
                }
            },
        );
    });
}
