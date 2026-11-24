use crate::{state, config, error, lang, event, file, ui, model, update};
use crate::event::{button, open, input};

/// 描画用のウィジェット
pub struct Render {
    /// アプリケーションの状態
    state: state::State,

    /// ジョブ
    job: state::Job,

    /// イベントアクション
    pending_actions: Vec<event::EventAction>,

    /// 本のモデル
    /// 最後に読んだページ用のモデルを追加
    book_model: model::Book,
}

/// public methods
impl Render {
    pub fn new(cc: &eframe::CreationContext<'_>, config: config::Config) -> Self {
        // フォントと SVG を追加
        ui::install(&cc.egui_ctx);

        // スタイルを設定
        cc.egui_ctx.global_style_mut(|style| {
            // ラベルを選択できないようにする
            style.interaction.selectable_labels = false;
        });

        // イベントコンテキストを設定
        event::launch::set_ctx(cc.egui_ctx.clone());

        // 前回保存した Config があれば復元（なければ引数の config を使う）
        let config = cc.storage
            .and_then(|storage| eframe::get_value(storage, eframe::APP_KEY))
            .unwrap_or(config);

        // 言語を設定
        lang::set(config.language());

        // 本のモデルを作成
        let book_model = model::Book::new();

        // アプリケーションの状態を作成
        let state = state::State {
            config,
            files: file::OpenFile::new(),
            setting_token: state::SettingToken::new(),
            update_token: update::UpdateToken::new(),
            error_token: state::ErrorToken::new(),
        };

        // ジョブを作成
        let job = state::Job {
            update_job: update::UpdateJob::new(cc.egui_ctx.clone()),
        };

        Self {
            state,
            job,
            pending_actions: Vec::new(),
            book_model,
        }
    }
}

/// eframe::App の実装
impl eframe::App for Render {
    /// 終了前に Config の状態を保存
    /// * `storage` - ストレージ
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.state.config);
    }

    /// 描画
    /// * `ui` - UI
    /// * `frame` - フレーム
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // 更新結果を取得
        self.job.update_job.result(&mut self.state.update_token);

        // OS から渡されたファイルを処理
        self.open_from_os();

        // 入力イベントを処理
        self.input_events(ui);

        // イベントアクションを処理
        self.process_pending_actions(ui);

        // メインウィンドウを描画
        ui::draw(
            ui,
            &mut self.state,
            &mut self.pending_actions,
        );
    }
}

/// private methods
impl Render {
    /// 入力イベントを処理
    /// * `ui` - UI
    fn input_events(&mut self, ui: &mut egui::Ui) {
        // ドラッグ&ドロップされたファイルを処理
        input::drop(ui, &mut self.pending_actions);

        // Command + O キーが押されたらファイルダイアログを開く
        input::command_open(ui, &mut self.pending_actions);

        // Command + Comma キーが押されたら設定ウィンドウを開く
        input::command_comma(ui, &mut self.pending_actions);

        // Command + W キーが押されたら開いている本を閉じる
        input::command_w(ui, &mut self.pending_actions);

        // キーイベントを処理
        input::arrow_left(ui, &mut self.pending_actions);
        input::arrow_right(ui, &mut self.pending_actions);
    }

    /// OS の「このアプリで開く」やコマンドライン引数のファイルを処理
    fn open_from_os(&mut self) {
        let Some(path) = event::launch::take_path() else {
            return;
        };

        // ファイルを開く
        let result = event::open::launch(
            path,
            &mut self.state.files,
            &mut self.book_model,
            *self.state.config.open_last_page(),
        );
        self.open_result(result);
    }

    /// ファイルを開く結果を処理
    /// * `result` - ファイルを開く結果
    fn open_result(&mut self, result: error::Result<bool>) {
        // エラーモーダルをリセット
        self.state.error_token.reset();

        // ファイルを開く結果を処理
        match result {
            Ok(true) => {
                // 最後に読んだページを保存
                self.save_last_page();

                // 画面に表示させるページリストを作成
                self.rebuild_spreads();
            }
            Ok(false) => {},
            Err(e) => self.state.error_token.show(e),
        }
    }

    /// 画面に表示させるページリストを作成
    fn rebuild_spreads(&mut self) {
        self.state.files.build_spreads(*self.state.config.cover_layout(), *self.state.config.page_layout());
    }

    /// イベントアクションを処理
    /// * `ui` - UI
    fn process_pending_actions(&mut self, ui: &mut egui::Ui) {
        // イベントアクションがない場合は何もしない
        if self.pending_actions.is_empty() { return; }

        // エラーモーダルをリセット
        self.state.error_token.reset();

        // 最後に読んだページを保存するかどうか
        let mut is_saved = false;

        // 処理予約されたアクションを取り出して空にする
        let mut pending_actions = std::mem::take(&mut self.pending_actions);

        // 取り出されたアクションを処理
        for action in pending_actions.drain(..) {
            match action {
                // ファイルダイアログを開くイベント
                event::EventAction::FilesOpen => {
                    let result = open::file(
                        lang::current().file_dialog.archive,
                        &mut self.state.files,
                        &mut self.book_model,
                        *self.state.config.open_last_page(),
                    );
                    self.open_result(result);
                }
                // フォルダダイアログを開くイベント
                event::EventAction::FolderOpen => {
                    let result = open::folder(
                        lang::current().file_dialog.archive,
                        &mut self.state.files,
                        &mut self.book_model,
                        *self.state.config.open_last_page(),
                    );
                    self.open_result(result);
                }
                // ドラッグ&ドロップされたファイルを処理
                event::EventAction::DropOpen(files) => {
                    let result = open::drop(
                        ui.ctx(),
                        files,
                        &mut self.state.files,
                        &mut self.book_model,
                        *self.state.config.open_last_page(),
                    );

                    self.open_result(result);
                }
                // 閉じるボタンイベント
                event::EventAction::Close => {
                    // 開いている本を閉じる
                    button::close_open_file(&mut self.state.files);
                }
                // 設定ウィンドウを開くイベント
                event::EventAction::SettingOpen(pos) => {
                    button::setting_open(&mut self.state.setting_token, pos);
                }
                // 設定ウィンドウを閉じるイベント
                event::EventAction::SettingClose => {
                    self.state.setting_token.open = false;
                }
                // クリックイベント
                event::EventAction::Click(pos) => {
                    // クリックした位置が左半分の場合は次のファイルを表示
                    if pos.x < ui.max_rect().max.x / 2.0 {
                        if let Err(e) = self.state.files.left_page(&self.state.config) {
                            self.state.error_token.show(e);
                        }
                    } else {
                        if let Err(e) = self.state.files.right_page(&self.state.config) {
                            self.state.error_token.show(e);
                        }
                    }
                    is_saved = true;
                }
                // 左矢印ボタンイベント
                event::EventAction::Left => {
                    if let Err(e) = self.state.files.left_page(&self.state.config) {
                        self.state.error_token.show(e);
                    }
                    is_saved = true;
                }
                // 右矢印ボタンイベント
                event::EventAction::Right => {
                    if let Err(e) = self.state.files.right_page(&self.state.config) {
                        self.state.error_token.show(e);
                    }
                    is_saved = true;
                }
                // 左端ボタンイベント
                event::EventAction::Leftmost => {
                    if let Err(e) = self.state.files.leftmost_page(&self.state.config) {
                        self.state.error_token.show(e);
                    }
                    is_saved = true;
                }
                // 右端ボタンイベント
                event::EventAction::Rightmost => {
                    if let Err(e) = self.state.files.rightmost_page(&self.state.config) {
                        self.state.error_token.show(e);
                    }
                    is_saved = true;
                }
                // スライダーイベント
                event::EventAction::Slider(index) => {
                    // インデックスを更新
                    self.state.files.set_current_spread(index);

                    is_saved = true;
                }
                // アップデート確認ボタンイベント
                event::EventAction::CheckForUpdate => {
                    button::check_for_update(&mut self.job.update_job, &mut self.state.update_token);
                }
            }

            // 再描画を要求
            ui.ctx().request_repaint();
        }

        // 最後に読んだページを保存
        if is_saved {
            self.save_last_page();
        }
    }

    /// sqlite に最後に読んだページを保存
    fn save_last_page(&mut self) {
        if !*self.state.config.remembered_last_page() {
            return;
        }

        let path = self.state.files.book_path();
        let page = self.state.files.current_page();
        if let Err(e) = self.book_model.save_last_page(&path, page) {
            self.state.error_token.show(e);
        }
    }
}
