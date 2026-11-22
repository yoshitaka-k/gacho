use getset::{Getters, MutGetters};
use serde::{Deserialize, Serialize};

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

/// アプリケーションの設定
#[derive(Deserialize, Serialize, Getters, MutGetters)]
#[getset(get = "pub", get_mut = "pub")]
#[serde(default)]
pub struct Config {
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

impl Default for Config {
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

impl Config {
    /// 新しい設定を作成
    /// * `return` - 新しい設定
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
