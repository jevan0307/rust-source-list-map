use linked_hash_map::LinkedHashMap;
use Node;

#[derive(Clone, Debug)]
pub struct MappingsContext {
    pub sources: LinkedHashMap<String, (usize, Option<Node>)>,
    pub has_source_content: bool,
    pub current_original_line: usize,
    pub current_source: usize,
    pub unfinished_generated_line: usize,
}

impl MappingsContext {
    pub fn new() -> Self {
        MappingsContext {
            sources: LinkedHashMap::new(),
            has_source_content: false,
            current_original_line: 1,
            current_source: 0,
            unfinished_generated_line: 0,
        }
    }

    pub fn ensure_source(&mut self, src: Option<String>, original_source: Option<Node>) -> usize {
        let src = match src {
            Some(s) => s,
            None => String::new(),
        };

        if self.sources.contains_key(&src) {
            self.sources.get(&src).unwrap().0
        } else {
            let sources_indices_len = self.sources.len();
            if let Some(Node::NString(_)) = original_source {
                self.has_source_content = true;
            }
            self.sources
                .insert(src, (sources_indices_len, original_source));
            sources_indices_len
        }
    }

    pub fn get_arrays(&self) -> Srcs {
        let mut sources: Vec<String> = Vec::new();
        let mut sources_content: Vec<Node> = Vec::new();
        for (key, val) in self.sources.clone() {
            if !key.is_empty() {
                sources.push(key);
                if let Some(content) = val.1 {
                    sources_content.push(content);
                }
            }
        }

        Srcs {
            sources,
            sources_content,
        }
    }
}

pub struct Srcs {
    pub sources: Vec<String>,
    pub sources_content: Vec<Node>,
}
