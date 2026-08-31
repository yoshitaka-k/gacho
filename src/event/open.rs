use crate::{error, file};

/// ファイルオープンダイアログを開いて選択結果を追加する
pub(crate) fn file(
    open_files: &mut file::OpenFiles,
) -> error::Result<()> {
    let extensions = file::Extension::to_archive_vec();

    let path = rfd::FileDialog::new()
        .add_filter("Archive", &extensions)
        .pick_file();

    // ファイルを追加
    if let Some(path) = path {
        open_files.add_path(path)?;
    }

    Ok(())
}

/// ファイルオープンダイアログを開いて選択結果を追加する
/// * `files` - 開いているファイル
pub(crate) fn folder(
    open_files: &mut file::OpenFiles,
) -> error::Result<()> {
    let extensions = file::Extension::to_archive_vec();

    // Macのみファイルとフォルダを同時選択できる
    #[cfg(target_os = "macos")]
    let path = rfd::FileDialog::new()
        .add_filter("Archive", &extensions)
        .pick_file_or_folder();

    // Mac以外はフォルダ選択のみ
    #[cfg(not(target_os = "macos"))]
    let path = rfd::FileDialog::new()
        .add_filter("Archive", &extensions)
        .pick_folder();

    // ファイルを追加
    if let Some(path) = path {
        open_files.add_path(path)?;
    }

    Ok(())
}
