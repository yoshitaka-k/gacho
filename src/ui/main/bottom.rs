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
    let bottom_panel_style = ui::panel_style(ui, ui::BOTTOM_PANEL_INNER_MARGIN);
    let button_color = assets::button_icon_color(ui);

    egui::Panel::bottom("bottom_taskbar").frame(bottom_panel_style).show(ui, |ui| {
        ui.horizontal(|ui| {
            if let Some(index) = open_files.selected_index() {
                match app.page_layout() {
                    event::PageLayout::Default | event::PageLayout::Single => {
                        ui.label(format!("{} / {}", index + 1, open_files.images().len()));
                    }
                    event::PageLayout::Spread => {
                        let first = index + 1;
                        let last = if open_files.images().len() > index + 1 + app.page_layout().to_offset() {
                            index + 1 + app.page_layout().to_offset()
                        } else {
                            open_files.images().len()
                        };

                        match app.read_from() {
                            event::ReadFrom::RightToLeft => {
                                ui.label(format!("{} - {} / {}", last, first, open_files.images().len()));
                            }
                            event::ReadFrom::LeftToRight => {
                                ui.label(format!("{} - {} / {}", first, last, open_files.images().len()));
                            }
                        }
                    }
                }
            } else {
                ui.label(format!("0 / {}", open_files.images().len()));
            }

            ui.separator();

            let file_name = open_files.file_name().unwrap_or("");
            ui.add(egui::Label::new(file_name).truncate());

            // ページャーを右寄せに配置
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // 前のファイル
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

                // 次のファイル
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

                ui.scope(|ui| {
                    let mut selected = open_files.selected_index_mut().unwrap_or(0);
                    let max = if open_files.len() > 0 { open_files.len() - 1 } else { DEFAULT_MAX_INDEX };

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
        });
    });
}
