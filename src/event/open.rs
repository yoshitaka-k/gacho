use std::path::PathBuf;
use crate::{error, file, model};

/// ファイルオープンダイアログを開いて選択結果を追加する
/// * `label` - フィルターのラベル
/// * `open_files` - 開いているファイル
/// * `book_model` - 本のモデル
/// * `is_open_last_page` - 最後に読んだページを開くかどうか
/// * `return` - ファイルを開けたかどうか
pub(crate) fn file(
    label: &str,
    open_files: &mut file::OpenFiles,
    book_model: &mut model::Book,
    is_open_last_page: bool,
) -> error::Result<bool> {
    let extensions = file::Extension::to_archive_vec();

    // ファイルを選択
    let path = rfd::FileDialog::new()
        .add_filter(label, &extensions)
        .pick_file();

    // ファイルを追加
    open_path(path, open_files, book_model, is_open_last_page)
}

/// フォルダオープンダイアログを開いて選択結果を追加する
/// * `label` - フィルターのラベル
/// * `open_files` - 開いているファイル
/// * `book_model` - 本のモデル
/// * `is_open_last_page` - 最後に読んだページを開くかどうか
/// * `return` - ファイルを開けたかどうか
pub(crate) fn folder(
    label: &str,
    open_files: &mut file::OpenFiles,
    book_model: &mut model::Book,
    is_open_last_page: bool,
) -> error::Result<bool> {
    let extensions = file::Extension::to_archive_vec();

    // フォルダを選択
    let path = rfd::FileDialog::new()
        .add_filter(label, &extensions)
        .pick_folder();

    // ファイルを追加
    open_path(path, open_files, book_model, is_open_last_page)
}

/// ドロップされたファイルを処理
/// * `ctx` - コンテキスト
/// * `dropped_files` - ドロップされたファイル
/// * `open_files` - 開いているファイル
/// * `book_model` - 本のモデル
/// * `is_open_last_page` - 最後に読んだページを開くかどうか
/// * `return` - ファイルを開けたかどうか
pub(crate) fn drop(
    ctx: &egui::Context,
    dropped_files: Vec<PathBuf>,
    open_files: &mut file::OpenFiles,
    book_model: &mut model::Book,
    is_open_last_page: bool,
) -> error::Result<bool> {
    let Some(file) = dropped_files.first() else {
        return Ok(false);
    };

    let books = if is_open_last_page {
        book_model.get_all()?
    } else {
        Vec::new()
    };

    // ウィンドウを前面に
    ctx.send_viewport_cmd_to(
        egui::ViewportId::ROOT,
        egui::ViewportCommand::Focus,
    );

    // ファイルを追加
    open_files.build_book(file.to_path_buf(), books)
}

/// OS の「このアプリで開く」やコマンドライン引数のファイルを処理
/// * `path` - ファイルのパス
/// * `open_files` - 開いているファイル
pub(crate) fn launch(
    path: PathBuf,
    open_files: &mut file::OpenFiles,
    book_model: &mut model::Book,
    is_open_last_page: bool,
) -> error::Result<bool> {
    let books = if is_open_last_page {
        book_model.get_all()?
    } else {
        Vec::new()
    };

    // ファイルを追加
    open_files.build_book(path, books)
}

/// ファイルを追加
/// * `path` - ファイルのパス
/// * `open_files` - 開いているファイル
/// * `book_model` - 本のモデル
/// * `is_open_last_page` - 最後に読んだページを開くかどうか
/// * `return`: ファイルを開けたかどうか
fn open_path(
    path: Option<PathBuf>,
    open_files: &mut file::OpenFiles,
    book_model: &mut model::Book,
    is_open_last_page: bool,
) -> error::Result<bool> {
    if let Some(path) = path {
        let books = if is_open_last_page {
            book_model.get_all()?
        } else {
            Vec::<(PathBuf, usize)>::new()
        };

        // ファイルを追加
        return open_files.build_book(path, books);
    }

    Ok(false)
}
