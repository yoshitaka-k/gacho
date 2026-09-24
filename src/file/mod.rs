pub mod extension;
pub mod open_file;
pub mod book;
pub mod image;
pub mod archive;
pub mod library;

pub(crate) use extension::Extension;
pub(crate) use open_file::OpenFile;
pub(crate) use book::Book;
pub(crate) use image::Image;
pub(crate) use archive::Archive;
pub(crate) use library::Library;

use std::path::PathBuf;
use std::sync::LazyLock;
use regex::Regex;
use std::iter::Peekable;
use std::str::Chars;

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
pub(crate) fn is_hidden_entry(path: &PathBuf) -> bool {
    path.components().any(|c| {
        let s = c.as_os_str().to_string_lossy();
        s.starts_with('.') || s == "__MACOSX"
    })
}

/// パスをソートする
/// * `path` - パスのベクター
/// * `return` - ソートされたパスのベクター
pub(crate) fn sort_path(paths: &Vec<PathBuf>) -> Vec<PathBuf> {
    let mut temp_paths = paths.clone();
    // パスで並び替え
    temp_paths.sort_by(|a, b| {
        cmp_natural(&a.to_string_lossy(), &b.to_string_lossy()).then(a.cmp(b))
    });
    temp_paths
}

/// 自然順で比較
/// * `a` - 比較する文字列
/// * `b` - 比較する文字列
/// * `return` - 比較結果
pub(crate) fn cmp_natural(a: &str, b: &str) -> std::cmp::Ordering {
    let mut a = a.chars().peekable();
    let mut b = b.chars().peekable();

    loop {
        match (a.peek().copied(), b.peek().copied()) {
            (None, None) => return std::cmp::Ordering::Equal,
            (None, Some(_)) => return std::cmp::Ordering::Less,
            (Some(_), None) => return std::cmp::Ordering::Greater,
            (Some(ac), Some(bc)) if ac.is_ascii_digit() && bc.is_ascii_digit() => {
                let cmp = read_number(&mut a).cmp(&read_number(&mut b));
                if cmp != std::cmp::Ordering::Equal {
                    return cmp;
                }
            }
            _ => {
                let cmp = read_text(&mut a).cmp(&read_text(&mut b));
                if cmp != std::cmp::Ordering::Equal {
                    return cmp;
                }
            }
        }
    }
}

/// 数字を読み込む
/// * `chars` - 文字列
/// * `return` - 読み込んだ数字
fn read_number(chars: &mut Peekable<Chars<'_>>) -> u64 {
    let mut num = 0;
    while let Some(c) = chars.peek() {
        if !c.is_ascii_digit() {
            break;
        }
        num = num * 10 + (c.to_digit(10).unwrap() as u64);
        chars.next();
    }
    num
}

/// テキストを読み込む
/// * `chars` - 文字列
/// * `return` - 読み込んだテキスト
fn read_text(chars: &mut Peekable<Chars<'_>>) -> String {
    let mut text = String::new();
    while let Some(c) = chars.peek() {
        if c.is_ascii_digit() {
            break;
        }
        text.push(*c);
        chars.next();
    }
    text
}
