use crate::{error, event, state, lang, model};
use crate::event::{open, button};

/// イベントを処理
/// * `ctx` - コンテキスト
/// * `state` - 状態
/// * `job` - ジョブ
/// * `book_model` - 本のモデル
/// * `action` - イベント
/// * `return` - 結果
pub(crate) fn handle(
    ctx: &egui::Context,
    state: &mut state::State,
    job: &mut state::Job,
    book_model: &mut model::Book,
    action: event::EventAction,
) -> error::Result<event::ActionResult> {
    match action {
        // ファイルダイアログを開くイベント
        event::EventAction::FilesOpen => {
            let result = open::file(
                lang::current().file_dialog.archive,
                &mut state.files,
                book_model,
                *state.config.open_last_page(),
            )?;
            // ファイルを開く結果を処理
            if result {
                Ok(event::ActionResult::OpenFileSuccess)
            } else {
                Ok(event::ActionResult::None)
            }
        }
        // フォルダダイアログを開くイベント
        event::EventAction::FolderOpen => {
            let result = open::folder(
                lang::current().file_dialog.archive,
                &mut state.files,
                book_model,
                *state.config.open_last_page(),
            )?;
            // ファイルを開く結果を処理
            if result {
                Ok(event::ActionResult::OpenFileSuccess)
            } else {
                Ok(event::ActionResult::None)
            }
        }
        // ドラッグ&ドロップされたファイルを処理
        event::EventAction::DropOpen(files) => {
            let result = open::drop(
                files,
                &mut state.files,
                book_model,
                *state.config.open_last_page(),
            )?;

            // ウィンドウを前面に
            ctx.send_viewport_cmd_to(
                egui::ViewportId::ROOT,
                egui::ViewportCommand::Focus,
            );

            // ファイルを開く結果を処理
            if result {
                Ok(event::ActionResult::OpenFileSuccess)
            } else {
                Ok(event::ActionResult::None)
            }
        }
        // 閉じるボタンイベント
        event::EventAction::Close => {
            // 開いている本を閉じる
            state.files.clear();
            Ok(event::ActionResult::None)
        }
        // 設定ウィンドウを開くイベント
        event::EventAction::SettingOpen(pos) => {
            button::setting_open(&mut state.setting_token, pos);
            Ok(event::ActionResult::None)
        }
        // 設定ウィンドウを閉じるイベント
        event::EventAction::SettingClose => {
            state.setting_token.open = false;
            Ok(event::ActionResult::None)
        }
        // 左矢印ボタンイベント
        event::EventAction::Left => {
            if let Err(e) = state.files.left_page(&state.config) {
                state.error_token.show(e);
            }
            // 最後に読んだページを保存
            Ok(event::ActionResult::SaveLastPage)
        }
        // 右矢印ボタンイベント
        event::EventAction::Right => {
            if let Err(e) = state.files.right_page(&state.config) {
                state.error_token.show(e);
            }
            // 最後に読んだページを保存
            Ok(event::ActionResult::SaveLastPage)
        }
        // 左端ボタンイベント
        event::EventAction::Leftmost => {
            if let Err(e) = state.files.leftmost_page(&state.config) {
                state.error_token.show(e);
            }
            // 最後に読んだページを保存
            Ok(event::ActionResult::SaveLastPage)
        }
        // 右端ボタンイベント
        event::EventAction::Rightmost => {
            if let Err(e) = state.files.rightmost_page(&state.config) {
                state.error_token.show(e);
            }
            // 最後に読んだページを保存
            Ok(event::ActionResult::SaveLastPage)
        }
        // スライダーイベント
        event::EventAction::Slider(index) => {
            // インデックスを更新
            state.files.set_current_spread(index);
            // 最後に読んだページを保存
            Ok(event::ActionResult::SaveLastPage)
        }
        // アップデート確認ボタンイベント
        event::EventAction::CheckForUpdate => {
            job.update_job.run(&mut state.update_token);
            Ok(event::ActionResult::None)
        }
    }
}
