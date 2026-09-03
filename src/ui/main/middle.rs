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

        // 中央寄せでレイアウトを指定
        ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
            // 本画像の available_size を先に取る
            let available = ui.available_size();

            // 試しに最初の画像を表示
            if let Some(image_file) = open_files.selected_index_file() {
                let image = ui_image(image_file)
                    .max_size(available);
                image_load(ui, image, image_file, true, error_token);
            }

            // 前後の画像を先読み
            for i in 1..=*app.preloading() {
                if let Some(image_file) = open_files.selected_next_file(i) {
                    let _ = ui_image(image_file).load_for_size(ui.ctx(), available);
                }

                if let Some(image_file) = open_files.selected_prev_file(i) {
                    let _ = ui_image(image_file).load_for_size(ui.ctx(), available);
                }
            }
        });

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
