use crate::utils::LSPMetaInfo;
use crate::ASTNode;

pub type LSPValue = ASTNode<LSPMetaInfo>;

impl LSPValue {
    pub fn set_range(&mut self, x1: u32, y1: u32, x2: u32, y2: u32) {
        self.meta.set_range(x1, y1, x2, y2)
    }
}
