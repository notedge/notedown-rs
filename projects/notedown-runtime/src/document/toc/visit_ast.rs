use super::*;
use notedown_ast::{traits::Slugify, ASTKind, ASTNode};

impl TocNode {
    fn last_at_level(&mut self, depth: u8) -> &mut TocNode {
        if depth == 0 || self.children.is_empty() { self } else { self.children.last_mut().unwrap().last_at_level(depth - 1) }
    }
}

impl TableOfContent for ASTNode {
    fn toc_configurable(&self, config: &TocConfig) -> TocNode {
        let mut root = TocNode::default();
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
                        let new = TocNode { level, detail: header.slugify(), range: self.range.to_owned().unwrap_or_default(), children: vec![] };
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
