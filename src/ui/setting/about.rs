use crate::{GITHUB_URL, lang, ui, update, event};
use crate::ui::{assets, setting};
use crate::ui::assets::{svg, icon};

/// バージョンを表示
/// * `ui` - UI
/// * `update_token` - アップデートトークン
/// * `pending_actions` - 待機中のアクション
pub(crate) fn view(
    ui: &mut egui::Ui,
    update_token: &update::UpdateToken,
    pending_actions: &mut Vec<event::EventAction>,
) {
    // ヘッダーパネルを表示
    setting::about_header_panel(
        ui,
        svg::INFO,
        lang::current().setting.about,
        update_token,
        pending_actions,
    );

    ui.add_space(ui::HEADER_BOTTOM_SPACING);

    ui.separator();

    egui::Frame::default().inner_margin(ui::PANEL_INNER_MARGIN).show(ui, |ui| {
        // アプリのアイコンを表示
        ui.add(egui::Image::new(assets::APP_ICON).max_height(icon::APP_ICON_SIZE));

        // バージョンを表示
        ui.label(format!("Gacho v{}", env!("CARGO_PKG_VERSION")));

        // 説明を表示
        ui.label(env!("CARGO_PKG_DESCRIPTION"));

        ui.label("");

        // ライセンスを表示
        ui.label(format!("License: {}", env!("CARGO_PKG_LICENSE")));

        // リポジトリを表示
        ui.horizontal(|ui| {
            ui.label("Repository:");
            ui.hyperlink_to(env!("CARGO_PKG_REPOSITORY"), GITHUB_URL.replace("{repository}", env!("CARGO_PKG_REPOSITORY")));
        });
    });
}
