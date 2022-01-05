use super::*;

impl VMFileSystem {
    pub fn find_git_root(base: &Path) -> Option<PathBuf> {
        let mut path: PathBuf = base.into();
        let git_root = Path::new(".git");
        loop {
            path.push(git_root);

            if path.is_file() {
                break Some(path);
            }

            if !(path.pop() && path.pop()) {
                // remove file && remove parent
                break None;
            }
        }
    }
    pub fn find_git_repo(base: &Path) {}
}
