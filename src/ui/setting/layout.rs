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
    // 読み込み方向のラベルの幅を設定
    let readfrom_label_width = if lang::current().code == "en" {
        setting::READFROM_EN_LABEL_WIDTH
    } else {
        setting::READFROM_JA_LABEL_WIDTH
    };

    // ラベルの幅を設定
    let layout_label_width = if lang::current().code == "en" {
        setting::LAYOUT_EN_LABEL_WIDTH
    } else {
        setting::LAYOUT_JA_LABEL_WIDTH
    };

    // ヘッダーパネルを表示
    setting::header_panel(ui, svg::IMAGE, lang::current().setting_general.layout);

    egui::Frame::group(ui.style()).show(ui, |ui| {
        // 横幅いっぱいまで伸ばす
        ui.take_available_width();

        // ページ送り方向を表示
        ui.horizontal(|ui| {
            ui::add_label(ui, lang::current().setting_general.read_from, readfrom_label_width);
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
            ui::add_label(ui, lang::current().setting_general.page_layout, layout_label_width);
            ui.scope(|ui| {
                ui.radio_value(config.page_layout_mut(), config::PageLayout::Single, lang::current().setting_general.single_page);
                ui.radio_value(config.page_layout_mut(), config::PageLayout::Spread, lang::current().setting_general.spread_page);
            });
        });

        // 表紙表示方式を表示
        ui.horizontal(|ui| {
            ui::add_label(ui, lang::current().setting_general.cover_layout, layout_label_width);
            ui.scope(|ui| {
                ui.radio_value(config.cover_layout_mut(), config::CoverLayout::Single, lang::current().setting_general.single_cover);
                ui.radio_value(config.cover_layout_mut(), config::CoverLayout::Spread, lang::current().setting_general.spread_cover);
            });
        });

        // 画面表示の表示方式を変更した場合は本を再読み込みする必要がある
        setting::warning_note(ui, lang::current().setting_general.layout_warning);
    });
}
