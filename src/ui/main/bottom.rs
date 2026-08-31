use crate::ui;

/// 下部パネル
pub(crate) fn view(ui: &mut egui::Ui) {
    let bottom_panel_style = ui::panel_style(ui, ui::BOTTOM_PANEL_INNER_MARGIN);

    egui::Panel::bottom("bottom_taskbar").frame(bottom_panel_style).show(ui, |ui| {
        ui.label("Status");
    });
}
