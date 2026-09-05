use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};

#[cfg(target_os = "macos")]
#[path = "macos_open.rs"]
mod macos_open;

/// `eframe::run_native` の前に呼ぶ。
pub fn install() {
    #[cfg(target_os = "macos")]
    macos_open::install();
}

/// eframe の生成時に呼ぶ。
pub fn set_ctx(ctx: egui::Context) {
    #[cfg(target_os = "macos")]
    macos_open::set_ctx(ctx);

    #[cfg(not(target_os = "macos"))]
    let _ = ctx;
}

/// OS から渡されたファイルパスを 1 つ取り出す。
/// コマンドライン引数は一度だけ、macOS の「開く」は都度キューから読む。
pub fn take_path() -> Option<PathBuf> {
    let mut paths = Vec::new();

    static ARGV_TAKEN: AtomicBool = AtomicBool::new(false);
    if !ARGV_TAKEN.swap(true, Ordering::Relaxed) {
        paths.extend(argv_paths());
    }

    #[cfg(target_os = "macos")]
    paths.extend(macos_open::take_opened_files());

    paths.into_iter().next()
}

fn argv_paths() -> Vec<PathBuf> {
    std::env::args_os()
        .skip(1)
        .filter(|arg| {
            let s = arg.to_string_lossy();
            !s.starts_with('-')
        })
        .map(PathBuf::from)
        .collect()
}
