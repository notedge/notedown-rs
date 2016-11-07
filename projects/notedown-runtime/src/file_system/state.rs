use super::*;
use ropey::Rope;

pub struct FileState {
    /// used to check weather the file needs re-parse
    fingerprint: u128,
    text: Rope,
    ast: ASTNode,
    meta: FileMeta,
}

impl FileState {
    pub async fn load_file(&mut self, path: &Path) -> Result<()> {
        self.text = Rope::from_reader(File::open(path).await?)?;
        Ok(())
    }
    pub async fn load_local(&mut self, url: &Url) {
        todo!("Local: {}", url)
    }
    pub async fn load_remote(&mut self, url: &Url) {
        todo!("Remote: {}", url)
    }
    #[inline]
    pub async fn update_ast(&mut self, parse: Parser) -> Result<()> {
        let text: String = self.text.chars().collect();
        self.meta.clear();
        parse(&text, &mut self.meta).map(|new| self.ast = new)
    }
}
