#[derive(Debug, Clone)]
pub struct TOC {
    level: usize,
    detail: String,
    children: Vec<TOC>,
}

impl TOC {
    fn last_at_level(&mut self, depth: usize) -> &mut TOC {
        if depth == 0 || self.children.len() == 0 { self } else { self.children.last_mut().unwrap().last_at_level(depth - 1) }
    }
}

fn build_toc(ts: &[(usize, String)]) -> TOC {
    let mut root = TOC { level: 0, detail: "ROOT".to_string(), children: vec![] };
    for (level, detail) in ts {
        let parent = root.last_at_level(level - 1);
        let new = TOC { level: *level, detail: detail.to_owned(), children: vec![] };
        parent.children.push(new);
    }
    return root;
}

#[test]
fn main() {
    let titles = vec![
        (1, "title1".to_string()),
        (3, "title2".to_string()),
        (2, "title3".to_string()),
        (3, "title4".to_string()),
        (2, "title5".to_string()),
        (3, "title6".to_string()),
        (1, "title6".to_string()),
    ];

    println!("{:#?}", build_toc(&titles))
}
