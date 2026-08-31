use crate::ui;
use crate::ui::assets::svg;
use crate::ui::setting;

/// 並行処理数を表示
/// * `ui` - UI
pub(crate) fn view(ui: &mut egui::Ui) {
    // ヘッダーパネルを表示
    setting::header_panel(ui, svg::SETTINGS, "General", None);

    ui.add_space(setting::HEADER_BOTTOM_SPACING);

    ui.separator();

    ui.add_space(setting::SETTING_ADD_SPACING);

    // フレームを表示
    egui::Frame::default().inner_margin(ui::PANEL_INNER_MARGIN).show(ui, |ui| {
        ui::add_label(ui, "General Settings", setting::GENERAL_LABEL_WIDTH);

        // 同じパスはスキップの注意書きを表示
        setting::warning_note(ui, "warning note.");
    });
}
