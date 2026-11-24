use crate::{state, lang, ui, update};
use crate::ui::assets::{icon, svg};

// モーダルのラベルの幅
pub(crate) const MODAL_LABEL_WIDTH: f32 = 130.0;

/// エラーモーダルを表示
/// * `show_modal` - モーダルを表示するかどうか
/// * `ctx` - コンテキスト
/// * `error_token` - エラーモーダルを表示するためのトークン
pub(crate) fn error(ctx: &egui::Context, error_token: &mut state::ErrorToken) {
    if error_token.value.is_none() {
        return;
    }

    // モーダルを表示
    let modal = egui::Modal::new(egui::Id::new("modal_error")).show(ctx, |ui| {
        ui.set_width(ui::MODAL_WINDOW_WIDTH);

        // 見出し部分
        ui.horizontal(|ui| {
            ui.add(heading_icon(svg::ERROR, icon::MODAL_ERROR_ICON_SIZE, egui::Color32::RED));
            ui.heading(lang::current().modal_error.heading);
        });

        ui.separator();

        ui.add_space(ui::MODAL_WINDOW_SPACING);

        // エラー内容を表示
        if let Some(error) = &error_token.value {
            egui::Frame::default().inner_margin(ui::PANEL_INNER_MARGIN).show(ui, |ui| {
                ui.label(error.to_string());

                // エラーコードを表示
                let code = error.code();
                ui.add_space(ui::MODAL_WINDOW_SPACING);
                ui.label(egui::RichText::new(format!("Code: {}", code)).weak());
            });
        }

        ui.add_space(ui::MODAL_WINDOW_SPACING);
    });

    // モーダルを閉じたらモーダルを非表示にする
    if modal.should_close() {
        error_token.dismiss();
    }
}

/// 更新モーダルを表示
/// * `ctx` - コンテキスト
/// * `update_token` - 更新モーダルを表示するためのトークン
pub(crate) fn updated(ctx: &egui::Context, update_token: &mut update::UpdateToken) {
    let Some(check) = update_token.check.clone() else {
        return;
    };

    // モーダルを表示
    let modal = egui::Modal::new(egui::Id::new("modal_updated")).show(ctx, |ui| {
        ui.set_width(ui::MODAL_WINDOW_WIDTH);

        // 見出し部分
        ui.horizontal(|ui| {
            match &check {
                // アップデートがある場合
                update::UpdateCheck::Available { .. } => {
                    ui.add(heading_icon(svg::UPDATE, icon::MODAL_UPDATE_ICON_SIZE, egui::Color32::GREEN));
                    ui.heading(lang::current().modal_update.available);
                }
                // アップデートが最新の場合
                update::UpdateCheck::Latest => {
                    ui.add(heading_icon(svg::UPDATE, icon::MODAL_UPDATE_ICON_SIZE, ui.visuals().text_color()));
                    ui.heading(lang::current().modal_update.latest);
                }
                // アップデートが取得できなかった場合
                update::UpdateCheck::Failed(_) => {
                    ui.add(heading_icon(svg::ERROR, icon::MODAL_ERROR_ICON_SIZE, egui::Color32::RED));
                    ui.heading(lang::current().modal_update.failed);
                }
            }
        });

        ui.separator();

        ui.add_space(ui::MODAL_WINDOW_SPACING);

        // 内容部分
        match &check {
            // アップデートがある場合
            update::UpdateCheck::Available { version, url } => {
                egui::Frame::default().inner_margin(ui::PANEL_INNER_MARGIN).show(ui, |ui| {
                    ui::add_label(ui, &format!("{}{}", lang::current().modal_update.new_version, version), MODAL_LABEL_WIDTH);
                    ui.add(egui::Label::new(
                        egui::RichText::new(format!("{}{}", lang::current().modal_update.current_version, env!("CARGO_PKG_VERSION"))).weak(),
                    ));
                    ui.add_space(ui::MODAL_WINDOW_SPACING);
                });

                ui.separator();

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button(lang::current().modal_update.download).clicked() {
                        ui.ctx().open_url(egui::OpenUrl::new_tab(url));
                    }
                });
            }
            // アップデートが最新の場合
            update::UpdateCheck::Latest => {
                egui::Frame::default().inner_margin(ui::PANEL_INNER_MARGIN).show(ui, |ui| {
                    ui.label(format!("{}{}", lang::current().modal_update.current_version, env!("CARGO_PKG_VERSION")));
                    ui.add_space(ui::MODAL_WINDOW_SPACING);
                });
            }
            // アップデートが取得できなかった場合
            update::UpdateCheck::Failed(e) => {
                egui::Frame::default().inner_margin(ui::PANEL_INNER_MARGIN).show(ui, |ui| {
                    ui.label(e.to_string());
                    ui.add_space(ui::MODAL_WINDOW_SPACING);
                });
            }
        }
    });

    // モーダルを閉じたらモーダルを非表示にする
    if modal.should_close() {
        update_token.open = false;
        update_token.check = None;
    }
}

/// 見出し横のアイコン
/// * `icon` - アイコン
/// * `size` - アイコンのサイズ
/// * `tint` - アイコンの色
/// * `return` - アイコンウィジェット
fn heading_icon(icon: egui::ImageSource<'static>, size: f32, tint: egui::Color32) -> egui::Image<'static> {
    // horizontal の行高 (interact_size.y = 18) に丸められるため、実サイズ (size) を指定する
    egui::Image::new(icon).fit_to_exact_size(egui::vec2(size, size)).tint(tint)
}
