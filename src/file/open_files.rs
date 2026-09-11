use std::path::PathBuf;
use std::collections::HashSet;
use getset::{Getters, MutGetters, Setters};

use crate::{app, event, file, error};

const DEFAULT_SELECTED_INDEX: usize = 0;

/// ドロップされたファイルを管理する構造体
#[derive(Getters, MutGetters, Setters)]
pub struct OpenFiles {
    /// ファイル一覧
    book: file::Book,

    /// 選択されたファイルのインデックス
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    selected_index: Option<usize>,
}

/// public methods
impl OpenFiles {
    /// 新しい OpenFiles を作成
    /// * `return` - OpenFiles のインスタンス
    pub fn new() -> Self {
        Self {
            book: file::Book::new(),
            selected_index: None,
        }
    }

    /// ファイルをクリア
    pub fn clear(&mut self) {
        self.book.clear();
        self.selected_index = None;
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
    /// * `return` - 本の画像のID
    pub fn book_image_ids(&self) -> Vec<u64> {
        self.book.image_ids()
    }

    /// 選択された画像のパスを取得
    /// * `return` - 選択されたファイルのパス
    pub fn selected_index_images(&mut self, app: &app::App) -> error::Result<Vec<file::Image>> {
        let Some(index) = self.selected_index else { return Ok(vec![]); };

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
        let Some(index) = self.selected_index else { return Ok(None); };
        let Some(add_index) = index.checked_add(offset) else { return Ok(None); };

        self.book.ensure_image_by_index(add_index)
    }

    /// 指定した前のインデックスを移動させて画像を取得
    /// * `offset` - オフセット
    /// * `return` - 前のファイル
    pub fn selected_prev_image(&mut self, offset: usize) -> error::Result<Option<file::Image>> {
        let Some(index) = self.selected_index else { return Ok(None); };
        let Some(sub_index) = index.checked_sub(offset) else { return Ok(None); };

        self.book.ensure_image_by_index(sub_index)
    }

    /// 次のインデックス
    /// * `app` - アプリケーション
    /// * `return` - 次のインデックス
    pub fn next_index(&mut self, app: &app::App) -> error::Result<Option<usize>> {
        match app.read_from() {
            event::ReadFrom::RightToLeft => self.from_next(app.page_layout().to_offset()),
            event::ReadFrom::LeftToRight => self.from_prev(app.page_layout().to_offset()),
        }

        // 見開き
        if matches!(app.page_layout(), event::PageLayout::Spread) {
            if let Some(index) = self.selected_index {
                // 最後のページの調整
                if index % 2 != 0 {
                    self.selected_index = Some(index - 1);
                }
            }
        }

        Ok(self.selected_index)
    }

    /// 前のインデックス
    // 読み込み方向によって前後が変わる
    /// * `return` - 前のインデックス
    pub fn prev_index(&mut self, app: &app::App) -> error::Result<Option<usize>> {
        match app.read_from() {
            event::ReadFrom::RightToLeft => self.from_prev(app.page_layout().to_offset()),
            event::ReadFrom::LeftToRight => self.from_next(app.page_layout().to_offset()),
        }

        // 見開き
        if matches!(app.page_layout(), event::PageLayout::Spread) {
            if let Some(index) = self.selected_index {
                // 最後のページの調整
                if index % 2 != 0 {
                    self.selected_index = Some(index - 1);
                }
            }
        }

        Ok(self.selected_index)
    }

    /// 本を追加
    /// * `path` - ドロップされたファイルのパス
    /// * `return` - 結果
    pub fn add_book(&mut self, path: PathBuf) -> error::Result<()> {
        // 本に画像を追加
        let file_name = self.book.open_from_path(path)?;

        // 本に画像が追加されたら
        if self.book.len() > 0 {
            // 画像ファイルから開かれたら
            if let Some(file_name) = file_name {
                self.selected_index = self.book.get_index_by_filename(&file_name);
            } else {
                self.selected_index = Some(DEFAULT_SELECTED_INDEX);
            }
        }

        Ok(())
    }
}

/// private methods
impl OpenFiles {
    /// 次のファイルを取得
    fn from_next(&mut self, offset: usize) {
        if let Some(index) = self.selected_index {
            if index + offset < self.book.len() - 1 {
                self.selected_index = Some(index + offset + 1);
            } else {
                self.selected_index = Some(self.book.len() - 1);
            }
        }
    }

    /// 前のファイルを取得
    fn from_prev(&mut self, offset: usize) {
        if let Some(index) = self.selected_index {
            if (index as isize - offset as isize) > 0 {
                self.selected_index = Some(index - offset - 1);
            } else {
                self.selected_index = Some(0);
            }
        }
    }
}
