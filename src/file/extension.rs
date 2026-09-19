use std::ffi::OsStr;

#[derive(Clone)]
pub enum Extension {
    Jpg,
    Jpeg,
    Png,
    Bmp,
    Gif,
    Webp,
    Avif,
    Zip,
    Cbz,
    None,
}

impl Extension {
    /// 画像ファイルのベクタに変換
    /// * `return` - 画像ファイルのベクタ
    pub fn to_image_vec() -> Vec<&'static str> {
        vec![
            Self::Jpg.to_str(),
            Self::Jpeg.to_str(),
            Self::Png.to_str(),
            Self::Bmp.to_str(),
            Self::Gif.to_str(),
            Self::Webp.to_str(),
            Self::Avif.to_str(),
        ]
    }

    /// アーカイブファイルのベクタに変換
    /// * `return` - アーカイブファイルのベクタ
    pub fn to_archive_vec() -> Vec<&'static str> {
        vec![
            Self::Zip.to_str(),
            Self::Cbz.to_str(),
        ]
    }

    /// 文字列から Extension を作成
    /// * `extension` - 文字列
    /// * `return` - Extension
    pub fn from_str(extension: &OsStr) -> Self {
        match extension.to_ascii_lowercase().to_string_lossy().as_ref() {
            "jpg" => Self::Jpg,
            "jpeg" => Self::Jpeg,
            "png" => Self::Png,
            "bmp" => Self::Bmp,
            "gif" => Self::Gif,
            "webp" => Self::Webp,
            "avif" => Self::Avif,
            "zip" => Self::Zip,
            "cbz" => Self::Cbz,
            _ => Self::None,
        }
    }

    /// Extension を文字列に変換
    /// * `return` - 文字列
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Jpg => "jpg",
            Self::Jpeg => "jpeg",
            Self::Png => "png",
            Self::Bmp => "bmp",
            Self::Gif => "gif",
            Self::Webp => "webp",
            Self::Avif => "avif",
            Self::Zip => "zip",
            Self::Cbz => "cbz",
            Self::None => "",
        }
    }
}
