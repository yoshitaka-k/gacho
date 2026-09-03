mod update;

pub(crate) use update::{UpdateCheck, UpdateJob, UpdatedToken};

use getset::{Getters, MutGetters};
use serde::{Deserialize, Serialize};

use crate::event;

/// GitHub リポジトリ URL
pub(crate) const GITHUB_URL: &str = "https://github.com/{repository}";

/// アップデート確認リクエスト URL
pub(crate) const REQUEST_URL: &str = "https://api.github.com/repos/{repository}/releases/latest";

/// 次の画像の表示数
const DEFAULT_PRELOADING: usize = 5;

/// アプリケーションの状態
#[derive(Deserialize, Serialize, Getters, MutGetters)]
#[serde(default)]
pub struct App {
    /// 前処理数
    #[getset(get = "pub", get_mut = "pub")]
    preloading: usize,

    /// ページ送り方向
    #[getset(get = "pub", get_mut = "pub")]
    read_from: event::ReadFrom,
}

impl Default for App {
    fn default() -> Self {
        Self {
            preloading: DEFAULT_PRELOADING,
            read_from: event::ReadFrom::RightToLeft,
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
