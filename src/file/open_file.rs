use std::path::PathBuf;
use std::collections::HashSet;
use getset::{Getters, MutGetters, Setters};

use crate::{app, event, file, error};

const DEFAULT_PAGE: usize = 0;

/// ドロップされたファイルを管理する構造体
#[derive(Getters, MutGetters, Setters)]
pub struct OpenFile {
    /// 本の構造体
    book: file::Book,

    /// 選択されたファイルのインデックス
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    page: Option<usize>,

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
            library: file::Library::new(),
            volume: None,
        }
    }

    /// ファイルをクリア
    pub fn clear(&mut self) {
        self.book.clear();
        self.page = None;
        self.library.clear();
        self.volume = None;
    }

    /// 本の名前を取得
    /// * `return` - 本の名前
    pub fn book_title(&self) -> &str {
        &self.book.title()
    }

    /// 本のページ数を取得
    /// * `return` - 本のページ数
    pub fn book_len(&self) -> usize {
        self.book.len()
    }

    /// 本の画像のIDを取得
    /// * `return` - 本の画像の ID ベクター
    pub fn book_image_ids(&self) -> Vec<u64> {
        self.book.image_ids()
    }

    /// 選択された画像のパスを取得
    /// * `app` - アプリケーション
    /// * `return` - 選択された Image ベクター
    pub fn page_images(&mut self, app: &app::App) -> error::Result<Vec<file::Image>> {
        let Some(index) = self.page else { return Ok(vec![]); };

        let mut images = vec![];
        match app.page_layout() {
            event::PageLayout::Single => {
                let image = self.book.ensure_image_by_index(index)?;
                if let Some(image) = image {
                    images.push(image);
                }
            }
            event::PageLayout::Spread => {
                for offset in 0..=app.page_layout().to_offset() {
                    let Some(add_index) = index.checked_add(offset) else { continue; };
                    let image = self.book.ensure_image_by_index(add_index)?;
                    if let Some(image) = image {
                        images.push(image);
                    }
                }

                // 重複を削除
                let mut seen = HashSet::new();
                let unique_images = images.into_iter()
                    .filter(|image| seen.insert(*image.id()))
                    .collect();

                images = unique_images;
            }
        }

        Ok(images)
    }

    /// 指定した次のインデックスを移動させて画像を取得
    /// * `offset` - オフセット
    /// * `return` - 次のファイル
    pub fn selected_next_image(&mut self, offset: usize) -> error::Result<Option<file::Image>> {
        let Some(index) = self.page else { return Ok(None); };
        let Some(add_index) = index.checked_add(offset) else { return Ok(None); };

        self.book.ensure_image_by_index(add_index)
    }

    /// 指定した前のインデックスを移動させて画像を取得
    /// * `offset` - オフセット
    /// * `return` - 前のファイル
    pub fn selected_prev_image(&mut self, offset: usize) -> error::Result<Option<file::Image>> {
        let Some(index) = self.page else { return Ok(None); };
        let Some(sub_index) = index.checked_sub(offset) else { return Ok(None); };

        self.book.ensure_image_by_index(sub_index)
    }

    /// 次のインデックス
    /// * `app` - アプリケーション
    /// * `return` - 次のインデックス
    pub fn left_page(&mut self, app: &app::App) -> error::Result<Option<usize>> {
        match app.read_from() {
            event::ReadFrom::RightToLeft => {
                if self.is_last_page(app) {
                    self.read_next_library()?;
                } else {
                    self.page_add(app.page_layout().to_offset());
                }
            }
            event::ReadFrom::LeftToRight => {
                if self.is_first_page() {
                    self.read_prev_library()?;
                } else {
                    self.page_subtract(app.page_layout().to_offset());
                }
            }
        }

        // 見開き
        if matches!(app.page_layout(), event::PageLayout::Spread) {
            if let Some(index) = self.page {
                // 最後のページの調整
                if index % 2 != 0 {
                    self.page = Some(index - 1);
                }
            }
        }

        Ok(self.page)
    }

    /// 前のインデックス
    // 読み込み方向によって前後が変わる
    /// * `app` - アプリケーション
    /// * `return` - 前のインデックス
    pub fn right_page(&mut self, app: &app::App) -> error::Result<Option<usize>> {
        match app.read_from() {
            event::ReadFrom::RightToLeft => {
                if self.is_first_page() {
                    self.read_prev_library()?;
                } else {
                    self.page_subtract(app.page_layout().to_offset());
                }
            }
            event::ReadFrom::LeftToRight => {
                if self.is_last_page(app) {
                    self.read_next_library()?;
                } else {
                    self.page_add(app.page_layout().to_offset());
                }
            }
        }

        // 見開き
        if matches!(app.page_layout(), event::PageLayout::Spread) {
            if let Some(index) = self.page {
                // 最後のページの調整
                if index % 2 != 0 {
                    self.page = Some(index - 1);
                }
            }
        }

        Ok(self.page)
    }

    /// 本を追加
    /// * `path` - ドロップされたファイルのパス
    /// * `return` - 結果
    pub fn add_book(&mut self, path: PathBuf) -> error::Result<()> {
        // 本に画像を追加
        let image_name = self.book.open_from_path(path.clone())?;

        // 本に画像が追加されていない場合はスキップ
        if self.book.is_empty() {
            return Ok(());
        }

        // 画像ファイルから開かれたら
        if let Some(image_name) = image_name {
            self.page = self.book.get_index_by_filename(&image_name);
        } else {
            self.page = Some(DEFAULT_PAGE);
        }

        // ライブラリにファイルを追加
        self.library.add_entry(path)?;

        // ライブラリのインデックスを取得
        self.volume = self.library.get_index_by_path(&self.book.path());

        Ok(())
    }
}

/// private methods
impl OpenFile {
    /// 最初のページかどうか
    /// * `return` - 最初のページかどうか
    fn is_first_page(&self) -> bool {
        let Some(index) = self.page else { return false; };
        index == 0
    }

    /// 最後のページかどうか
    /// * `return` - 最後のページかどうか
    fn is_last_page(&self, app: &app::App) -> bool {
        let Some(index) = self.page else { return false; };
        index + app.page_layout().to_offset() >= self.book.len().saturating_sub(1)
    }

    /// 次のライブラリを読み込む
    fn read_next_library(&mut self) -> error::Result<()> {
        if self.library.len() == 0 { return Ok(()); }

        let Some(index) = self.volume  else { return Ok(()); };

        if index == self.library.len() - 1 {
            return Ok(());
        }

        self.volume_add();
        self.read_book_from_library()?;

        Ok(())
    }

    /// 前のライブラリを読み込む
    /// * `return` - 結果
    fn read_prev_library(&mut self) -> error::Result<()> {
        if self.library.len() == 0 { return Ok(()); }

        let Some(index) = self.volume  else { return Ok(()); };

        if index == 0 {
            return Ok(());
        }

        self.volume_subtract();
        self.read_book_from_library()?;

        Ok(())
    }

    /// ライブラリから本を読み込む
    /// * `return` - 結果
    fn read_book_from_library(&mut self) -> error::Result<()> {
        let Some(index) = self.volume else { return Ok(()); };
        let Some(entry) = self.library.get(index) else { return Ok(()); };
        let path = entry.path().clone();

        self.book.clear();
        self.page = None;

        self.add_book(path)?;

        Ok(())
    }

    /// 次のファイルを取得
    /// * `offset` - オフセット
    fn page_add(&mut self, offset: usize) {
        if let Some(index) = self.page {
            if index + offset < self.book.len() - 1 {
                self.page = Some(index + offset + 1);
            } else {
                self.page = Some(self.book.len() - 1);
            }
        }
    }

    /// 前のファイルを取得
    /// * `offset` - オフセット
    fn page_subtract(&mut self, offset: usize) {
        if let Some(index) = self.page {
            if (index as isize - offset as isize) > 0 {
                self.page = Some(index - offset - 1);
            } else {
                self.page = Some(0);
            }
        }
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
