mod handler;
pub(crate) mod input;
pub(crate) mod button;
pub(crate) mod open;
pub(crate) mod launch;

pub(crate) use handler::handle;

/// イベントアクション
pub(crate) enum EventAction {
    FilesOpen,
    FolderOpen,
    DropOpen(Vec<std::path::PathBuf>),
    Close,
    SettingOpen(Option<egui::Pos2>),
    SettingClose,
    Left,
    Right,
    Leftmost,
    Rightmost,
    Slider(usize),
    CheckForUpdate,
}

/// イベントアクションの結果
pub(crate) enum ActionResult {
    None,
    OpenFileSuccess,
    SaveLastPage,
}
