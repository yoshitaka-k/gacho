use getset::{Getters, MutGetters};
use serde::{Deserialize, Serialize};

/// 次の画像の表示数
const DEFAULT_PRELOADING: usize = 5;

/// 最小ページインデックス
pub(crate) const MIN_PAGE_INDEX: usize = 0;
/// 最大ページインデックス
pub(crate) const MAX_PAGE_INDEX: usize = 0;

/// 前処理数の最小値
pub(crate) const MIN_PRELOADING: usize = 0;
/// 前処理数の最大値
pub(crate) const MAX_PRELOADING: usize = 10;

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
    /// ページ送り方向
    read_from: ReadFrom,
    /// ページ送り表示方式
    page_layout: PageLayout,
    /// 表紙表示方式
    cover_layout: CoverLayout,

    /// 最後に読んだページを保存するかどうか
    remembered_last_page: bool,
    /// 最後に読んだページを開くかどうか
    #[getset(skip)]
    #[getset(get_mut = "pub")]
    open_last_page: bool,

    /// 前処理数
    preloading: usize,

    /// 言語
    language: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            read_from: ReadFrom::RightToLeft,
            page_layout: PageLayout::Single,
            cover_layout: CoverLayout::Single,
            remembered_last_page: false,
            open_last_page: false,
            preloading: DEFAULT_PRELOADING,
            language: "en".to_string(),
        }
    }
}

impl Config {
    /// 新しい設定を作成
    /// * `return` - 新しい設定
    pub fn new() -> Self {
        Self::default()
    }

    /// 最後に読んだページを開くかどうかを取得
    ///
    /// 最後に読んだページを保存していない場合は false を返す
    ///
    /// * `return` - 最後に読んだページを開くかどうか
    pub fn open_last_page(&self) -> &bool {
        if self.remembered_last_page {
            &self.open_last_page
        } else {
            &false
        }
    }

    /// 値をクランプする
    pub(crate) fn clamp_values(&mut self) {
        self.preloading = self.preloading.clamp(MIN_PRELOADING, MAX_PRELOADING);
    }
}
