use std::path::PathBuf;
use getset::{Getters, MutGetters, Setters};

use crate::{app, event, file, error};

const DEFAULT_PAGE: usize = 0;

/// 画面に表示するページの種類
#[derive(Clone, Copy)]
pub enum Spread {
    /// 単一ページ
    Single { index: usize },
    /// 見開きページ
    Pair { left: usize, right: usize },
}

/// ドロップされたファイルを管理する構造体
#[derive(Getters, MutGetters, Setters)]
pub struct OpenFile {
    /// 本の構造体
    book: file::Book,

    /// 選択されたファイルのインデックス
    #[getset(get = "pub", set = "pub")]
    page: Option<usize>,

    /// 画面に表示させるページリスト
    spreads: Vec<Spread>,

    /// 画面に表示させるページリストのインデックス
    #[getset(get_mut = "pub")]
    current_spread: Option<usize>,

    /// ライブラリの構造体
    // cbz (zip) ファイルだったら同階層の cbz (zip) ファイルをライブラリに追加
    library: file::Library,

    /// ライブラリのインデックス
    volume: Option<usize>,
}

/// public methods
impl OpenFile {
    /// 新しい OpenFile を作成
    /// * `return` - OpenFile のインスタンス
    pub fn new() -> Self {
        Self {
            book: file::Book::new(),
            page: None,
            spreads: vec![],
            current_spread: None,
            library: file::Library::new(),
            volume: None,
        }
    }

    /// ファイルをクリア
    pub fn clear(&mut self) {
        self.book.clear();
        self.page = None;
        self.spreads.clear();
        self.current_spread = None;
        self.library.clear();
        self.volume = None;
    }

    /// 本の名前を取得
    /// * `return` - 本の名前
    pub fn book_title(&self) -> &str {
        &self.book.title()
    }

    /// 本の画像のIDを取得
    /// * `return` - 本の画像の ID ベクター
    pub fn book_image_ids(&self) -> Vec<u64> {
        self.book.image_ids()
    }

    /// ページリストの長さを取得
    /// * `return` - ページリストの長さ
    pub fn spreads_len(&self) -> usize {
        self.spreads.len()
    }

    /// 現在のページリストのインデックスを設定
    /// * `index` - 現在のページリストのインデックス
    pub fn set_current_spread(&mut self, index: usize) {
        self.current_spread = Some(index);

        // ページを更新
        self.update_page();
    }

    /// ページから現在のページリストのインデックスを取得
    /// * `page` - ページ
    /// * `return` - 現在のページリストのインデックス
    pub fn get_current_by_page(&self, page: usize) -> Option<usize> {
        self.spreads.iter().position(|spread| {
            match spread {
                Spread::Single { index } => *index == page,
                Spread::Pair { left, right } => *left == page || *right == page,
            }
        })
    }

    /// 画面に表示させるページリストを作成
    /// * `page_layout` - ページ送り表示方式
    pub fn build_spreads(&mut self, cover_layout: event::CoverLayout, page_layout: event::PageLayout) {
        self.spreads.clear();
        let mut page_index = 0;

        // 表紙（0ページ目）を単独表示
        if !self.book.is_empty() && matches!(cover_layout, event::CoverLayout::Single) {
            self.spreads.push(Spread::Single { index: 0 });
            page_index += 1;
        }

        while page_index < self.book.len() {
            let image = self.book.get_image_by_index(page_index);
            let Some(image) = image else {
                page_index += 1;
                continue;
            };

            // 単一ページで表示
            if matches!(page_layout, event::PageLayout::Single) {
                self.spreads.push(Spread::Single { index: page_index });
                page_index += 1;
                continue;
            }

            // 横長の場合は単一ページで表示
            if image.is_landscape() {
                self.spreads.push(Spread::Single { index: page_index });
                page_index += 1;
                continue;
            }

            // 次のページが存在して、次の画像も縦長であれば見開きページで表示
            if let Some(next_image) = self.book.get_image_by_index(page_index + 1) {
                if !next_image.is_landscape() {
                    // デフォルト Right to Left で表示
                    // Left to Right で表示する場合は Render 側で反転させる
                    let (left, right) = (page_index + 1, page_index);

                    self.spreads.push(Spread::Pair { left, right });
                    page_index += 2;
                    continue;
                }
            }

            // 次の画像がない、または次の画像が横長の場合は単一ページで表示
            self.spreads.push(Spread::Single { index: page_index });
            page_index += 1;
        }

        // ページインデックスから現在のページリストのインデックスを取得
        self.current_spread = if self.spreads.is_empty() {
            None
        } else {
            Some(
                self.page.and_then(|page| self.get_current_by_page(page))
                    .unwrap_or(DEFAULT_PAGE)
            )
        };

        // ページを更新
        self.update_page();
    }

    /// 選択された画像のパスを取得
    /// * `app` - アプリケーション
    /// * `return` - 選択された Image ベクター
    pub fn page_images(&mut self, offset: Option<isize>) -> error::Result<Vec<file::Image>> {
        let Some(mut current_index) = self.current_spread else { return Ok(vec![]); };

        if let Some(offset) = offset {
            if offset > 0 {
                let Some(add_index) = current_index.checked_add(offset.unsigned_abs()) else { return Ok(vec![]); };
                current_index = add_index;
            } else {
                let Some(sub_index) = current_index.checked_sub(offset.unsigned_abs()) else { return Ok(vec![]); };
                current_index = sub_index;
            }
        }

        // ページリストを取得
        let mut images = vec![];
        match self.spreads.get(current_index).copied().ok_or_else(|| {
            error::GachoError::IndexError(current_index)
        })? {
            Spread::Single { index } => {
                let image = self.book.ensure_image_by_index(index)?;
                if let Some(image) = image {
                    images.push(image);
                }
            }
            Spread::Pair { left, right } => {
                // デフォルト Right to Left で表示
                // Left to Right で表示する場合は Render 側で反転させる
                let image = self.book.ensure_image_by_index(left)?;
                if let Some(image) = image {
                    images.push(image);
                }
                let image = self.book.ensure_image_by_index(right)?;
                if let Some(image) = image {
                    images.push(image);
                }
            }
        }

        Ok(images)
    }

    /// 左へのインデックス
    /// * `app` - アプリケーション
    /// * `return` - 左へのインデックス
    pub fn left_page(&mut self, app: &app::App) -> error::Result<Option<usize>> {
        match app.read_from() {
            event::ReadFrom::RightToLeft => {
                if self.is_last_page() {
                    // 次のライブラリを読み込む
                    if self.read_next_library()? {
                        // ページリストを再構築
                        self.build_spreads(*app.cover_layout(), *app.page_layout());
                    }
                } else {
                    if self.page_add() {
                        self.update_page();
                    }
                }
            }
            event::ReadFrom::LeftToRight => {
                if self.is_first_page() {
                    // 前のライブラリを読み込む
                    if self.read_prev_library()? {
                        // ページリストを再構築
                        self.build_spreads(*app.cover_layout(), *app.page_layout());
                    }
                } else {
                    if self.page_subtract() {
                        self.update_page();
                    }
                }
            }
        }

        Ok(self.page)
    }

    /// 右へのインデックス
    // 読み込み方向によって前後が変わる
    /// * `app` - アプリケーション
    /// * `return` - 右へのインデックス
    pub fn right_page(&mut self, app: &app::App) -> error::Result<Option<usize>> {
        match app.read_from() {
            event::ReadFrom::RightToLeft => {
                if self.is_first_page() {
                    // 前のライブラリを読み込む
                    if self.read_prev_library()? {
                        // ページリストを再構築
                        self.build_spreads(*app.cover_layout(), *app.page_layout());
                    }
                } else {
                    if self.page_subtract() {
                        self.update_page();
                    }
                }
            }
            event::ReadFrom::LeftToRight => {
                if self.is_last_page() {
                    // 次のライブラリを読み込む
                    if self.read_next_library()? {
                        // ページリストを再構築
                        self.build_spreads(*app.cover_layout(), *app.page_layout());
                    }
                } else {
                    if self.page_add() {
                        self.update_page();
                    }
                }
            }
        }

        Ok(self.page)
    }

    /// 本を追加
    /// * `path` - ドロップされたファイルのパス
    /// * `return` - 結果
    pub fn open_book(&mut self, path: PathBuf) -> error::Result<bool> {
        // 本に画像を追加
        let image_name = self.book.open_from_path(path.clone())?;

        // 本に画像が追加されていない場合はスキップ
        if self.book.is_empty() {
            return Ok(false);
        }

        // 画像ファイルから開かれたら、ページインデックスを取得
        if let Some(image_name) = image_name {
            self.page = self.book.get_index_by_filename(&image_name);
        } else {
            self.page = Some(DEFAULT_PAGE);
        }

        // ライブラリにファイルを追加
        self.library.add_entry(path)?;

        // ライブラリのインデックスを取得
        self.volume = self.library.get_index_by_path(&self.book.path());

        Ok(true)
    }
}

/// private methods
impl OpenFile {
    /// 最初のページかどうか
    /// * `return` - 最初のページかどうか
    fn is_first_page(&self) -> bool {
        let Some(index) = self.current_spread else { return false; };
        index == 0
    }

    /// 最後のページかどうか
    /// * `app` - アプリケーション
    /// * `return` - 最後のページかどうか
    fn is_last_page(&self) -> bool {
        let Some(index) = self.current_spread else { return false; };
        index >= self.spreads.len().saturating_sub(1)
    }

    /// ページを更新
    /// * `from` - 読み込み方向
    fn update_page(&mut self) {
        if !self.spreads.is_empty() {
            let Some(index) = self.current_spread else { return; };
            let Some(spread) = self.spreads.get(index) else { return; };

            self.page = Some(match spread {
                Spread::Single { index } => *index,
                Spread::Pair { right, .. } => *right,
            });
        }
    }

    /// 次のライブラリを読み込む
    /// * `return` - 次のライブラリを読み込めたかどうか
    fn read_next_library(&mut self) -> error::Result<bool> {
        if self.library.len() == 0 { return Ok(false); }
        let Some(index) = self.volume  else { return Ok(false); };

        if index == self.library.len() - 1 { return Ok(false); }

        self.volume_add();
        if !self.read_book_from_library()? { return Ok(false); }

        Ok(true)
    }

    /// 前のライブラリを読み込む
    /// * `return` - 結果
    fn read_prev_library(&mut self) -> error::Result<bool> {
        if self.library.len() == 0 { return Ok(false); }
        let Some(index) = self.volume  else { return Ok(false); };

        if index == 0 { return Ok(false); }

        self.volume_subtract();
        if !self.read_book_from_library()? { return Ok(false); }

        Ok(true)
    }

    /// ライブラリから本を読み込む
    /// * `return` - 結果
    fn read_book_from_library(&mut self) -> error::Result<bool> {
        let Some(index) = self.volume else { return Ok(false); };
        let Some(entry) = self.library.get(index) else { return Ok(false); };
        let path = entry.path().clone();

        self.book.clear();
        self.page = None;
        self.current_spread = None;

        self.open_book(path)?;

        Ok(true)
    }

    /// 次のファイルを取得
    /// * `offset` - オフセット
    fn page_add(&mut self) -> bool {
        if let Some(index) = self.current_spread {
            if index < self.spreads.len() - 1 {
                self.current_spread = Some(index + 1);
            } else {
                self.current_spread = Some(self.spreads.len() - 1);
            }

            return true;
        }

        return false;
    }

    /// 前のファイルを取得
    /// * `offset` - オフセット
    fn page_subtract(&mut self) -> bool {
        if let Some(index) = self.current_spread {
            if index as isize > 0 {
                self.current_spread = Some(index - 1);
            } else {
                self.current_spread = Some(0);
            }

            return true;
        }

        return false;
    }

    /// 次のボリュームを取得
    fn volume_add(&mut self) {
        if let Some(index) = self.volume {
            if index + 1 < self.library.len() {
                self.volume = Some(index + 1);
            } else {
                self.volume = Some(self.library.len() - 1);
            }
        }
    }

    /// 前のボリュームを取得
    fn volume_subtract(&mut self) {
        if let Some(index) = self.volume {
            if index > 0 {
                self.volume = Some(index - 1);
            } else {
                self.volume = Some(0);
            }
        }
    }
}
