use crate::{TextRange, AST};
use std::rc::Rc;

#[derive(Debug)]
pub struct TOC {
    level: usize,
    parent: Option<Box<TOC>>,
    pub detail: String,
    pub range: TextRange,
    pub selection_range: TextRange,
    pub children: Vec<TOC>,
}

impl Default for TOC {
    fn default() -> Self {
        Self {
            level: 0,
            detail: "ROOT".to_string(),
            range: Default::default(),
            selection_range: Default::default(),
            children: vec![],
            parent: None,
        }
    }
}

impl AST {
    pub fn toc(&self, max_depth: usize) -> TOC {
        let mut out = &mut TOC::default();
        let mut last_level = 0;
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

                        if *level > last_level {
                            let mut new = &mut TOC {
                                level: *level,
                                parent: Some(Box::new(out.clone())),
                                detail: join_ast_list(children),
                                range: r.clone(),
                                selection_range: r.clone(),
                                children: vec![],
                            };
                            out.children.push(new.clone());
                            out = &mut new
                        }
                        else {
                            println!("{:#?}", out);
                            println!("{:#?}", level);
                            println!("{}", join_ast_list(children).trim());
                            println!("{:#?}", r);
                            unimplemented!();
                        }
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
        return out;
    }
}

pub fn join_ast_list(list: &[AST]) -> String {
    let mut out = String::new();
    for i in list {
        out.push_str(&i.to_string())
    }
    return out;
}
