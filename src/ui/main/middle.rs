use std::sync::Arc;
use crate::{app, error, event, file, ui};

/// メインパネルを表示する
/// * `ui` - UI
/// * `app` - アプリケーション
/// * `open_files` - 開いているファイル
/// * `pending_actions` - 処理予約
/// * `error_token` - エラートークン
/// メインパネル
pub(crate) fn view(
    ui: &mut egui::Ui,
    app: &app::App,
    open_files: &mut file::OpenFiles,
    pending_actions: &mut Vec<event::EventAction>,
    error_token: &mut ui::ErrorToken,
) {
    egui::CentralPanel::default().show(ui, |ui| {
        // クリックエリアを作成
        let rect = ui.max_rect();
        let click = ui.interact(rect, ui.id().with("middle"), egui::Sense::click());

        // 本画像の available_size を先に取る
        let available_rect = ui.available_rect_before_wrap();
        let available = available_rect.size();

        match open_files.selected_index_images(app) {
            Ok(images) if !images.is_empty() => {
                let n = images.len() as f32;
                let max_each = egui::vec2(available.x / n, available.y);
                let sizes: Vec<egui::Vec2> = images.iter().map(|image| image.fit_to(max_each)).collect();

                let total_width: f32 = sizes.iter().map(|size| size.x).sum();
                let total_height = sizes.iter().map(|size| size.y).fold(0.0_f32, f32::max);
                let rect = egui::Align2::CENTER_CENTER
                    .align_size_within_rect(egui::vec2(total_width, total_height), available_rect);

                ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.horizontal(|ui| {
                        let mut images: Vec<(&file::Image, egui::Vec2)> =
                            images.iter().zip(sizes).collect();
                        if matches!(app.read_from(), event::ReadFrom::RightToLeft) {
                            images.reverse();
                        }

                        for (image_file, size) in images {
                            let image = ui_image(image_file).fit_to_exact_size(size);
                            image_load(ui, image, image_file, size, true, error_token);
                        }
                    });
                });
            }
            Ok(_) => (),
            Err(e) => {
                error_token.show(e);
            }
        };

        // 前後の画像を先読み
        for i in 1..=*app.preloading() {
            if let Ok(Some(image_file)) = open_files.selected_next_image(i) {
                let _ = ui_image(&image_file).load_for_size(ui.ctx(), available);
            }

            if let Ok(Some(image_file)) = open_files.selected_prev_image(i) {
                let _ = ui_image(&image_file).load_for_size(ui.ctx(), available);
            }
        }

        // クリックイベントを処理
        if click.clicked() {
            if let Some(pos) = click.interact_pointer_pos() {
                pending_actions.push(event::EventAction::Click(pos));
            }
        }
    });
}

/// 画像を表示する
/// * `image_file` - 画像ファイル
/// * `return` - 画像
fn ui_image(image_file: &file::Image) -> egui::Image<'static> {
    // バイト列かどうかを判断
    let image = if image_file.is_archive() {
        let uri = format!("bytes://{}/{}/{}", image_file.path().display(), image_file.id(), image_file.file_name());
        egui::Image::from_bytes(uri, Arc::clone(&image_file.bytes()))
    } else {
        let uri = format!("file://{}", image_file.path().display());
        egui::Image::new(uri)
    };

    image.maintain_aspect_ratio(true)
        .show_loading_spinner(false)
}

/// 画像を読み込む
/// * `ui` - UI
/// * `image` - 画像
/// * `error_token` - エラートークン
/// * `return` - 読み込み完了
fn image_load(
    ui: &mut egui::Ui,
    image: egui::Image<'_>,
    image_file: &file::Image,
    size: egui::Vec2,
    is_loading: bool,
    error_token: &mut ui::ErrorToken,
) {
    match image.load_for_size(ui.ctx(), size) {
        Ok(egui::load::TexturePoll::Ready { .. }) => {
            // 読み込み完了
            ui.add(image);
        }
        Ok(egui::load::TexturePoll::Pending { .. }) => {
            // 読み込み中
            if is_loading {
                ui.spinner();
            }
        }
        Err(e) => {
            // 読み込みエラー
            let error = error::GachoError::FileError(e.to_string(), image_file.path().clone());
            error_token.show(error);
        }
    }
}
