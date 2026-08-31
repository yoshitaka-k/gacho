pub mod extension;
pub mod open_files;
pub mod image_file;

pub(crate) use extension::Extension;
pub(crate) use open_files::OpenFiles;
pub(crate) use image_file::ImageFile;

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
