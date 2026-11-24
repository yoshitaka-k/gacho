mod token;

pub(crate) use token::SettingToken;
pub(crate) use token::ErrorToken;

use crate::{config, file, lang, update};

/// アプリケーションの状態
pub(crate) struct State {
    pub config: config::Config,
    pub files: file::OpenFiles,
    pub setting_token: SettingToken,
    pub update_token: update::UpdateToken,
    pub error_token: ErrorToken,
}

/// ジョブ
pub(crate) struct Job {
    pub update_job: update::UpdateJob,
}

/// 設定タブ
#[derive(PartialEq)]
pub(crate) enum SettingTab {
    General,
    Layout,
    Memory,
    About,
}
impl std::fmt::Display for SettingTab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SettingTab::General => write!(f, "{}", lang::current().setting.general),
            SettingTab::Layout => write!(f, "{}", lang::current().setting.layout),
            SettingTab::Memory => write!(f, "{}", lang::current().setting.memory),
            SettingTab::About => write!(f, "{}", lang::current().setting.about),
        }
    }
}
