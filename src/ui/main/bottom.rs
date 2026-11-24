use crate::{config, event, file, ui};
use crate::ui::assets::{self, icon, label, svg};

/// 最小インデックス
const MIN_INDEX: usize = 0;
/// デフォルトの最大インデックス
const DEFAULT_MAX_INDEX: usize = 0;

/// 下部パネル
/// * `ui` - UI
/// * `config` - 設定
/// * `files` - 開いているファイル
/// * `pending_actions` - 待機中のアクション
pub(crate) fn view(
    ui: &mut egui::Ui,
    config: &config::Config,
    files: &mut file::OpenFile,
    pending_actions: &mut Vec<event::EventAction>,
) {
    // スタイルとボタンの色を設定
    let style = ui::panel_style(ui, ui::BOTTOM_PANEL_INNER_MARGIN);
    let icon_color = assets::button_icon_color(ui);

    // ファイルのIDリストと選択されたファイルを取得
    let all_images_ids = files.book_image_ids();
    let selected_images = files.page_images(None).unwrap_or(vec![]);

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
    egui::Panel::bottom("bottom_taskbar").frame(style).show(ui, |ui| {
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
                let hover_text = label::MainBottom::rightmost(config.read_from(), files.is_current_pages(), files.is_current_books());
                let rightmost_button_image = egui::Image::new(svg::LAST_PAGE)
                    .tint(icon_color);
                if ui.add_enabled_ui(!files.book_is_empty(), |ui| {
                    ui.add_sized(icon::ICON_BUTTON_SIZE, egui::Button::image(rightmost_button_image))
                }).inner.on_hover_text(hover_text).on_disabled_hover_text(hover_text).clicked()
                {
                    pending_actions.push(event::EventAction::Rightmost);
                }

                // 右矢印のファイルボタン
                let hover_text = label::MainBottom::right(config.read_from(), files.is_current_pages(), files.is_current_books());
                let right_button_image = egui::Image::new(svg::KEYBOARD_ARROW_RIGHT)
                    .tint(icon_color);
                if ui.add_enabled_ui(!files.book_is_empty(), |ui| {
                    ui.add_sized(icon::ICON_BUTTON_SIZE, egui::Button::image(right_button_image))
                }).inner.on_hover_text(hover_text).on_disabled_hover_text(hover_text).clicked()
                {
                    pending_actions.push(event::EventAction::Right);
                }

                // 左矢印のファイルボタン
                let hover_text = label::MainBottom::left(config.read_from(), files.is_current_pages(), files.is_current_books());
                let left_button_image = egui::Image::new(svg::KEYBOARD_ARROW_LEFT)
                    .tint(icon_color);
                if ui.add_enabled_ui(!files.book_is_empty(), |ui| {
                    ui.add_sized(icon::ICON_BUTTON_SIZE, egui::Button::image(left_button_image))
                }).inner.on_hover_text(hover_text).on_disabled_hover_text(hover_text).clicked()
                {
                    pending_actions.push(event::EventAction::Left);
                }

                // 左端のページボタン
                let hover_text = label::MainBottom::leftmost(config.read_from(), files.is_current_pages(), files.is_current_books());
                let leftmost_button_image = egui::Image::new(svg::FIRST_PAGE)
                    .tint(icon_color);
                if ui.add_enabled_ui(!files.book_is_empty(), |ui| {
                    ui.add_sized(icon::ICON_BUTTON_SIZE, egui::Button::image(leftmost_button_image))
                }).inner.on_hover_text(hover_text).on_disabled_hover_text(hover_text).clicked()
                {
                    pending_actions.push(event::EventAction::Leftmost);
                }

                ui.separator();

                // ページャースライダー
                let mut selected = files.current_spread_mut().unwrap_or(0);
                let max = if files.spreads_len() > 0 { files.spreads_len() - 1 } else { DEFAULT_MAX_INDEX };
                ui.scope(|ui| {
                    ui.spacing_mut().slider_width = ui::SLIDER_WIDTH;
                    let slider = match config.read_from() {
                        config::ReadFrom::RightToLeft => {
                            ui.add_enabled(
                                !files.book_is_empty(),
                                egui::Slider::new(&mut selected, max..=MIN_INDEX).show_value(false)
                            )
                        }
                        config::ReadFrom::LeftToRight => {
                            ui.add_enabled(
                                !files.book_is_empty(),
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
