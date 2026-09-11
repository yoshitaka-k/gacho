use crate::{app, event, file, ui};
use crate::ui::assets::{self, icon, svg};

const SLIDER_WIDTH: f32 = 50.0;
const MIN_INDEX: usize = 0;
const DEFAULT_MAX_INDEX: usize = 0;

/// 下部パネル
pub(crate) fn view(
    ui: &mut egui::Ui,
    app: &app::App,
    open_files: &mut file::OpenFiles,
    pending_actions: &mut Vec<event::EventAction>,
    error_token: &mut ui::ErrorToken,
) {
    // スタイルとボタンの色を設定
    let bottom_panel_style = ui::panel_style(ui, ui::BOTTOM_PANEL_INNER_MARGIN);
    let button_color = assets::button_icon_color(ui);

    // ファイルのIDリストと選択されたファイルを取得
    let all_images_ids = open_files.book_image_ids();
    let selected_images = open_files.selected_index_images(app).unwrap_or(vec![]);

    egui::Panel::bottom("bottom_taskbar").frame(bottom_panel_style).show(ui, |ui| {
        // 左右分割のレイアウトで、左にページャー・ファイル名、右にボタンを配置する
        egui::Sides::new().shrink_left().truncate().show(ui,
            |ui| {
                let mut pagers = vec![];
                let mut file_names = vec![];

                for (index, image) in selected_images.iter().enumerate() {
                    let image_index = all_images_ids.iter().position(|id| {
                        id == image.id()
                    }).unwrap_or(0);

                    pagers.push(format!("{}", image_index + 1));
                    file_names.push(image.file_name().as_str());

                    if index < selected_images.len() - 1 {
                        pagers.push("-".to_string());
                        file_names.push(" - ");
                    }
                }

                if pagers.is_empty() {
                    pagers.push("0".to_string());
                }

                // ページャー
                ui.label(format!("#{} / {}", pagers.join(""), all_images_ids.len()));

                ui.separator();

                // ファイル名
                ui.add(egui::Label::new(file_names.join("")).truncate());
            },
            |ui| {
                // ページャーを右寄せに配置
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // 前のファイルボタン
                    let hover_text = match app.read_from() {
                        event::ReadFrom::RightToLeft => "Previous page",
                        event::ReadFrom::LeftToRight => "Next page",
                    };
                    let prev_button = egui::Image::new(svg::KEYBOARD_ARROW_RIGHT)
                        .tint(button_color);
                    if ui.add_sized(icon::ICON_BUTTON_SIZE, egui::Button::image(prev_button))
                        .on_hover_text(hover_text).clicked()
                    {
                        pending_actions.push(event::EventAction::Right);
                    }

                    // 次のファイルボタン
                    let hover_text = match app.read_from() {
                        event::ReadFrom::RightToLeft => "Next page",
                        event::ReadFrom::LeftToRight => "Previous page",
                    };
                    let next_button = egui::Image::new(svg::KEYBOARD_ARROW_LEFT)
                        .tint(button_color);
                    if ui.add_sized(icon::ICON_BUTTON_SIZE, egui::Button::image(next_button))
                        .on_hover_text(hover_text).clicked()
                    {
                        pending_actions.push(event::EventAction::Left);
                    }

                    ui.separator();

                    // ページャースライダー
                    let mut selected = open_files.selected_index_mut().unwrap_or(0);
                    let max = if open_files.book_len() > 0 { open_files.book_len() - 1 } else { DEFAULT_MAX_INDEX };
                    ui.scope(|ui| {
                        ui.spacing_mut().slider_width = SLIDER_WIDTH;
                        let slider = match app.read_from() {
                            event::ReadFrom::RightToLeft => ui.add(
                                egui::Slider::new(&mut selected, max..=MIN_INDEX)
                                .show_value(false)
                            ),
                            event::ReadFrom::LeftToRight => ui.add(
                                egui::Slider::new(&mut selected, MIN_INDEX..=max)
                                .show_value(false)
                            ),
                        };

                        // 選択が変更された場合はインデックスを更新
                        if slider.changed() {
                            // エラーモーダルをリセット
                            error_token.reset();

                            // インデックスを更新
                            open_files.set_selected_index(Some(selected));
                        }
                    });

                    ui.separator();
                });
            }
        );
    });
}
