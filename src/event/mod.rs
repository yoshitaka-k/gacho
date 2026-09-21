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
    Close,
}

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
