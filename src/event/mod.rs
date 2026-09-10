use serde::{Deserialize, Serialize};

pub(crate) mod input;
pub(crate) mod button;
pub(crate) mod open;
pub(crate) mod drop;
pub(crate) mod launch;

/// イベントアクション
pub(crate) enum EventAction {
    Click(egui::Pos2),
    Left,
    Right,
}

/// ページ送り方向
#[derive(Deserialize, Serialize, PartialEq)]
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
#[derive(Deserialize, Serialize, PartialEq)]
pub(crate) enum PageLayout {
    Default,
    Single,
    Spread,
}

impl PageLayout {
    /// 文字列を取得
    /// * `return` - 文字列
    pub(crate) fn to_string(&self) -> &str {
        match self {
            PageLayout::Default => "Default",
            PageLayout::Single => "Single Page",
            PageLayout::Spread => "Two-Page Spread",
        }
    }

    /// オフセットを取得
    /// * `return` - オフセット
    pub(crate) fn to_offset(&self) -> usize {
        match self {
            PageLayout::Default => 0,
            PageLayout::Single => 0,
            PageLayout::Spread => 1,
        }
    }
}
