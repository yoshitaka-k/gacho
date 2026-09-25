use crate::{app, ui};
use crate::ui::assets::svg;
use crate::ui::setting;

const MIN_PRELOADING: usize = 0;
const MAX_PRELOADING: usize = 10;

/// 前処理数を表示
/// * `app` - アプリケーション
/// * `ui` - UI
pub(crate) fn view(ui: &mut egui::Ui, app: &mut app::App) {
    // ヘッダーパネルを表示
    setting::header_panel(ui, svg::SETTINGS, "General", None);

    ui.add_space(setting::SETTING_ADD_SPACING);
    ui.separator();
    ui.add_space(setting::SETTING_ADD_SPACING);

    egui::Frame::default().inner_margin(ui::PANEL_INNER_MARGIN).show(ui, |ui| {
        // 画面表示の表示方式を表示
        ui.horizontal(|ui| {
            ui::add_label(ui, "Page Layout:", setting::GENERAL_LABEL_WIDTH);
            ui.scope(|ui| {
                ui.radio_value(app.page_layout_mut(), app::PageLayout::Single, app::PageLayout::Single.to_string());
                ui.radio_value(app.page_layout_mut(), app::PageLayout::Spread, app::PageLayout::Spread.to_string());
            });
        });

        // 表紙表示方式を表示
        ui.horizontal(|ui| {
            ui::add_label(ui, "Cover Layout:", setting::GENERAL_LABEL_WIDTH);
            ui.scope(|ui| {
                ui.radio_value(app.cover_layout_mut(), app::CoverLayout::Single, app::CoverLayout::Single.to_string());
                ui.radio_value(app.cover_layout_mut(), app::CoverLayout::Spread, app::CoverLayout::Spread.to_string());
            });
        });

        // 画面表示の表示方式を変更した場合は本を再読み込みする必要がある
        setting::warning_note(ui, "Reopen the book file to apply the layout changes.");
    });

    ui.add_space(setting::SETTING_ADD_SPACING);
    ui.separator();
    ui.add_space(setting::SETTING_ADD_SPACING);

    // ページ送り方向を表示
    egui::Frame::default().inner_margin(ui::PANEL_INNER_MARGIN).show(ui, |ui| {
        ui.horizontal(|ui| {
            ui::add_label(ui, "Read From:", setting::GENERAL_LABEL_WIDTH);
            ui.scope(|ui| {
                ui.radio_value(app.read_from_mut(), app::ReadFrom::RightToLeft, app::ReadFrom::RightToLeft.to_string());
                ui.radio_value(app.read_from_mut(), app::ReadFrom::LeftToRight, app::ReadFrom::LeftToRight.to_string());
            });
        });
    });

    ui.add_space(setting::SETTING_ADD_SPACING);
    ui.separator();
    ui.add_space(setting::SETTING_ADD_SPACING);

    // 前処理数を表示
    egui::Frame::default().inner_margin(ui::PANEL_INNER_MARGIN).show(ui, |ui| {
        ui.horizontal(|ui| {
            ui::add_label(ui, "Preloading:", setting::GENERAL_LABEL_WIDTH);
            ui.scope(|ui| {
                ui.spacing_mut().slider_width = setting::remaining_slider_width(ui);
                ui.add(egui::Slider::new(app.preloading_mut(), MIN_PRELOADING..=MAX_PRELOADING));
            });
        });
    });
}
