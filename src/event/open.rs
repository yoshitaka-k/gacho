use crate::{error, file};

/// ファイルオープンダイアログを開いて選択結果を追加する
pub(crate) fn file(
    open_file: &mut file::OpenFile,
) -> error::Result<bool> {
    let extensions = file::Extension::to_archive_vec();

    // ファイルを選択
    let path = rfd::FileDialog::new()
        .add_filter("Archive", &extensions)
        .pick_file();

    // ファイルを追加
    if let Some(path) = path {
        // ファイルを追加
        return open_file.open_book(path);
    }

    Ok(false)
}

/// ファイルオープンダイアログを開いて選択結果を追加する
/// * `files` - 開いているファイル
pub(crate) fn folder(
    open_file: &mut file::OpenFile,
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
    if let Some(path) = path {
        // ファイルを追加
        return open_file.open_book(path);
    }

    Ok(false)
}
