use std::path::{Path, PathBuf};
use getset::{Getters, MutGetters, Setters};

use crate::{file, error};

/// 本の情報
#[derive(Getters, MutGetters, Setters)]
pub struct Book {
    /// ファイル一覧
    images: Vec<file::Image>,

    /// アーカイブ
    archive: Option<file::Archive>,

    /// 本の名前
    #[getset(get = "pub")]
    title: String,

    /// パス
    #[getset(get = "pub")]
    path: PathBuf,

    /// 一時ファイルのパス
    temp_path: Vec<PathBuf>,
}

/// public methods
impl Book {
    /// 新しい Book を作成
    /// * `return` - Book のインスタンス
    pub fn new() -> Self {
        Self {
            images: vec![],
            archive: None,
            title: String::new(),
            path: PathBuf::new(),
            temp_path: vec![],
        }
    }

    /// ページ数を取得
    /// * `return` - ページ数
    pub fn len(&self) -> usize {
        self.images.len()
    }

    /// 本が空かどうかを取得
    /// * `return` - 本が空かどうか
    pub fn is_empty(&self) -> bool {
        self.images.is_empty()
    }

    /// 一時ファイルが空かどうかを取得
    /// * `return` - 一時ファイルが空かどうか
    pub fn is_temp_empty(&self) -> bool {
        self.temp_path.is_empty()
    }

    /// 本をクリア
    pub fn clear(&mut self) {
        self.title = String::new();
        self.images.clear();
        self.archive = None;
        self.path = PathBuf::new();
        self.temp_path.clear();
    }

    /// ファイルのIDを取得
    /// * `return` - ファイルのIDリスト
    pub fn image_ids(&self) -> Vec<u64> {
        self.images.iter().map(|image| *image.id()).collect()
    }

    /// インデックスからファイルを取得
    /// * `index` - インデックス
    /// * `return` - Image のインスタンス
    pub fn get_image_by_index(&self, index: usize) -> Option<&file::Image> {
        self.images.get(index)
    }

    /// インデックスからファイルを取得
    /// * `index` - インデックス
    /// * `return` - Image のインスタンス
    pub fn ensure_image_by_index(&mut self, index: usize) -> error::Result<Option<file::Image>> {
        // インデックスが範囲外の場合は None を返す
        if index >= self.len() { return Ok(None); }
        let image = self.images.get_mut(index).ok_or_else(|| {
            error::GachoError::IndexError(index)
        })?;

        // アーカイブの場合は、アーカイブからファイルのバイト列を取得
        if image.is_archive() && image.is_empty_bytes() {
            let archive = self.archive.as_mut().ok_or_else(|| {
                error::GachoError::ArchiveError(
                    format!("Archive not found: {}", image.relative_path())
                )
            })?;

            let archive_file = archive.get_zipfile(*image.archive_index()).map_err(|e| {
                error::GachoError::ArchiveError(
                    format!("{} : {}", e.to_string(), image.relative_path())
                )
            })?;

            image.set_bytes(archive_file.bytes().to_vec().into());
            image.set_size(*archive_file.size());
        }

        Ok(Some(image.clone()))
    }

    /// ファイル名でindexを取得
    /// * `file_name` - ファイル名
    /// * `return` - index
    pub fn get_index_by_filename(&self, name: &str) -> Option<usize> {
        // ファイル名が一致するindexを取得
        self.images.iter().position(|file| {
            let file_name = if let Some(file_name) = file.path().file_name() {
                file_name.to_string_lossy().into_owned()
            } else {
                return false;
            };
            file_name == name
        })
    }

    /// 画像を追加
    /// * `path` - ドロップされたファイルのパス
    /// * `return` - 結果
    pub fn open_from_path_to_temp(&mut self, path: PathBuf) -> error::Result<Option<String>> {
        // parent() は path を借りるので、先に PathBuf にして借用を終わらせる
        let base_dir = path.parent().unwrap_or(&path).to_path_buf();

        // ファイル名の控えを用意
        let mut image_name: Option<String> = None;

        // 一時ファイルをクリア
        self.temp_path.clear();

        if file::is_image(&path) {
            // 画像ファイルの場合は、同ディレクトリの他の画像ファイルも検索する
            if let Some(parent) = path.parent() {
                // ファイルを検索
                self.find_file(&parent.to_path_buf(), &base_dir)?;

                // 画像ファイルの場合は、ファイル名を控えておく
                image_name = if let Some(file_name) = path.file_name() {
                    Some(file_name.to_string_lossy().into_owned())
                } else {
                    None
                };
            }
        } else {
            // ファイルを検索
            self.find_file(&path, &base_dir)?;
        }

        // 一時ファイルをソート
        self.sort_temp_path();

        Ok(image_name)
    }

    /// 一時ファイルから本を開く
    /// * `base_dir` - ドロップされたパスの親（相対パスの基準）
    /// * `return` - 結果
    pub fn open_from_temp_to_book(&mut self, path: &PathBuf) -> error::Result<bool> {
        if self.temp_path.is_empty() {
            return Err(error::GachoError::FileError(
                "No files to open".to_string(),
                path.clone()
            ));
        }

        // parent() は path を借りるので、先に PathBuf にして借用を終わらせる
        let base_dir = path.parent().unwrap_or(&path).to_path_buf();

        // 画像から開いたときは画像だけ、それ以外でアーカイブがあれば先頭の1冊だけ
        let prefer_archive = !file::is_image(&path)
            && self.temp_path.iter().any(|temp| file::is_archive(temp));

        // 一時パス一覧を順番に処理
        for temp in std::mem::take(&mut self.temp_path) {
            // アーカイブを優先する場合
            if prefer_archive {
                if !file::is_archive(&temp) { continue; }
                self.image_new(&temp, &base_dir)?;
                break;
            }

            // 画像の場合
            if !file::is_image(&temp) { continue; }

            // 画像を新規作成して登録
            self.image_new(&temp, &base_dir)?;
        }

        // ファイルをパス順にソート
        self.sort();

        // 本の名前を取得
        if let Some(image) = self.images.first() {
            let title = self.find_title(&image);
            self.title = self.extract_book_title(&title);
        }

        Ok(true)
    }
}

/// private methods
impl Book {
    /// ファイルをパス順にソート
    fn sort(&mut self) {
        self.images.sort_by(|a, b| a.path().cmp(&b.path()));
    }

    /// 一時ファイルをソート
    fn sort_temp_path(&mut self) {
        self.temp_path.sort();
    }

    /// ファイル名から本のタイトルを取り出す。
    /// 同人形式を先に試し、一致しなければ簡易形式を試す。
    fn extract_book_title(&self, name: &str) -> String {
        for re in [&*file::RE_DOJIN_NAME_PATTERN, &*file::RE_BOOK_NAME_PATTERN] {
            if let Some(title) = re.captures(name).and_then(|c| c.name("title")) {
                return title.as_str().trim().to_string();
            }
        }
        name.trim().to_string()
    }

    /// 本の名前を取得
    /// * `image` - 画像のインスタンス
    /// * `return` - 本の名前（拡張子なしのファイル名）
    fn find_title(&self, image: &file::Image) -> String {
        let mut path = image.path().clone();

        // アーカイブの場合は、ファイル名を取得
        let title = if matches!(image.extension(), file::Extension::Zip | file::Extension::Cbz) {
            // 拡張子を削除したファイル名を取得
            path.set_extension("");
            if let Some(name) = path.file_name() {
                name.to_string_lossy().trim().to_string()
            } else {
                image.file_name().clone()
            }

        // ファイルの場合は、ファイル名を取得
        } else if path.is_file() {
            if let Some(parent) = path.parent() {
                if let Some(parent_name) = parent.file_name() {
                    parent_name.to_string_lossy().to_string()
                } else {
                    image.file_name().clone()
                }
            } else {
                image.file_name().clone()
            }
        } else {
            let replace_path = format!("/{}", image.file_name());
            image.relative_path().clone().replace(&replace_path, "")
        };

        title
    }

    /// ファイルを検索
    /// * `path` - ドロップされたファイルのパス
    /// * `base_dir` - ドロップされたパスの親（相対パスの基準）
    /// * `return` - ファイルのパス
    fn find_file(&mut self,
        path: &PathBuf,
        base_dir: &Path,
    ) -> error::Result<()> {
        let metadata = path.metadata().map_err(|e| {
            error::GachoError::FileError(e.to_string(), path.clone())
        })?;

        // ファイルの場合は、画像を新規作成
        if metadata.is_file() {
            self.temp_path.push(path.clone());

        // ディレクトリの場合は、ディレクトリの中のファイルを再起で探索
        } else if metadata.is_dir() {
            // ディレクトリの中のファイルを探索
            for entry in std::fs::read_dir(&path).map_err(|e| {
                error::GachoError::FileError(e.to_string(), path.clone())
            })? {
                let entry = entry.map_err(|e| {
                    error::GachoError::FileError(e.to_string(), path.clone())
                })?;

                let metadata = entry.path().metadata().map_err(|e| {
                    error::GachoError::FileError(e.to_string(), path.clone())
                })?;

                // ディレクトリの中のディレクトリはスキップ
                if metadata.is_dir() {
                    continue;
                }

                self.find_file(&entry.path(), base_dir)?;
            }
        }

        Ok(())
    }

    /// 画像を新規作成
    /// * `path` - ドロップされたファイルのパス
    /// * `base_dir` - ドロップされたパスの親（相対パスの基準）
    /// * `return` - 結果
    fn image_new(&mut self, path: &PathBuf, base_dir: &Path) -> error::Result<()> {
        // strip_prefix は path を借りるので、
        // 先に String にして into_owned()で所有権を移す
        let relative_path = path.strip_prefix(base_dir)
            .unwrap_or(&path)
            .to_string_lossy()
            .into_owned();

        // ファイルがアーカイブかどうかを判断
        if file::is_archive(&path) {
            let mut archive = file::Archive::new();
            archive.unarchive(&path).map_err(|e| {
                error::GachoError::ArchiveError(e.to_string())
            })?;

            for file in archive.files() {
                let image_file = file::Image::new(
                    path.clone(),
                    file.relative_path().clone(),
                    Some(file.file_name().clone()),
                    Some(*file.size()),
                    Some(file.bytes().to_vec()),
                    Some(*file.index()),
                )?;

                // ファイルを追加
                self.images.push(image_file);
            }

            // アーカイブを控えておく
            self.archive = Some(archive);

            // パスを控えておく
            self.path = path.clone();

        } else if file::is_image(&path) {
            // ファイルを作成
            let image_file = file::Image::new(
                path.clone(),
                relative_path,
                None,
                None,
                None,
                None,
            )?;

            // ファイルを追加
            self.images.push(image_file);

            // パスを控えておく
            let parent = path.parent().ok_or_else(|| {
                error::GachoError::FileError(
                    "Parent not found".to_string(),
                    path.clone()
                )
            })?;
            self.path = parent.to_path_buf();
        }

        Ok(())
    }
}
