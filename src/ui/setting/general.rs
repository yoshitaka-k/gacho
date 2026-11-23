use crate::{config, lang};
use crate::ui::assets::svg;
use crate::ui::setting;

/// 前処理数を表示
/// * `config` - 設定
/// * `ui` - UI
pub(crate) fn view(
    ui: &mut egui::Ui,
    config: &mut config::Config
) {
    // 言語選択のヘッダーパネルを表示
    setting::header_panel(ui, svg::LANGUAGE, lang::current().setting_general.language);

    // 言語選択のフレームを表示
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.take_available_width();

        ui.horizontal(|ui| {
            for lang in lang::LIST {
                if ui.radio_value(config.language_mut(), lang.code.to_string(), lang.name.to_string()).changed() {
                    lang::set(lang.code);
                }
            }
        });
    });
}
