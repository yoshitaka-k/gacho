use crate::{error, file};

/// ファイルオープンダイアログを開いて選択結果を追加する
pub(crate) fn file(
) -> error::Result<()> {
    let extensions = file::Extension::to_vec();

    let paths = rfd::FileDialog::new()
        .add_filter("Archive", &extensions)
        .pick_file();

    // ファイルを追加
    if let Some(paths) = paths {
        println!("paths: {:?}", paths);
    }

    Ok(())
}
