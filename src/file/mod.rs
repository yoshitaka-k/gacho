pub mod extension;
pub mod open_files;
pub mod image;
pub mod archive;

pub(crate) use extension::Extension;
pub(crate) use open_files::OpenFiles;
pub(crate) use image::Image;
pub(crate) use archive::Archive;

use std::path::PathBuf;

/// ファイルの拡張子が許可されているかどうか
/// * `path` - ファイルのパス
/// * `return` - ファイルの拡張子が許可されているかどうか
pub(crate) fn is_allowed_extension(path: &PathBuf) -> bool {
    let Some(ext) = path.extension() else {
        eprintln!("File extension not found: {:?}", path);
        return false;
    };

    let Some(ext) = ext.to_str() else {
        eprintln!("File extension not found: {:?}", path);
        return false;
    };

    self::extension::Extension::to_vec().iter().any(|e| e.eq_ignore_ascii_case(ext))
}

/// ファイルがアーカイブかどうかを判断
/// * `path` - ファイルのパス
/// * `return` - ファイルがアーカイブかどうか
pub(crate) fn is_archive(path: &PathBuf) -> bool {
    let Some(ext) = path.extension() else {
        return false;
    };

    let Some(ext) = ext.to_str() else {
        return false;
    };

    self::extension::Extension::to_archive_vec().iter().any(|e| e.eq_ignore_ascii_case(ext))
}
