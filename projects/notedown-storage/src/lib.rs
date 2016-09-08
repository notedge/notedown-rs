mod get_env;

#[cfg(feature = "syntect")]
mod get_syntect;

mod get_download;

pub use self::get_env::{get_root_path, set_root_path, NOTEDOWN_ROOT};
#[cfg(feature = "syntect")]
pub use self::get_syntect::{get_syntect_language_addition, get_syntect_language_pack, get_syntect_themes};
