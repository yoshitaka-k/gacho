use crate::{config, lang};

/// メインボトムのラベル
pub(crate) struct PageLabel;
/// public methods
impl PageLabel {
    /// 右矢印のファイルのラベル
    /// * `read_from` - ページ送り方向
    /// * `return` - 右矢印のファイルのラベル
    pub fn right(
        read_from: &config::ReadFrom,
        is_current_pages: (bool, bool),
        is_current_books: (bool, bool),
    ) -> &str {
        let prev_label = Self::prev(is_current_pages, is_current_books);
        let next_label = Self::next(is_current_pages, is_current_books);

        match read_from {
            config::ReadFrom::RightToLeft => {
                prev_label
            },
            config::ReadFrom::LeftToRight => {
                next_label
            },
        }
    }

    /// 左矢印のファイルのラベル
    /// * `read_from` - ページ送り方向
    /// * `return` - 左矢印のファイルのラベル
    pub fn left(
        read_from: &config::ReadFrom,
        is_current_pages: (bool, bool),
        is_current_books: (bool, bool),
    ) -> &str {
        let prev_label = Self::prev(is_current_pages, is_current_books);
        let next_label = Self::next(is_current_pages, is_current_books);

        match read_from {
            config::ReadFrom::RightToLeft => {
                next_label
            },
            config::ReadFrom::LeftToRight => {
                prev_label
            },
        }
    }

    /// 右端のページのラベル
    /// * `read_from` - ページ送り方向
    /// * `return` - 右端のページのラベル
    pub fn rightmost(
        read_from: &config::ReadFrom,
        is_current_pages: (bool, bool),
        is_current_books: (bool, bool),
    ) -> &str {
        let first_label = Self::firstmost(is_current_pages, is_current_books);
        let last_label = Self::lastmost(is_current_pages, is_current_books);

        match read_from {
            config::ReadFrom::RightToLeft => {
                first_label
            },
            config::ReadFrom::LeftToRight => {
                last_label
            },
        }
    }

    /// 左端のページのラベル
    /// * `read_from` - ページ送り方向
    /// * `return` - 左端のページのラベル
    pub fn leftmost(
        read_from: &config::ReadFrom,
        is_current_pages: (bool, bool),
        is_current_books: (bool, bool),
    ) -> &str {
        let first_label = Self::firstmost(is_current_pages, is_current_books);
        let last_label = Self::lastmost(is_current_pages, is_current_books);

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

/// private methods
impl PageLabel {
    /// 前のページのラベル
    /// * `is_current_pages` - 現在のページの状態
    /// * `is_current_books` - 現在の本の状態
    /// * `return` - 前のページのラベル
    fn prev(
        is_current_pages: (bool, bool),
        is_current_books: (bool, bool),
    ) -> &'static str {
        let (is_first_page, _) = is_current_pages;
        let (is_prev_book, _) = is_current_books;

        if is_first_page && is_prev_book {
            lang::current().main_bottom.prev_page_prev_book
        } else {
            lang::current().main_bottom.prev_page
        }
    }

    /// 次のページのラベル
    /// * `is_current_pages` - 現在のページの状態
    /// * `is_current_books` - 現在の本の状態
    /// * `return` - 次のページのラベル
    fn next(
        is_current_pages: (bool, bool),
        is_current_books: (bool, bool),
    ) -> &'static str {
        let (_, is_last_page) = is_current_pages;
        let (_, is_next_book) = is_current_books;

        if is_last_page && is_next_book {
            lang::current().main_bottom.next_page_next_book
        } else {
            lang::current().main_bottom.next_page
        }
    }

    /// 最初のページのラベル
    /// * `is_current_pages` - 現在のページの状態
    /// * `is_current_books` - 現在の本の状態
    /// * `return` - 最初のページのラベル
    fn firstmost(
        is_current_pages: (bool, bool),
        is_current_books: (bool, bool),
    ) -> &'static str {
        let (is_first_page, _) = is_current_pages;
        let (is_prev_book, _) = is_current_books;

        if is_first_page && is_prev_book {
            lang::current().main_bottom.first_page_prev_book
        } else {
            lang::current().main_bottom.first_page
        }
    }

    /// 最後のページのラベル
    /// * `is_current_pages` - 現在のページの状態
    /// * `is_current_books` - 現在の本の状態
    /// * `return` - 最後のページのラベル
    fn lastmost(
        is_current_pages: (bool, bool),
        is_current_books: (bool, bool),
    ) -> &'static str {
        let (_, is_last_page) = is_current_pages;
        let (_, is_next_book) = is_current_books;

        if is_last_page && is_next_book {
            lang::current().main_bottom.last_page_next_book
        } else {
            lang::current().main_bottom.last_page
        }
    }
}
