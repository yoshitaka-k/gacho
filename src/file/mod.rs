pub mod extension;
pub mod open_files;
pub mod image;
pub mod archive;

pub(crate) use extension::Extension;
pub(crate) use open_files::OpenFiles;
pub(crate) use image::Image;
pub(crate) use archive::Archive;

use std::path::{Path, PathBuf};
use std::sync::LazyLock;
use regex::Regex;

/// 同人形式: (カテゴリ)[著者]タイトル(ジャンル)[備考]
/// 例:
/// - `(カテゴリ)[著者]タイトル(ジャンル)`
/// - `(カテゴリ)[著者]タイトル(ジャンル)[備考]`
/// - `(カテゴリ)[著者(別名)]タイトル(ジャンル)`
/// - `(カテゴリ)[著者(別名)]タイトル(ジャンル)[備考]`
pub(crate) static RE_DOJIN_NAME_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^\((?P<category>[^)]+)\)\s*\[(?P<author>[^\]]+)\]\s*(?P<title>.+)\((?P<genre>[^)]+)\)(?P<notes>(?:\s*\[[^\]]+\])*)\s*$").unwrap()
});

/// 簡易形式: [著者]タイトル
/// 例:
/// - `[著者]タイトル`
/// - `[著者(別名)]タイトル`
/// - `(カテゴリ)[著者]タイトル`
/// - `(カテゴリ)[著者(別名)]タイトル`
pub(crate) static RE_BOOK_NAME_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?:\((?P<category>[^)]+)\)\s*)?\[(?P<author>[^\]]+)\]\s*(?P<title>.+)\s*$").unwrap()
});

/// ファイル名から本のタイトルを取り出す。
/// 同人形式を先に試し、一致しなければ簡易形式を試す。
pub(crate) fn extract_book_title(name: &str) -> String {
    for re in [&*RE_DOJIN_NAME_PATTERN, &*RE_BOOK_NAME_PATTERN] {
        if let Some(title) = re.captures(name).and_then(|c| c.name("title")) {
            return title.as_str().trim().to_string();
        }
    }
    name.trim().to_string()
}

/// ファイルの拡張子が許可されているかどうか
/// * `path` - ファイルのパス
/// * `return` - ファイルの拡張子が許可されているかどうか
pub(crate) fn is_allowed_extension(path: &PathBuf) -> bool {
    let Some(ext) = path.extension() else {
        return false;
    };

    let Some(ext) = ext.to_str() else {
        return false;
    };

    self::extension::Extension::to_vec().iter().any(|e| e.eq_ignore_ascii_case(ext))
}

/// ファイルが画像かどうかを判断
/// * `path` - ファイルのパス
/// * `return` - ファイルが画像かどうか
pub(crate) fn is_image(path: &PathBuf) -> bool {
    let Some(ext) = path.extension() else {
        return false;
    };

    let Some(ext) = ext.to_str() else {
        return false;
    };

    self::extension::Extension::to_image_vec().iter().any(|e| e.eq_ignore_ascii_case(ext))
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

/// ファイルが隠しファイルかどうかをチェックする
/// * `path` - ファイルのパス
/// * `return` - ファイルが隠しファイルかどうか
pub(crate) fn is_hidden_entry(path: &Path) -> bool {
    path.components().any(|c| {
        let s = c.as_os_str().to_string_lossy();
        s.starts_with('.') || s == "__MACOSX"
    })
}
