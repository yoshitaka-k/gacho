use crate::{app, event, file};
use crate::event::{open, input};
use crate::ui::{self, modal};
use crate::ui::assets::{fonts, svg};
use crate::ui::main::{top, bottom, middle};
use crate::ui::setting::view as setting_window;

/// 描画用のウィジェット
pub struct Render {
    app: app::App,
    open_files: file::OpenFiles,
    pending_actions: Vec<event::EventAction>,

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
        // フォントと SVG ローダーを追加
        fonts::install(&cc.egui_ctx);
        svg::install(&cc.egui_ctx);

        // 前回保存した App があれば復元（なければ引数の app を使う）
        let app = cc.storage
            .and_then(|storage| eframe::get_value(storage, eframe::APP_KEY))
            .unwrap_or(app);

        Self {
            app,
            open_files: file::OpenFiles::new(),
            pending_actions: Vec::new(),
            open_dialog_token: ui::OpenDialogToken {
                file_dialog: false,
                folder_dialog: false,
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

        // Command + O キーが押されたらファイルダイアログを開く
        input::command_open(ui, &mut self.open_dialog_token);

        // Command + Comma キーが押されたら設定ウィンドウを開く
        input::command_comma(ui, &mut self.setting_token);

        // キーイベントを処理
        input::arrow_left(ui, &mut self.pending_actions);
        input::arrow_right(ui, &mut self.pending_actions);

        // ファイルを開く
        self.open_dialog();
        self.drop_files(ui);

        // イベントアクションを処理
        self.process_actions(ui);

        // パネルのスタイルを設定
        // 上部パネルを表示
        top::view(ui, &mut self.open_files, &mut self.setting_token, &mut self.open_dialog_token);

        // 下部パネルを表示
        bottom::view(ui, &self.app, &mut self.open_files);

        // 中央パネルを表示
        middle::view(ui, &self.app, &mut self.open_files, &mut self.pending_actions, &mut self.error_token);

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

            // ファイルを開く
            if let Err(e) = open::file(&mut self.open_files) {
                eprintln!("Error opening file: {}", e);
                self.error_token.open = true;
                self.error_token.value = Some(e);
            }
        }

        // フォルダダイアログを開くボタンが押されてたらフォルダダイアログを開く
        if self.open_dialog_token.folder_dialog {
            self.open_dialog_token.folder_dialog = false;

            // フォルダを開く
            if let Err(e) = open::folder(&mut self.open_files) {
                eprintln!("Error opening folder: {}", e);
                self.error_token.open = true;
                self.error_token.value = Some(e);
            }
        }
    }

    /// ドラッグ&ドロップされたファイルを処理
    /// * `ui` - UI
    fn drop_files(&mut self, ui: &egui::Ui) {
        // ドラッグ&ドロップされたファイルを処理
        if let Err(e) = input::drop(ui, &mut self.open_files) {
            eprintln!("Error dropping files: {}", e);
            self.error_token.open = true;
            self.error_token.value = Some(e);
        }
    }

    /// イベントアクションを処理
    /// * `ui` - UI
    fn process_actions(&mut self, ui: &egui::Ui) {
        for action in self.pending_actions.drain(..) {
            match action {
                event::EventAction::Click(pos) => {
                    if pos.x < ui.max_rect().max.x / 2.0 {
                        self.open_files.next(&self.app);
                    } else {
                        self.open_files.prev(&self.app);
                    }
                }
                event::EventAction::Left => {
                    self.open_files.next(&self.app);
                }
                event::EventAction::Right => {
                    self.open_files.prev(&self.app);
                }
            }
        }
    }
}
