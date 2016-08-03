use super::*;

use crate::{nodes::ASTKind, ASTNode};
use crate::traits::Slugify;


impl TableNode {
    fn last_at_level(&mut self, depth: u8) -> &mut TableNode {
        if depth == 0 || self.children.len() == 0 { self } else { self.children.last_mut().unwrap().last_at_level(depth - 1) }
    }
}

impl TableOfContent for ASTNode {
    fn table_of_content(&self, config: TableConfig) -> TableNode {
        let mut root = TableNode::default();
        let mut toc_ignore = false;
        if let ASTKind::Statements(terms) = &self.value {
            for term in terms {
                match &term.value {
                    ASTKind::Header(header) => {
                        let level = header.level;
                        if toc_ignore {
                            toc_ignore = false;
                            continue;
                        }
                        if level > config.max_depth {
                            continue;
                        }
                        let parent = root.last_at_level(level - 1);
                        let new = TableNode {
                            level,
                            detail: header.slugify(),
                            range: todo!(),
                            children: vec![],
                        };
                        parent.children.push(new);
                    }
                    ASTKind::Command(cmd) => {
                        if cmd.is("toc_ignore") {
                            toc_ignore = true
                        }
                    }
                    _ => (),
                }
            }
        }
        return root;
    }
}
