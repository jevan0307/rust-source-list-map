extern crate vlq;

use source_node::SourceNode;
use code_node::CodeNode;
use source_list_map::SourceListMap;
use source_list_map::GenCode;
use Node;

pub fn from_string_with_source_map(code: &str,
                                   sources: Vec<&str>,
                                   sources_content: Vec<&str>,
                                   mappings: &str) -> SourceListMap {
	let mappings: Vec<&str> = mappings.split(';').collect();
	let lines: Vec<&str> = code.split('\n').collect();
	let mut nodes: Vec<Node> = vec![];

    let mut current_line: usize = 1;
	let mut current_source_index: usize = 0;
	let mut current_source_node_line: usize = 0;

    for (i, mapping) in mappings.iter().enumerate() {
        match lines.get(i) {
            Some(line) => {
                let line = if i != lines.len() - 1 {
                    String::from(*line) + "\n"
                } else {
                    String::from(*line)
                };
                if mapping.is_empty() {
                    let mut line_added: bool = false;
                    let mut rest = mapping.as_bytes().iter().cloned().peekable();

                    while let Some(_) = rest.peek() {
                        line_added = {
                            if let Some(c) = rest.clone().peek() {
                                if *c != (',' as u8) {
                                    vlq::decode(&mut rest).unwrap();
                                }
                            }

                            match rest.clone().peek() {
                                None => {
                                    false
                                }
                                Some(c) => {
                                    if *c == (',' as u8) {
                                        rest.next();
                                        false
                                    } else {
                                        let value = vlq::decode(&mut rest).unwrap();
                                        let source_index = value as usize + current_source_index;
                                        current_source_index = source_index;

                                        let mut line_position: usize;
                                        if let Some(c) = rest.clone().peek() {
                                            if *c != (',' as u8) {
                                                let value = vlq::decode(&mut rest).unwrap();
                                                line_position = value as usize + current_line;
                                                current_line = line_position;
                                            } else {
                                                line_position = current_line;
                                            }
                                        } else {
                                            line_position = current_line;
                                        }

                                        while let Some(c) = rest.clone().peek() {
                                            if *c != (',' as u8) {
                                                rest.next();
                                            }
                                        }

                                        if !line_added {
                                            add_source(&mut nodes, &mut current_source_node_line,
                                                line.clone(),
                                                sources.get(source_index),
                                                sources_content.get(source_index),
                                                line_position);
                                            true
                                        } else {
                                            false
                                        }
                                    }
                                }
                            }
                        } || line_added;
                    }
                    if !line_added {
                        add_code(&mut nodes, &mut current_source_node_line, line);
                    }
                } else {
                    add_code(&mut nodes, &mut current_source_node_line, line);
                }
            }
            _ => {}
        }
    }
    if mappings.len() < lines.len() {
        let line_len = lines.len();
        let mut index = mappings.len();
        while index < line_len - 1 && !lines[index].trim().is_empty() {
            let line = String::from(lines[index]);
            add_code(&mut nodes, &mut current_source_node_line, line);
            index += 1;
        }
        let line = String::from(lines[index..].join("\n"));
        add_code(&mut nodes, &mut current_source_node_line, line);
    }
    SourceListMap::new(Some(GenCode::CodeVec(nodes)), "", "")
}

fn add_code(nodes: &mut Vec<Node>,
            current_source_node_line: &mut usize,
            generated_code: String) {
    match nodes.last_mut() {
        Some(Node::NCodeNode(ref mut n)) => {
            n.add_generated_code(&generated_code);
            return;
        }
        Some(Node::NSourceNode(ref mut n)) => {
            n.add_generated_code(&generated_code);
            *current_source_node_line += 1;
            return;
        }
        _ => {}
    }
    nodes.push(Node::NCodeNode(CodeNode::new(generated_code)));
}

fn add_source(nodes: &mut Vec<Node>,
              current_source_node_line: &mut usize,
              generated_code: String,
              source: Option<&&str>,
              original_source: Option<&&str>,
              line: usize) {
    let source = match source {
        Some(s) => String::from(*s),
        None => String::from(""),
    };
    let original_source = match original_source {
        Some(s) => String::from(*s),
        None => String::from(""),
    };

    if let Some(Node::NSourceNode(ref mut n)) = nodes.last_mut() {
         if n.source == source &&
            *current_source_node_line == line {
            n.add_generated_code(&generated_code);
            *current_source_node_line += 1;
            return;
         }
    }
    nodes.push(Node::NSourceNode(SourceNode::new(generated_code,
                                                 source,
                                                 original_source,
                                                 line)));
    *current_source_node_line = line + 1;
}
