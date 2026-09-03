use std::sync::Arc;
use crate::{app, error, file, ui};

/// メインパネル
pub(crate) fn view(
    ui: &mut egui::Ui,
    app: &app::App,
    open_files: &mut file::OpenFiles,
    error_token: &mut ui::ErrorToken,
) {
    egui::CentralPanel::default().show(ui, |ui| {
        // 中央寄せでレイアウトを指定
        ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
            // 試しに最初の画像を表示
            if let Some(image_file) = open_files.selected_index_file() {
                let image = ui_image(image_file)
                    .max_size(ui.available_size());
                image_load(ui, image, image_file, true, error_token);
            }

            if *app.preloading() > 0 {
                for i in 1..=*app.preloading() {
                    // 次の画像を表示
                    if let Some(image_file) = open_files.selected_next_file(i) {
                        let image = ui_image(image_file)
                            .fit_to_exact_size(egui::vec2(0.0, 0.0));
                        image_load(ui, image, image_file, false, error_token);
                    }
                }

                for i in 1..=*app.preloading() {
                    // 前の画像を表示
                    if let Some(image_file) = open_files.selected_prev_file(i) {
                        let image = ui_image(image_file)
                            .fit_to_exact_size(egui::vec2(0.0, 0.0));
                        image_load(ui, image, image_file, false, error_token);
                    }
                }
            }
        });
    });
}

/// 画像を表示する
/// * `image_file` - 画像ファイル
/// * `return` - 画像
fn ui_image(image_file: &file::Image) -> egui::Image<'static> {
    // バイト列かどうかを判断
    let image = if image_file.is_archive() {
        let uri = format!("bytes://{}/{}", image_file.file_name(), image_file.id());
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
fn image_load(ui: &mut egui::Ui, image: egui::Image<'_>, image_file: &file::Image, is_loading: bool, error_token: &mut ui::ErrorToken) {
    match image.load_for_size(ui.ctx(), ui.available_size()) {
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
            eprintln!("Error loading image: {}", e);
            let error = error::GachoError::FileError(e.to_string(), image_file.path().clone());
            error_token.open = true;
            error_token.value = Some(error);
        }
    }
}
