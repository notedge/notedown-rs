use crate::{TextRange, AST};

#[derive(Debug)]
pub struct TOC {
    level: usize,
    pub detail: String,
    pub range: TextRange,
    pub selection_range: TextRange,
    pub children: Vec<TOC>,
}

impl Default for TOC {
    fn default() -> Self {
        Self { level: 0, detail: String::from("ROOT"), range: Default::default(), selection_range: Default::default(), children: vec![] }
    }
}

impl TOC {
    fn last_at_level(&mut self, depth: usize) -> &mut TOC {
        if depth == 0 || self.children.len() == 0 { self } else { self.children.last_mut().unwrap().last_at_level(depth - 1) }
    }
}

impl AST {
    pub fn toc(&self, max_depth: usize) -> TOC {
        let mut root = TOC::default();
        let mut toc_ignore = false;
        if let AST::Statements(terms) = self {
            for term in terms {
                match term {
                    AST::Header { level, children, r } => {
                        if toc_ignore {
                            toc_ignore = false;
                            continue;
                        }
                        if *level > max_depth {
                            continue;
                        }
                        let parent = root.last_at_level(level - 1);
                        let (a, b) = r.start;
                        let new = TOC {
                            level: *level,
                            detail: join_ast_list(children),
                            range: *r,
                            selection_range: TextRange::new(a, b, a, b),
                            children: vec![],
                        };
                        parent.children.push(new);
                    }
                    AST::Command { cmd, .. } => {
                        if let "toc_ignore" = cmd.as_str() {
                            toc_ignore = true
                        }
                    }
                    _ => (),
                }
            }
        };
        return root;
    }
}

pub fn join_ast_list(list: &[AST]) -> String {
    let mut out = String::new();
    for i in list {
        out.push_str(&i.to_string())
    }
    return out;
}
