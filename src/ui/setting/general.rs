use crate::{config, lang, ui};
use crate::ui::assets::svg;
use crate::ui::setting;

const MIN_PRELOADING: usize = 0;
const MAX_PRELOADING: usize = 10;

/// 前処理数を表示
/// * `config` - 設定
/// * `ui` - UI
pub(crate) fn view(ui: &mut egui::Ui, config: &mut config::Config) {
    // ヘッダーパネルを表示
    setting::header_panel(ui, svg::SETTINGS, lang::current().setting.general);

    ui.add_space(setting::SETTING_ADD_SPACING);
    ui.separator();
    ui.add_space(setting::SETTING_ADD_SPACING);

    // グループパネル名を表示
    setting::group_header_panel(ui, lang::current().setting_general.layout);

    egui::Frame::group(ui.style()).show(ui, |ui| {
        // 横幅いっぱいまで伸ばす
        ui.take_available_width();

        // ページ送り方向を表示
        ui.horizontal(|ui| {
            ui::add_label(ui, lang::current().setting_general.read_from, setting::GENERAL_LABEL_WIDTH);
            ui.scope(|ui| {
                ui.radio_value(config.read_from_mut(), config::ReadFrom::RightToLeft, lang::current().setting_general.rtl);
                ui.radio_value(config.read_from_mut(), config::ReadFrom::LeftToRight, lang::current().setting_general.ltr);
            });
        });

        ui.add_space(setting::SETTING_ADD_SPACING);
        ui.separator();
        ui.add_space(setting::SETTING_ADD_SPACING);

        // 画面表示の表示方式を表示
        ui.horizontal(|ui| {
            ui::add_label(ui, lang::current().setting_general.page_layout, setting::GENERAL_LABEL_WIDTH);
            ui.scope(|ui| {
                ui.radio_value(config.page_layout_mut(), config::PageLayout::Single, lang::current().setting_general.single_page);
                ui.radio_value(config.page_layout_mut(), config::PageLayout::Spread, lang::current().setting_general.spread_page);
            });
        });

        // 表紙表示方式を表示
        ui.horizontal(|ui| {
            ui::add_label(ui, lang::current().setting_general.cover_layout, setting::GENERAL_LABEL_WIDTH);
            ui.scope(|ui| {
                ui.radio_value(config.cover_layout_mut(), config::CoverLayout::Single, lang::current().setting_general.single_cover);
                ui.radio_value(config.cover_layout_mut(), config::CoverLayout::Spread, lang::current().setting_general.spread_cover);
            });
        });

        // 画面表示の表示方式を変更した場合は本を再読み込みする必要がある
        setting::warning_note(ui, lang::current().setting_general.layout_warning);
    });

    ui.add_space(setting::SETTING_ADD_SPACING);

    // グループヘッダーパネルを表示
    setting::group_header_panel(ui, lang::current().setting_general.memory);

    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.take_available_width();

        // 最後に読んだページを保存するかどうかを表示
        ui.horizontal(|ui| {
            ui::add_label(ui, lang::current().setting_general.remembered_last_page, setting::GENERAL_LONG_LABEL_WIDTH);
            ui.scope(|ui| {
                ui.radio_value(config.remembered_last_page_mut(), true, lang::current().setting_general.yes);
                ui.radio_value(config.remembered_last_page_mut(), false, lang::current().setting_general.no);
            });
        });

        // 最後に読んだページを開くかどうかを表示
        ui.add_enabled_ui(*config.remembered_last_page(), |ui| {
            ui.horizontal(|ui| {
                ui::add_label(ui, lang::current().setting_general.open_last_page, setting::GENERAL_LONG_LABEL_WIDTH);
                ui.scope(|ui| {
                    ui.radio_value(config.open_last_page_mut(), true, lang::current().setting_general.yes);
                    ui.radio_value(config.open_last_page_mut(), false, lang::current().setting_general.no);
                });
            });
        });

        // 最後に読んだページを開くかどうかを変更した場合は本を再読み込みする必要がある
        setting::warning_note(ui, lang::current().setting_general.memory_warning);
    });

    ui.add_space(setting::SETTING_ADD_SPACING);

    // グループヘッダーパネルを表示
    setting::group_header_panel(ui, lang::current().setting_general.preloading);

    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.take_available_width();

        // 前処理数を表示
        ui.horizontal(|ui| {
            ui::add_label(ui, lang::current().setting_general.load_pages, setting::GENERAL_LABEL_WIDTH);
            ui.scope(|ui| {
                ui.spacing_mut().slider_width = setting::remaining_slider_width(ui);
                ui.add(egui::Slider::new(config.preloading_mut(), MIN_PRELOADING..=MAX_PRELOADING));
            });
        });
    });
}
