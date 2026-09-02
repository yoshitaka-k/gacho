use crate::{file, ui};

/// 下部パネル
pub(crate) fn view(ui: &mut egui::Ui, open_files: &file::OpenFiles) {
    let bottom_panel_style = ui::panel_style(ui, ui::BOTTOM_PANEL_INNER_MARGIN);

    egui::Panel::bottom("bottom_taskbar").frame(bottom_panel_style).show(ui, |ui| {
        ui.horizontal(|ui| {
            if let Some(index) = open_files.selected_index() {
                ui.label(format!("{} / {}", index + 1, open_files.images().len()));
            } else {
                ui.label(format!("0 / {}", open_files.images().len()));
            }

            ui.separator();

            if let Some(image_file) = open_files.selected_index_file() {
                ui.label(image_file.file_name());
            }
        });
    });
}
