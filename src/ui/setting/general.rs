use crate::{app, ui};
use crate::ui::assets::{label, svg};
use crate::ui::setting;

const MIN_PRELOADING: usize = 0;
const MAX_PRELOADING: usize = 10;

/// 前処理数を表示
/// * `app` - アプリケーション
/// * `ui` - UI
pub(crate) fn view(ui: &mut egui::Ui, app: &mut app::App) {
    // ヘッダーパネルを表示
    setting::header_panel(ui, svg::SETTINGS, label::SettingGeneral::heading(), None);

    ui.add_space(setting::SETTING_ADD_SPACING);
    ui.separator();
    ui.add_space(setting::SETTING_ADD_SPACING);

    egui::Frame::default().inner_margin(ui::PANEL_INNER_MARGIN).show(ui, |ui| {
        // 画面表示の表示方式を表示
        ui.horizontal(|ui| {
            ui::add_label(ui, label::SettingGeneral::page_layout(), setting::GENERAL_LABEL_WIDTH);
            ui.scope(|ui| {
                ui.radio_value(app.page_layout_mut(), app::PageLayout::Single, label::SettingGeneral::page_layout_single());
                ui.radio_value(app.page_layout_mut(), app::PageLayout::Spread, label::SettingGeneral::page_layout_spread());
            });
        });

        // 表紙表示方式を表示
        ui.horizontal(|ui| {
            ui::add_label(ui, label::SettingGeneral::cover_layout(), setting::GENERAL_LABEL_WIDTH);
            ui.scope(|ui| {
                ui.radio_value(app.cover_layout_mut(), app::CoverLayout::Single, label::SettingGeneral::cover_layout_single());
                ui.radio_value(app.cover_layout_mut(), app::CoverLayout::Spread, label::SettingGeneral::cover_layout_spread());
            });
        });

        // 画面表示の表示方式を変更した場合は本を再読み込みする必要がある
        setting::warning_note(ui, label::SettingGeneral::layout_warning());
    });

    ui.add_space(setting::SETTING_ADD_SPACING);
    ui.separator();
    ui.add_space(setting::SETTING_ADD_SPACING);

    // ページ送り方向を表示
    egui::Frame::default().inner_margin(ui::PANEL_INNER_MARGIN).show(ui, |ui| {
        ui.horizontal(|ui| {
            ui::add_label(ui, label::SettingGeneral::read_from(), setting::GENERAL_LABEL_WIDTH);
            ui.scope(|ui| {
                ui.radio_value(app.read_from_mut(), app::ReadFrom::RightToLeft, label::SettingGeneral::read_from_right_to_left());
                ui.radio_value(app.read_from_mut(), app::ReadFrom::LeftToRight, label::SettingGeneral::read_from_left_to_right());
            });
        });
    });

    ui.add_space(setting::SETTING_ADD_SPACING);
    ui.separator();
    ui.add_space(setting::SETTING_ADD_SPACING);

    // 前処理数を表示
    egui::Frame::default().inner_margin(ui::PANEL_INNER_MARGIN).show(ui, |ui| {
        ui.horizontal(|ui| {
            ui::add_label(ui, label::SettingGeneral::preloading(), setting::GENERAL_LABEL_WIDTH);
            ui.scope(|ui| {
                ui.spacing_mut().slider_width = setting::remaining_slider_width(ui);
                ui.add(egui::Slider::new(app.preloading_mut(), MIN_PRELOADING..=MAX_PRELOADING));
            });
        });
    });

    ui.add_space(setting::SETTING_ADD_SPACING);
    ui.separator();
    ui.add_space(setting::SETTING_ADD_SPACING);

    egui::Frame::default().inner_margin(ui::PANEL_INNER_MARGIN).show(ui, |ui| {
        // 最後に読んだページを保存するかどうかを表示
        ui.horizontal(|ui| {
            ui::add_label(ui, label::SettingGeneral::remembered_last_page(), setting::GENERAL_LONG_LABEL_WIDTH);
            ui.scope(|ui| {
                ui.radio_value(app.remembered_last_page_mut(), true, label::SettingGeneral::yes());
                ui.radio_value(app.remembered_last_page_mut(), false, label::SettingGeneral::no());
            });
        });

        // 最後に読んだページを開くかどうかを表示
        ui.add_enabled_ui(*app.remembered_last_page(), |ui| {
            ui.horizontal(|ui| {
                ui::add_label(ui, label::SettingGeneral::open_last_page(), setting::GENERAL_LONG_LABEL_WIDTH);
                ui.scope(|ui| {
                    ui.radio_value(app.open_last_page_mut(), true, label::SettingGeneral::yes());
                    ui.radio_value(app.open_last_page_mut(), false, label::SettingGeneral::no());
                });
            });
        });

        // 最後に読んだページを開くかどうかを変更した場合は本を再読み込みする必要がある
        setting::warning_note(ui, label::SettingGeneral::reopen_book_warning());
    });
}
