mod update;

pub(crate) use update::{UpdateCheck, UpdateJob, UpdatedToken};

use serde::{Deserialize, Serialize};

/// GitHub リポジトリ URL
pub(crate) const GITHUB_URL: &str = "https://github.com/{repository}";

/// アップデート確認リクエスト URL
pub(crate) const REQUEST_URL: &str = "https://api.github.com/repos/{repository}/releases/latest";

/// アプリケーションの状態
#[derive(Serialize, Deserialize)]
pub struct App;

impl App {
    /// 新しいアプリケーションを作成
    /// * `return` - 新しいアプリケーション
    pub fn new() -> Self {
        Self {}
    }
}
