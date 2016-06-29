use crate::{ast::ASTKind, ASTNode, TextRange};

#[derive(Debug)]
pub struct TOC {
    pub level: usize,
    pub detail: String,
    pub range: TextRange,
    pub children: Vec<TOC>,
}

impl Default for TOC {
    fn default() -> Self {
        Self { level: 0, detail: String::from("ROOT"), range: Default::default(), children: vec![] }
    }
}

impl TOC {
    fn last_at_level(&mut self, depth: usize) -> &mut TOC {
        if depth == 0 || self.children.len() == 0 { self } else { self.children.last_mut().unwrap().last_at_level(depth - 1) }
    }
}

impl ASTNode {
    pub fn toc(&self, max_depth: usize) -> TOC {
        let mut root = TOC::default();
        let mut toc_ignore = false;
        if let ASTKind::Statements(terms) = &self.kind {
            for term in terms {
                match &term.kind {
                    ASTKind::Header(header) => {
                        let level = header.level;
                        if toc_ignore {
                            toc_ignore = false;
                            continue;
                        }
                        if level > max_depth {
                            continue;
                        }
                        let parent = root.last_at_level(level - 1);
                        let new = TOC { level, detail: join_ast_list(&header.children), range: term.range, children: vec![] };
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

pub fn join_ast_list(list: &[ASTNode]) -> String {
    let mut out = String::new();
    for i in list {
        out.push_str(&i.to_string())
    }
    return out;
}
