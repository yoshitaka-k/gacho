use crate::{config, lang};

/// メインボトムのラベル
pub(crate) struct MainBottom;
impl MainBottom {
    /// 最初のページボタンのラベル
    /// * `is_current_pages` - 現在のページの状態
    /// * `is_current_books` - 現在の本の状態
    /// * `return` - 最初のページボタンのラベル
    fn firstmost_label(is_current_pages: (bool, bool), is_current_books: (bool, bool)) -> &'static str {
        let (is_first_page, _) = is_current_pages;
        let (is_prev_book, _) = is_current_books;

        if is_first_page && is_prev_book {
            lang::current().main_bottom.first_page_prev_book
        } else {
            lang::current().main_bottom.first_page
        }
    }

    /// 最後のページボタンのラベル
    /// * `is_current_pages` - 現在のページの状態
    /// * `is_current_books` - 現在の本の状態
    /// * `return` - 最後のページボタンのラベル
    fn lastmost_label(is_current_pages: (bool, bool), is_current_books: (bool, bool)) -> &'static str {
        let (_, is_last_page) = is_current_pages;
        let (_, is_next_book) = is_current_books;

        if is_last_page && is_next_book {
            lang::current().main_bottom.last_page_next_book
        } else {
            lang::current().main_bottom.last_page
        }
    }

    fn prev_label(is_current_pages: (bool, bool), is_current_books: (bool, bool)) -> &'static str {
        let (is_first_page, _) = is_current_pages;
        let (is_prev_book, _) = is_current_books;

        if is_first_page && is_prev_book {
            lang::current().main_bottom.prev_page_prev_book
        } else {
            lang::current().main_bottom.prev_page
        }
    }

    fn next_label(is_current_pages: (bool, bool), is_current_books: (bool, bool)) -> &'static str {
        let (_, is_last_page) = is_current_pages;
        let (_, is_next_book) = is_current_books;

        if is_last_page && is_next_book {
            lang::current().main_bottom.next_page_next_book
        } else {
            lang::current().main_bottom.next_page
        }
    }

    /// 右端のページボタンのラベル
    /// * `read_from` - ページ送り方向
    /// * `return` - 右端のページボタンのラベル
    pub fn rightmost(read_from: &config::ReadFrom, is_current_pages: (bool, bool), is_current_books: (bool, bool)) -> &str {
        let first_label = Self::firstmost_label(is_current_pages, is_current_books);
        let last_label = Self::lastmost_label(is_current_pages, is_current_books);

        match read_from {
            config::ReadFrom::RightToLeft => {
                first_label
            },
            config::ReadFrom::LeftToRight => {
                last_label
            },
        }
    }

    /// 右矢印のファイルボタンのラベル
    /// * `read_from` - ページ送り方向
    /// * `return` - 右矢印のファイルボタンのラベル
    pub fn right(read_from: &config::ReadFrom, is_current_pages: (bool, bool), is_current_books: (bool, bool)) -> &str {
        let prev_label = Self::prev_label(is_current_pages, is_current_books);
        let next_label = Self::next_label(is_current_pages, is_current_books);

        match read_from {
            config::ReadFrom::RightToLeft => {
                prev_label
            },
            config::ReadFrom::LeftToRight => {
                next_label
            },
        }
    }

    /// 左矢印のファイルボタンのラベル
    /// * `read_from` - ページ送り方向
    /// * `return` - 左矢印のファイルボタンのラベル
    pub fn left(read_from: &config::ReadFrom, is_current_pages: (bool, bool), is_current_books: (bool, bool)) -> &str {
        let prev_label = Self::prev_label(is_current_pages, is_current_books);
        let next_label = Self::next_label(is_current_pages, is_current_books);

        match read_from {
            config::ReadFrom::RightToLeft => {
                next_label
            },
            config::ReadFrom::LeftToRight => {
                prev_label
            },
        }
    }

    /// 左端のページボタンのラベル
    /// * `read_from` - ページ送り方向
    /// * `return` - 左端のページボタンのラベル
    pub fn leftmost(read_from: &config::ReadFrom, is_current_pages: (bool, bool), is_current_books: (bool, bool)) -> &str {
        let first_label = Self::firstmost_label(is_current_pages, is_current_books);
        let last_label = Self::lastmost_label(is_current_pages, is_current_books);

        match read_from {
            config::ReadFrom::RightToLeft => {
                last_label
            },
            config::ReadFrom::LeftToRight => {
                first_label
            },
        }
    }
}
