use super::*;
use crate::VMFileSystem;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum DocumentTime {
    /// `\date: runtime-today`
    RuntimeToday,
    /// `\date: file-changed`
    FileChanged,
    /// `\date: file-created`
    FileCreated,
    /// `\date: git-changed`
    GitChanged,
    /// `\date: git-created`
    GitCreated,
    /// UTC time in any cases
    DateTime(NaiveDateTime),
}

impl Default for DocumentTime {
    fn default() -> Self {
        Self::RuntimeToday
    }
}

impl DocumentTime {
    #[inline]
    pub fn parse_unix(date: i64) -> Self {
        let time = NaiveDateTime::from_timestamp(date, 0);
        Self::DateTime(time)
    }
    #[inline]
    pub fn parse_fmt(date: &str, fmt: &str) -> Result<Self> {
        let time = NaiveDateTime::parse_from_str(date, fmt)?;
        Ok(Self::DateTime(time))
    }
    #[inline]
    pub fn now() -> DocumentTime {
        Self::DateTime(Utc::now().naive_utc())
    }
    #[inline]
    pub fn instantiate(&self, file: &Url) -> Option<NaiveDateTime> {
        match self {
            Self::RuntimeToday => None,
            Self::FileChanged => {
                todo!()
            }
            Self::FileCreated => {
                todo!()
            }
            Self::GitChanged => Self::instantiate_git_time(false, file),
            Self::GitCreated => Self::instantiate_git_time(true, file),
            Self::DateTime(t) => Some(t.to_owned()),
        }
    }
    fn instantiate_git_time(created: bool, file: &Url) -> Option<NaiveDateTime> {
        let path = file.to_file_path().ok()?;
        let git_root = VMFileSystem::find_git_root(&path)?;

        let _ = created;

        todo!()
    }
}

/// Methods about [`DocumentDate`]
impl NoteDocument {
    #[inline]
    pub fn get_date(&self) -> &DocumentTime {
        &self.meta.date
    }
    #[inline]
    pub fn set_date(&mut self, date: DocumentTime) {
        self.meta.date = date;
    }
}
