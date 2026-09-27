use crate::model::Model;

/// 本のモデル
pub(crate) struct Book {
    model: Model,
}

impl Book {
    /// 新しい本のモデルを作成
    pub fn new() -> Self {
        let model = Model::new();
        Self { model }
    }
}
