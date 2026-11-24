use crate::{state, error};

/// 設定ウィンドウを表示するためのトークン
pub(crate) struct SettingToken {
    pub open: bool,
    pub pos: Option<egui::Pos2>,
    pub tab: state::SettingTab,
}

impl SettingToken {
    pub fn new() -> Self {
        Self {
            open: false,
            pos: None,
            tab: state::SettingTab::General,
        }
    }
}
/// エラーモーダルを表示するためのトークン
pub(crate) struct ErrorToken {
    pub open: bool,
    pub value: Option<error::GachoError>,
    pub dismissed: bool,
}

impl ErrorToken {
    pub fn new() -> Self {
        Self {
            open: false,
            value: None,
            dismissed: false,
        }
    }

    /// エラーモーダルを表示
    /// * `error` - エラー内容
    pub fn show(&mut self, error: error::GachoError) {
        if self.dismissed {
            return;
        }

        // 同じ失敗が毎フレーム来ても、ログは最初の1回だけ出す
        if self.value.is_none() {
            let (file, line) = error.location();
            log::error!("{error} @ {file}:{line}");
        }

        self.open = true;
        self.value = Some(error);
    }

    /// エラーモーダルを非表示
    pub fn dismiss(&mut self) {
        self.open = false;
        self.value = None;
        self.dismissed = true;
    }

    /// エラーモーダルをリセット
    pub fn reset(&mut self) {
        self.open = false;
        self.value = None;
        self.dismissed = false;
    }
}
