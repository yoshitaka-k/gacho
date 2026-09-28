use crate::{app, event, file, ui};
use crate::ui::assets::{self, icon, label, svg};

const SLIDER_WIDTH: f32 = 50.0;
const MIN_INDEX: usize = 0;
const DEFAULT_MAX_INDEX: usize = 0;

/// 下部パネル
/// * `ui` - UI
/// * `app` - アプリケーション
/// * `open_file` - 開いているファイル
/// * `pending_actions` - 待機中のアクション
pub(crate) fn view(
    ui: &mut egui::Ui,
    app: &app::App,
    open_file: &mut file::OpenFile,
    pending_actions: &mut Vec<event::EventAction>,
) {
    // スタイルとボタンの色を設定
    let bottom_panel_style = ui::panel_style(ui, ui::BOTTOM_PANEL_INNER_MARGIN);
    let button_color = assets::button_icon_color(ui);

    // ファイルのIDリストと選択されたファイルを取得
    let all_images_ids = open_file.book_image_ids();
    let selected_images = open_file.page_images(None).unwrap_or(vec![]);

    // ページャーとファイル名を生成
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
    } else {
        pagers.reverse();
    }

    // 下部パネルを表示
    egui::Panel::bottom("bottom_taskbar").frame(bottom_panel_style).show(ui, |ui| {
        // 左右分割のレイアウトで、左にページャー・ファイル名、右にボタンを配置する
        egui::Sides::new().shrink_left().truncate().show(ui,
            |ui| {
                // ページャー
                ui.label(format!("#{} / {}", pagers.join(""), all_images_ids.len()));

                ui.separator();

                // ファイル名
                ui.add(egui::Label::new(file_names.join("")).truncate());
            },
            |ui| {
                // 右端のページボタン
                let hover_text = label::MainBottom::rightmost(app.read_from());
                let rightmost_button_image = egui::Image::new(svg::LAST_PAGE)
                    .tint(button_color);
                if ui.add_enabled_ui(!open_file.book_is_empty(), |ui| {
                    ui.add_sized(icon::ICON_BUTTON_SIZE, egui::Button::image(rightmost_button_image))
                }).inner.on_hover_text(hover_text).on_disabled_hover_text(hover_text).clicked()
                {
                    pending_actions.push(event::EventAction::Rightmost);
                }

                // 右矢印のファイルボタン
                let hover_text = label::MainBottom::right(app.read_from());
                let right_button_image = egui::Image::new(svg::KEYBOARD_ARROW_RIGHT)
                    .tint(button_color);
                if ui.add_enabled_ui(!open_file.book_is_empty(), |ui| {
                    ui.add_sized(icon::ICON_BUTTON_SIZE, egui::Button::image(right_button_image))
                }).inner.on_hover_text(hover_text).on_disabled_hover_text(hover_text).clicked()
                {
                    pending_actions.push(event::EventAction::Right);
                }

                // 左矢印のファイルボタン
                let hover_text = label::MainBottom::left(app.read_from());
                let left_button_image = egui::Image::new(svg::KEYBOARD_ARROW_LEFT)
                    .tint(button_color);
                if ui.add_enabled_ui(!open_file.book_is_empty(), |ui| {
                    ui.add_sized(icon::ICON_BUTTON_SIZE, egui::Button::image(left_button_image))
                }).inner.on_hover_text(hover_text).on_disabled_hover_text(hover_text).clicked()
                {
                    pending_actions.push(event::EventAction::Left);
                }

                // 左端のページボタン
                let hover_text = label::MainBottom::leftmost(app.read_from());
                let leftmost_button_image = egui::Image::new(svg::FIRST_PAGE)
                    .tint(button_color);
                if ui.add_enabled_ui(!open_file.book_is_empty(), |ui| {
                    ui.add_sized(icon::ICON_BUTTON_SIZE, egui::Button::image(leftmost_button_image))
                }).inner.on_hover_text(hover_text).on_disabled_hover_text(hover_text).clicked()
                {
                    pending_actions.push(event::EventAction::Leftmost);
                }

                ui.separator();

                // ページャースライダー
                let mut selected = open_file.current_spread_mut().unwrap_or(0);
                let max = if open_file.spreads_len() > 0 { open_file.spreads_len() - 1 } else { DEFAULT_MAX_INDEX };
                ui.scope(|ui| {
                    ui.spacing_mut().slider_width = SLIDER_WIDTH;
                    let slider = match app.read_from() {
                        app::ReadFrom::RightToLeft => {
                            ui.add_enabled(
                                !open_file.book_is_empty(),
                                egui::Slider::new(&mut selected, max..=MIN_INDEX).show_value(false)
                            )
                        }
                        app::ReadFrom::LeftToRight => {
                            ui.add_enabled(
                                !open_file.book_is_empty(),
                                egui::Slider::new(&mut selected, MIN_INDEX..=max).show_value(false)
                            )
                        }
                    };

                    // 選択が変更された場合はインデックスを更新
                    if slider.changed() {
                        pending_actions.push(event::EventAction::Slider(selected));
                    }
                });

                ui.separator();
            }
        );
    });
}
