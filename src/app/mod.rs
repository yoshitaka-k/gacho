mod update;

pub(crate) use update::{UpdateCheck, UpdateJob, UpdatedToken};

use getset::{Getters, MutGetters};
use serde::{Deserialize, Serialize};

/// GitHub リポジトリ URL
pub(crate) const GITHUB_URL: &str = "https://github.com/{repository}";

/// アップデート確認リクエスト URL
pub(crate) const REQUEST_URL: &str = "https://api.github.com/repos/{repository}/releases/latest";

/// 次の画像の表示数
const DEFAULT_PRELOADING: usize = 5;

/// 表紙表示方式
#[derive(Clone, Copy, Deserialize, Serialize, PartialEq)]
pub(crate) enum CoverLayout {
    Single,
    Spread,
}

impl CoverLayout {
    /// 文字列を取得
    /// * `return` - 文字列
    pub(crate) fn to_string(&self) -> &str {
        match self {
            CoverLayout::Single => "Single Cover",
            CoverLayout::Spread => "No Single Cover",
        }
    }
}

/// ページ送り方向
#[derive(Clone, Copy, Deserialize, Serialize, PartialEq)]
pub(crate) enum ReadFrom {
    RightToLeft,
    LeftToRight,
}

impl ReadFrom {
    /// 文字列を取得
    /// * `return` - 文字列
    pub(crate) fn to_string(&self) -> &str {
        match self {
            ReadFrom::RightToLeft => "Right to Left",
            ReadFrom::LeftToRight => "Left to Right",
        }
    }
}

/// ページ送り表示方式
#[derive(Clone, Copy, Deserialize, Serialize, PartialEq)]
pub(crate) enum PageLayout {
    Single,
    Spread,
}

impl PageLayout {
    /// 文字列を取得
    /// * `return` - 文字列
    pub(crate) fn to_string(&self) -> &str {
        match self {
            PageLayout::Single => "Single Page",
            PageLayout::Spread => "Two-Page Spread",
        }
    }
}

/// アプリケーションの状態
#[derive(Deserialize, Serialize, Getters, MutGetters)]
#[getset(get = "pub", get_mut = "pub")]
#[serde(default)]
pub struct App {
    /// 前処理数
    preloading: usize,

    /// 表紙表示方式
    cover_layout: CoverLayout,

    /// ページ送り方向
    read_from: ReadFrom,

    /// ページ送り表示方式
    page_layout: PageLayout,
}

impl Default for App {
    fn default() -> Self {
        Self {
            preloading: DEFAULT_PRELOADING,
            cover_layout: CoverLayout::Single,
            read_from: ReadFrom::RightToLeft,
            page_layout: PageLayout::Single,
        }
    }
}

impl App {
    /// 新しいアプリケーションを作成
    /// * `return` - 新しいアプリケーション
    pub fn new() -> Self {
        Self::default()
    }
}
