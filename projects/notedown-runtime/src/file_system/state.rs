use super::*;
use notedown_ast::utils::{lsp_types::Url, Rope};

pub struct FileState {
    /// used to check weather the file needs re-parse
    fingerprint: u128,
    text: Rope,
    ast: ASTNode,
    meta: FileMeta,
}

impl PartialEq for FileState {
    fn eq(&self, other: &Self) -> bool {
        self.fingerprint.eq(&other.fingerprint)
    }
}

impl FileState {
    pub async fn load_file(&mut self, path: &Path) -> Result<()> {
        let mut file = File::open(path).await?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).await?;
        unsafe { self.text = Rope::from_str(&String::from_utf8_unchecked(contents)) }
        Ok(())
    }
    pub async fn load_local(&mut self, url: &Url) {
        todo!("Local: {}", url)
    }
    pub async fn load_remote(&mut self, url: &Url) {
        todo!("Remote: {}", url)
    }
    #[inline]
    pub async fn update_ast(&mut self, parse: &Parser) -> Result<()> {
        let text: String = self.text.chars().collect();
        self.meta.clear();
        parse(&text, &mut self.meta).map(|new| self.ast = new)
    }
}
