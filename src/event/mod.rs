pub(crate) mod input;
pub(crate) mod button;
pub(crate) mod open;
pub(crate) mod launch;

/// イベントアクション
pub(crate) enum EventAction {
    FilesOpen,
    FolderOpen,
    DropOpen(Vec<std::path::PathBuf>),
    Close,
    SettingOpen(Option<egui::Pos2>),
    SettingClose,
    Click(egui::Pos2),
    Left,
    Right,
    Leftmost,
    Rightmost,
    Slider(usize),
}
