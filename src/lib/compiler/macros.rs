use crate::ast::Node;

use super::{parse::ParseResult, Compiler};

pub struct Macro {
    args: Vec<String>,
    contents: String,
}

impl Macro {
    pub fn new(args: Vec<String>, contents: String) -> Self {
        Self { args, contents }
    }

    pub fn expand_to_string(&self, args: &Vec<&str>) -> String {
        let mut expanded = self.contents.clone();

        for i in 0..self.args.len() {
            expanded = expanded.replace(&format!("${}", self.args[i]), &args[i]);
        }

        expanded
    }

    pub fn expand_to_ast(&self, args: &Vec<&str>) -> ParseResult<Vec<Node>> {
        let expanded = self.expand_to_string(args);
        let parsed = Compiler::parse(&expanded)?;
        Ok(parsed)
    }
}
