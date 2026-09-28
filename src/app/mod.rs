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

/// ページ送り方向
#[derive(Clone, Copy, Deserialize, Serialize, PartialEq)]
pub(crate) enum ReadFrom {
    RightToLeft,
    LeftToRight,
}

/// ページ送り表示方式
#[derive(Clone, Copy, Deserialize, Serialize, PartialEq)]
pub(crate) enum PageLayout {
    Single,
    Spread,
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

    /// 最後に読んだページを保存するかどうか
    #[getset(skip)]
    #[getset(get_mut = "pub")]
    remembered_last_page: bool,

    /// 最後に読んだページを開くかどうか
    #[getset(skip)]
    #[getset(get_mut = "pub")]
    open_last_page: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            preloading: DEFAULT_PRELOADING,
            cover_layout: CoverLayout::Single,
            read_from: ReadFrom::RightToLeft,
            page_layout: PageLayout::Single,
            remembered_last_page: false,
            open_last_page: false,
        }
    }
}

impl App {
    /// 新しいアプリケーションを作成
    /// * `return` - 新しいアプリケーション
    pub fn new() -> Self {
        Self::default()
    }

    /// 最後に読んだページを保存するかどうかを取得
    /// * `return` - 最後に読んだページを保存するかどうか
    pub fn remembered_last_page(&self) -> &bool {
        &self.remembered_last_page
    }

    /// 最後に読んだページを開くかどうかを取得
    /// 最後に読んだページを保存していない場合は false を返す
    /// * `return` - 最後に読んだページを開くかどうか
    pub fn open_last_page(&self) -> &bool {
        if self.remembered_last_page {
            &self.open_last_page
        } else {
            &false
        }
    }
}
