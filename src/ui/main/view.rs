use crate::app;
use crate::event::{open, input};
use crate::ui::{self, modal};
use crate::ui::assets::svg;
use crate::ui::main::{top, bottom};
use crate::ui::setting::view as setting_window;

/// 描画用のウィジェット
pub struct Render {
    app: app::App,

    // ファイルダイアログを開くタイミング
    open_dialog_token: ui::OpenDialogToken,

    // 設定ウィンドウのトークン
    setting_token: ui::SettingToken,

    // アップデートトークン
    updated_token: app::UpdatedToken,

    // アップデートジョブ
    update_job: app::UpdateJob,

    // エラーモーダルのトークン
    error_token: ui::ErrorToken,
}

impl Render {
    pub fn new(cc: &eframe::CreationContext<'_>, app: app::App) -> Self {
        // SVG ローダーを登録
        svg::install(&cc.egui_ctx);

        // 前回保存した App があれば復元（なければ引数の app を使う）
        let app = cc.storage
            .and_then(|storage| eframe::get_value(storage, eframe::APP_KEY))
            .unwrap_or(app);

        Self {
            app,
            open_dialog_token: ui::OpenDialogToken {
                file_dialog: false,
            },
            setting_token: ui::SettingToken {
                open: false,
                pos: None,
                tab: ui::SettingTab::General,
            },
            updated_token: app::UpdatedToken {
                open: false,
                check: None,
            },
            error_token: ui::ErrorToken {
                open: false,
                value: None,
            },
            update_job: app::UpdateJob::new(cc.egui_ctx.clone()),
        }
    }
}

impl eframe::App for Render {
    /// 終了前に App の状態を保存
    /// * `storage` - ストレージ
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.app);
    }

    /// 描画
    /// * `ui` - UI
    /// * `frame` - フレーム
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // スタイルを設定
        ui.ctx().global_style_mut(|style| {
            // ラベルを選択できないようにする
            style.interaction.selectable_labels = false;
        });

        // ダイアログを開く
        self.open_dialog();

        // ドラッグ&ドロップされたファイルを処理
        self.drop_files(ui);

        // パネルのスタイルを設定
        // 上部パネルを表示
        top::view(ui, &mut self.setting_token, &mut self.open_dialog_token);

        // 下部パネルを表示
        bottom::view(ui);

        // 中央パネルを表示
        egui::CentralPanel::default().show(ui, |ui| {
            ui.label("Gacho");
        });

        // 設定ウィンドウを表示
        if self.setting_token.open {
            setting_window::view(
                ui.ctx(),
                &mut self.app,
                &mut self.setting_token,
                &mut self.updated_token,
                &mut self.update_job,
            );
        }

        // エラーモーダルを表示
        if self.error_token.open {
            modal::error(
                ui.ctx(),
                &mut self.error_token,
            );
        }
    }
}

impl Render {
    /// ダイアログを開く
    fn open_dialog(&mut self) {
        // ファイルダイアログを開くボタンが押されてたらファイルダイアログを開く
        if self.open_dialog_token.file_dialog {
            self.open_dialog_token.file_dialog = false;
            if let Err(e) = open::file() {
                eprintln!("Error opening file: {}", e);
                self.error_token.open = true;
                self.error_token.value = Some(e);
            }
        }
    }

    /// ドラッグ&ドロップされたファイルを処理
    /// * `ui` - UI
    fn drop_files(&mut self, ui: &egui::Ui) {
        if let Err(e) = input::drop(ui) {
            eprintln!("Error dropping files: {}", e);
            self.error_token.open = true;
            self.error_token.value = Some(e);
        }
    }
}
