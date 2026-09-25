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
