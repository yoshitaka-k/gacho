use std::{path, error, fmt};

pub type Result<T> = std::result::Result<T, GachoError>;

#[derive(Debug)]
pub enum GachoError {
    FileNotFound(String, path::PathBuf, String, u32),
    FileError(String, path::PathBuf, String, u32),
    ArchiveError(String, String, u32),
    InvalidVersion(String, u32),
    IndexError(usize, String, u32),
    DatabaseError(String, String, u32),
}

/// GachoError を表示
impl fmt::Display for GachoError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GachoError::FileNotFound(e, p, _, _) => write!(fmt, "File does not exist: {}\n\n{}", e, p.display()),
            GachoError::FileError(e, p, _, _) => write!(fmt, "File error: {}\n\n{}", e, p.display()),
            GachoError::ArchiveError(e, _, _) => write!(fmt, "Archive error: {}", e),
            GachoError::InvalidVersion(_, _) => write!(fmt, "Invalid version"),
            GachoError::IndexError(i, _, _) => write!(fmt, "Index error: {}", i),
            GachoError::DatabaseError(e, _, _) => write!(fmt, "Database error: {}", e),
        }
    }
}

/// GachoError を表示
impl error::Error for GachoError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            GachoError::FileNotFound(_, _, _, _) => None,
            GachoError::FileError(_, _, _, _) => None,
            GachoError::ArchiveError(_, _, _) => None,
            GachoError::InvalidVersion(_, _) => None,
            GachoError::IndexError(_, _, _) => None,
            GachoError::DatabaseError(_, _, _) => None,
        }
    }
}

/// GachoError のメソッド
impl GachoError {
    /// 呼び出し元のファイル名と行番号
    /// * `return` - 呼び出し元のファイル名と行番号
    #[track_caller]
    fn caller() -> (String, u32) {
        let location = std::panic::Location::caller();
        (location.file().to_string(), location.line())
    }

    /// ファイルが見つからない
    /// * `message` - エラーメッセージ
    /// * `path` - ファイルパス
    /// * `return` - ファイルが見つからないエラー
    #[track_caller]
    pub fn file_not_found(message: impl Into<String>, path: path::PathBuf) -> Self {
        let (file, line) = Self::caller();
        Self::FileNotFound(message.into(), path, file, line)
    }

    /// ファイルエラー
    /// * `message` - エラーメッセージ
    /// * `path` - ファイルパス
    /// * `return` - ファイルエラー
    #[track_caller]
    pub fn file_error(message: impl Into<String>, path: path::PathBuf) -> Self {
        let (file, line) = Self::caller();
        Self::FileError(message.into(), path, file, line)
    }

    /// アーカイブエラー
    /// * `message` - エラーメッセージ
    /// * `return` - アーカイブエラー
    #[track_caller]
    pub fn archive_error(message: impl Into<String>) -> Self {
        let (file, line) = Self::caller();
        Self::ArchiveError(message.into(), file, line)
    }

    /// バージョンが不正
    /// * `return` - バージョンが不正
    #[track_caller]
    pub fn invalid_version() -> Self {
        let (file, line) = Self::caller();
        Self::InvalidVersion(file, line)
    }

    /// インデックスエラー
    /// * `index` - インデックス
    /// * `return` - インデックスエラー
    #[track_caller]
    pub fn index_error(index: usize) -> Self {
        let (file, line) = Self::caller();
        Self::IndexError(index, file, line)
    }

    /// データベースエラー
    /// * `message` - エラーメッセージ
    /// * `return` - データベースエラー
    #[track_caller]
    pub fn database_error(message: impl Into<String>) -> Self {
        let (file, line) = Self::caller();
        Self::DatabaseError(message.into(), file, line)
    }

    /// 発生箇所のファイル名と行番号
    /// * `return` - ファイル名と行番号
    pub fn location(&self) -> (&str, u32) {
        match self {
            Self::FileNotFound(_, _, file, line)
            | Self::FileError(_, _, file, line)
            | Self::ArchiveError(_, file, line)
            | Self::InvalidVersion(file, line)
            | Self::IndexError(_, file, line)
            | Self::DatabaseError(_, file, line) => (file, *line),
        }
    }

    /// エラーコード
    /// エラー項目・ファイル名・行番号を組み合わせて作る
    /// * `return` - エラーコード
    pub fn code(&self) -> String {
        match self {
            Self::FileNotFound(_, _, f, l) => format!("FNF_{}_{}", Self::file_to_code(f), l),
            Self::FileError(_, _, f, l) => format!("FE_{}_{}", Self::file_to_code(f), l),
            Self::ArchiveError(_, f, l) => format!("AE_{}_{}", Self::file_to_code(f), l),
            Self::InvalidVersion(f, l) => format!("IVE_{}_{}", Self::file_to_code(f), l),
            Self::IndexError(_, f, l) => format!("IE_{}_{}", Self::file_to_code(f), l),
            Self::DatabaseError(_, f, l) => format!("DBE_{}_{}", Self::file_to_code(f), l),
        }
    }

    /// ファイルパスからコードを作る
    /// `src/` 以降を `/` と `_` で区切り、各語の頭文字をつなぐ
    /// 語が1つならその語をそのまま大文字にする。`mod` は除く
    /// * `file` - ファイルパス
    /// * `return` - コード
    fn file_to_code(file: &str) -> String {
        let normalized = file.replace('\\', "/");
        let relative = normalized
            .rsplit_once("src/")
            .map(|(_, rest)| rest)
            .unwrap_or(normalized.as_str());
        let words: Vec<&str> = relative
            .trim_end_matches(".rs")
            .split(['/', '_'])
            .filter(|word| !word.is_empty() && *word != "mod")
            .collect();

        match words.as_slice() {
            [] => "UNKNOWN".to_string(),
            [word] => word.to_uppercase(),
            words => words
                .iter()
                .filter_map(|word| word.chars().next())
                .flat_map(|ch| ch.to_uppercase())
                .collect(),
        }
    }
}

// tests ディレクトリ直下は別クレートになって、非公開関数が呼べないので、
// 子モジュールとして読むためのパスを指定する
#[cfg(test)]
#[path = "../tests/unit/error.rs"]
mod tests;
