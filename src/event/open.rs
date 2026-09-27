use std::path::PathBuf;
use crate::{error, file, model};

/// ファイルオープンダイアログを開いて選択結果を追加する
pub(crate) fn file(
    open_file: &mut file::OpenFile,
    book_model: &mut model::Book,
    is_open_last_page: bool,
) -> error::Result<bool> {
    let extensions = file::Extension::to_archive_vec();

    // ファイルを選択
    let path = rfd::FileDialog::new()
        .add_filter("Archive", &extensions)
        .pick_file();

    // ファイルを追加
    open_path(path, open_file, book_model, is_open_last_page)
}

/// ファイルオープンダイアログを開いて選択結果を追加する
/// * `files` - 開いているファイル
pub(crate) fn folder(
    open_file: &mut file::OpenFile,
    book_model: &mut model::Book,
    is_open_last_page: bool,
) -> error::Result<bool> {
    let extensions = file::Extension::to_archive_vec();

    // Macのみファイルとフォルダを同時選択できる
    #[cfg(target_os = "macos")]
    let path = rfd::FileDialog::new()
        .add_filter("Archive", &extensions)
        .pick_folder();

    // Mac以外はフォルダ選択のみ
    #[cfg(not(target_os = "macos"))]
    let path = rfd::FileDialog::new()
        .add_filter("Archive", &extensions)
        .pick_folder();

    // ファイルを追加
    open_path(path, open_file, book_model, is_open_last_page)
}

/// ファイルを追加
/// * `path` - ファイルのパス
/// * `open_file` - 開いているファイル
/// * `book_model` - 本のモデル
/// * `is_open_last_page` - 最後に読んだページを開くかどうか
/// * `return`: ファイルを開けたかどうか
fn open_path(
    path: Option<PathBuf>,
    open_file: &mut file::OpenFile,
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
        return open_file.build_book(path, books);
    }

    Ok(false)
}
