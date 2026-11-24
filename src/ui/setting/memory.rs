use crate::{config, lang, ui};
use crate::ui::assets::svg;
use crate::ui::setting;

/// 前処理数を表示
/// * `config` - 設定
/// * `ui` - UI
pub(crate) fn view(
    ui: &mut egui::Ui,
    config: &mut config::Config
) {
    // メモリのラベルの幅を設定
    let memory_label_width = if lang::current().code == "en" {
        ui::MEMORY_EN_LABEL_WIDTH
    } else {
        ui::MEMORY_JA_LABEL_WIDTH
    };

    // 前処理数のラベルの幅を設定
    let preloading_label_width = if lang::current().code == "en" {
        ui::PRELOADING_EN_LABEL_WIDTH
    } else {
        ui::PRELOADING_JA_LABEL_WIDTH
    };

    // グループヘッダーパネルを表示
    setting::header_panel(ui, svg::MEMORY, lang::current().setting_general.memory);

    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.take_available_width();

        // 最後に読んだページを保存するかどうかを表示
        ui.horizontal(|ui| {
            ui::add_label(ui, lang::current().setting_general.remembered_last_page, memory_label_width);
            ui.scope(|ui| {
                ui.radio_value(config.remembered_last_page_mut(), true, lang::current().setting_general.yes);
                ui.radio_value(config.remembered_last_page_mut(), false, lang::current().setting_general.no);
            });
        });

        // 最後に読んだページを開くかどうかを表示
        ui.add_enabled_ui(*config.remembered_last_page(), |ui| {
            ui.horizontal(|ui| {
                ui::add_label(ui, lang::current().setting_general.open_last_page, memory_label_width);
                ui.scope(|ui| {
                    ui.radio_value(config.open_last_page_mut(), true, lang::current().setting_general.yes);
                    ui.radio_value(config.open_last_page_mut(), false, lang::current().setting_general.no);
                });
            });
        });

        // 最後に読んだページを開くかどうかを変更した場合は本を再読み込みする必要がある
        setting::warning_note(ui, lang::current().setting_general.memory_warning);
    });

    ui.add_space(ui::SETTING_ADD_SPACING);

    // グループヘッダーパネルを表示
    setting::header_panel(ui, svg::CACHED, lang::current().setting_general.preloading);

    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.take_available_width();

        // 前処理数を表示
        ui.horizontal(|ui| {
            ui::add_label(ui, lang::current().setting_general.load_pages, preloading_label_width);
            ui.scope(|ui| {
                ui.spacing_mut().slider_width = setting::remaining_slider_width(ui);
                ui.add(egui::Slider::new(config.preloading_mut(), config::MIN_PRELOADING..=config::MAX_PRELOADING));
            });
        });
    });
}
