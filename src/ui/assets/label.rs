use crate::app;

pub(crate) struct OpenDialog;
impl OpenDialog {
    /// アーカイブファイルのフィルターラベル
    /// * `return` - アーカイブファイルのフィルターラベル
    pub fn archive() -> &'static str {
        "Archive"
    }
}

/// エラーモーダルのラベル
pub(crate) struct ModalError;
impl ModalError {
    /// エラーモーダルの見出しラベル
    /// * `return` - エラーモーダルの見出しラベル
    pub fn heading() -> &'static str {
        "An error occurred"
    }
}

/// 更新モーダルのラベル
pub(crate) struct ModalUpdated;
impl ModalUpdated {
    /// アップデートがある場合の見出しラベル
    /// * `return` - アップデートがある場合の見出しラベル
    pub fn available() -> &'static str {
        "Update available."
    }

    /// アップデートが最新の場合の見出しラベル
    /// * `return` - アップデートが最新の場合の見出しラベル
    pub fn latest() -> &'static str {
        "Update not available."
    }

    /// アップデートが取得できなかった場合の見出しラベル
    /// * `return` - アップデートが取得できなかった場合の見出しラベル
    pub fn failed() -> &'static str {
        "Couldn't check for updates."
    }

    /// 新しいバージョンのラベル
    /// * `return` - 新しいバージョンのラベル
    pub fn new_version() -> &'static str {
        "New version: "
    }

    /// 現在のバージョンのラベル
    /// * `return` - 現在のバージョンのラベル
    pub fn current_version() -> &'static str {
        "Current version: v"
    }

    /// ダウンロードボタンのラベル
    /// * `return` - ダウンロードボタンのラベル
    pub fn download() -> &'static str {
        "Download"
    }
}

/// メイントップのラベル
pub(crate) struct MainTop;
impl MainTop {
    /// 設定ボタンのラベル
    /// * `return` - 設定ボタンのラベル
    pub fn settings() -> &'static str {
        "Settings"
    }

    /// 閉じるボタンのラベル
    /// * `return` - 閉じるボタンのラベル
    pub fn close() -> &'static str {
        "File Close"
    }

    /// フォルダダイアログを開くボタンのラベル
    /// * `return` - フォルダダイアログを開くボタンのラベル
    pub fn folder_open() -> &'static str {
        "Folder Open"
    }

    /// ファイルダイアログを開くボタンのラベル
    /// * `return` - ファイルダイアログを開くボタンのラベル
    pub fn files_open() -> &'static str {
        "Files Open"
    }
}

/// メインボトムのラベル
pub(crate) struct MainBottom;
impl MainBottom {
    /// 右端のページボタンのラベル
    /// * `read_from` - ページ送り方向
    /// * `return` - 右端のページボタンのラベル
    pub fn rightmost(read_from: &app::ReadFrom) -> &str {
        match read_from {
            app::ReadFrom::RightToLeft => "First page",
            app::ReadFrom::LeftToRight => "Last page",
        }
    }

    /// 右矢印のファイルボタンのラベル
    /// * `read_from` - ページ送り方向
    /// * `return` - 右矢印のファイルボタンのラベル
    pub fn right(read_from: &app::ReadFrom) -> &str {
        match read_from {
            app::ReadFrom::RightToLeft => "Previous page",
            app::ReadFrom::LeftToRight => "Next page",
        }
    }

    /// 左矢印のファイルボタンのラベル
    /// * `read_from` - ページ送り方向
    /// * `return` - 左矢印のファイルボタンのラベル
    pub fn left(read_from: &app::ReadFrom) -> &str {
        match read_from {
            app::ReadFrom::RightToLeft => "Next page",
            app::ReadFrom::LeftToRight => "Previous page",
        }
    }

    /// 左端のページボタンのラベル
    /// * `read_from` - ページ送り方向
    /// * `return` - 左端のページボタンのラベル
    pub fn leftmost(read_from: &app::ReadFrom) -> &str {
        match read_from {
            app::ReadFrom::RightToLeft => "Last page",
            app::ReadFrom::LeftToRight => "First page",
        }
    }
}

/// 設定のラベル
pub(crate) struct Setting;
impl Setting {
    pub fn heading() -> &'static str {
        "Settings"
    }

    /// 一般設定のラベル
    /// * `return` - 一般設定のラベル
    pub fn general() -> &'static str {
        "General"
    }

    /// アバウトのラベル
    /// * `return` - アバウトのラベル
    pub fn about() -> &'static str {
        "About"
    }

    /// アップデート確認ボタンのラベル
    /// * `return` - アップデート確認ボタンのラベル
    pub fn update_check() -> &'static str {
        "Check for updates"
    }
}

/// 設定のラベル
pub(crate) struct SettingGeneral;
impl SettingGeneral {
    /// 一般設定のヘッダーラベル
    /// * `return` - 一般設定のヘッダーラベル
    pub fn heading() -> &'static str {
        "General"
    }

    /// 画面表示の表示方式のラベル
    /// * `return` - 画面表示の表示方式のラベル
    pub fn page_layout() -> &'static str {
        "Page Layout"
    }

    /// 画面表示の表示方式（単一ページ）のラベル
    /// * `return` - 画面表示の表示方式（単一ページ）のラベル
    pub fn page_layout_single() -> &'static str {
        "Single Page"
    }

    /// 画面表示の表示方式（両ページ）のラベル
    /// * `return` - 画面表示の表示方式（両ページ）のラベル
    pub fn page_layout_spread() -> &'static str {
        "Two-Page Spread"
    }

    /// 表紙表示方式のラベル
    /// * `return` - 表紙表示方式のラベル
    pub fn cover_layout() -> &'static str {
        "Cover Layout"
    }

    /// 表紙表示方式（単一表紙）のラベル
    /// * `return` - 表紙表示方式（単一表紙）のラベル
    pub fn cover_layout_single() -> &'static str {
        "Single Cover"
    }

    /// 表紙表示方式（両表紙）のラベル
    /// * `return` - 表紙表示方式（両表紙）のラベル
    pub fn cover_layout_spread() -> &'static str {
        "No Single Cover"
    }

    /// ページ送り方向のラベル
    /// * `return` - ページ送り方向のラベル
    pub fn read_from() -> &'static str {
        "Read From"
    }

    /// ページ送り方向（右から左）のラベル
    /// * `return` - ページ送り方向（右から左）のラベル
    pub fn read_from_right_to_left() -> &'static str {
        "Right to Left"
    }

    /// ページ送り方向（左から右）のラベル
    /// * `return` - ページ送り方向（左から右）のラベル
    pub fn read_from_left_to_right() -> &'static str {
        "Left to Right"
    }

    /// 前処理数のラベル
    /// * `return` - 前処理数のラベル
    pub fn preloading() -> &'static str {
        "Preloading"
    }

    /// 最後に読んだページを保存するかどうかのラベル
    /// * `return` - 最後に読んだページを保存するかどうかのラベル
    pub fn remembered_last_page() -> &'static str {
        "Remembered Last Page"
    }

    /// 最後に読んだページを開くかどうかのラベル
    /// * `return` - 最後に読んだページを開くかどうかのラベル
    pub fn open_last_page() -> &'static str {
        "Open Last Page"
    }

    /// 画面表示の表示方式を変更した場合は本を再読み込みする必要がある
    /// * `return` - 画面表示の表示方式を変更した場合は本を再読み込みする必要がある
    pub fn layout_warning() -> &'static str {
        "Reopen the book file to apply the layout changes."
    }

    /// 最後に読んだページを開くかどうかを変更した場合は本を再読み込みする必要がある
    /// * `return` - 最後に読んだページを開くかどうかを変更した場合は本を再読み込みする必要がある
    pub fn reopen_book_warning() -> &'static str {
        "Reopen the book to resume from the saved page."
    }

    /// はいのラベル
    /// * `return` - はいのラベル
    pub fn yes() -> &'static str {
        "Yes"
    }

    /// いいえのラベル
    /// * `return` - いいえのラベル
    pub fn no() -> &'static str {
        "No"
    }
}

/// 設定のラベル
pub(crate) struct SettingAbout;
impl SettingAbout {
    /// アバウトのヘッダーラベル
    /// * `return` - アバウトのヘッダーラベル
    pub fn heading() -> &'static str {
        "About"
    }
}
