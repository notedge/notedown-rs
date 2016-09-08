use notedown_ast::{NoteError, Result};
use std::{
    env::VarError,
    path::{Path, PathBuf},
};

pub static NOTEDOWN_ROOT: &str = "NOTEDOWN_ROOT";

pub fn set_root_path<P: AsRef<Path>>(path: P) -> Result<()> {
    Ok(std::env::set_var(NOTEDOWN_ROOT, path.as_ref().display()))
}

pub fn get_root_path() -> Result<PathBuf> {
    let maybe_dir = PathBuf::from(get_env_var(NOTEDOWN_ROOT)?);
    match maybe_dir.is_dir() {
        true => Ok(maybe_dir),
        false => Err(NoteError::runtime_error(&format!("The environment variable {} does not point to a directory!", NOTEDOWN_ROOT))),
    }
}

fn get_env_var(key: &str) -> Result<String> {
    match std::env::var(NOTEDOWN_ROOT) {
        Ok(o) => Ok(o),
        Err(VarError::NotPresent) => Err(NoteError::runtime_error(&format!("The environment variable {} does not found!", key))),
        Err(VarError::NotUnicode(s)) => Err(NoteError::runtime_error(&format!("The environment variable {} seems not valid unicode: {:?}", key, s))),
    }
}
