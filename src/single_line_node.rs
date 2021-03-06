use helpers;
use mappings_context::MappingsContext;
use source_node::SourceNode;
use std::rc::Rc;
use std::str;
use vlq;
use MappingFunction;
use Node;
use StringPtr;

#[derive(Clone, Debug)]
pub struct SingleLineNode {
    pub generated_code: String,
    pub original_source: Option<Rc<String>>,
    pub source: Option<Rc<String>>,
    pub line: usize,
    pub _number_of_lines: usize,
    pub _ends_with_new_line: bool,
}

impl SingleLineNode {
    pub fn new(
        generated_code: String,
        source: Option<StringPtr>,
        original_source: Option<StringPtr>,
        line: usize,
    ) -> Self {
        let source = source.map(|sp| sp.to_ptr());
        let original_source = original_source.map(|sp| sp.to_ptr());

        SingleLineNode {
            original_source,
            source,
            line,
            _number_of_lines: helpers::number_of_lines(&generated_code),
            _ends_with_new_line: generated_code.ends_with('\n'),
            generated_code,
        }
    }

    pub fn map_generated_code<T: MappingFunction>(self, mf: &mut T) -> SingleLineNode {
        let generated_code = mf.map(self.generated_code);
        SingleLineNode::new(
            generated_code,
            self.source.clone().map(|p| StringPtr::Ptr(p)),
            self.original_source.clone().map(|p| StringPtr::Ptr(p)),
            self.line,
        )
    }

    pub fn merge(self, other_node: &Node) -> Result<Node, Node> {
        match other_node {
            Node::NSingleLineNode(n) => self.merge_single_line_node(n),
            _ => Err(Node::NSingleLineNode(self)),
        }
    }

    fn merge_single_line_node(mut self, other_node: &SingleLineNode) -> Result<Node, Node> {
        if self.source == other_node.source && self.original_source == other_node.original_source {
            if self.line == other_node.line {
                self.generated_code += &other_node.generated_code;
                self._number_of_lines += other_node._number_of_lines;
                self._ends_with_new_line = other_node._ends_with_new_line;
                Ok(Node::NSingleLineNode(self))
            } else if self.line + 1 == other_node.line
                && self._ends_with_new_line
                && self._number_of_lines == 1
                && other_node._number_of_lines <= 1
            {
                Ok(Node::NSourceNode(SourceNode::new(
                    self.generated_code + &other_node.generated_code,
                    self.source.map(|p| StringPtr::Ptr(Rc::clone(&p))),
                    self.original_source.map(|p| StringPtr::Ptr(Rc::clone(&p))),
                    self.line,
                )))
            } else {
                Err(Node::NSingleLineNode(self))
            }
        } else {
            Err(Node::NSingleLineNode(self))
        }
    }

    // fn add_single_line_node(&mut self, other_node: SingleLineNode) {
    //     self.generated_code += &other_node.generated_code;
    //     self.number_of_lines += other_node.number_of_lines;
    //     self.ends_with_new_line = other_node.ends_with_new_line;
    //     self
    // }

    pub fn get_generated_code(&self) -> &str {
        &self.generated_code
    }

    pub fn get_mappings(&self, mappings_context: &mut MappingsContext) -> String {
        let mut buf = Vec::<u8>::new();
        if self.generated_code.is_empty() {
            String::new()
        } else {
            let line_mapping = ";AAAA";
            let lines = self._number_of_lines;
            let source_index = mappings_context.ensure_source(
                self.source.clone(),
                self.original_source.clone().map(|p| Node::NRcString(p)),
            );

            let mut mappings = String::from("A");
            if mappings_context.unfinished_generated_line != 0 {
                vlq::encode(mappings_context.unfinished_generated_line as i64, &mut buf).unwrap();
                mappings = String::from(",");
            }
            vlq::encode(
                source_index as i64 - mappings_context.current_source as i64,
                &mut buf,
            ).unwrap();
            vlq::encode(
                self.line as i64 - mappings_context.current_original_line as i64,
                &mut buf,
            ).unwrap();
            buf.push(b'A');
            mappings += str::from_utf8(&buf).unwrap();
            buf.clear();

            mappings_context.current_source = source_index;
            mappings_context.current_original_line = self.line;

            let unfinished_generated_line = helpers::get_unfinished_lines(&self.generated_code);
            mappings_context.unfinished_generated_line = unfinished_generated_line;
            if lines > 0 {
                mappings += &line_mapping.repeat(lines.wrapping_sub(1));
            }

            if mappings_context.unfinished_generated_line == 0 {
                mappings += ";";
            } else if lines != 0 {
                mappings += line_mapping;
            }
            mappings
        }
    }

    pub fn get_normalized_nodes(self) -> Vec<SingleLineNode> {
        vec![self]
    }
}
