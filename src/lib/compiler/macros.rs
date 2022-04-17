use std::collections::HashMap;

use super::{
    parse::{DatabindParser, ParseResult, Rule},
    Compiler,
};
use crate::ast::Node;
use pest::Parser;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Macro {
    pub args: Vec<String>,
    pub contents: String,
}

impl Macro {
    pub fn expand_to_string(&self, args: &Vec<String>) -> String {
        let mut expanded = self.contents.clone();

        for i in 0..self.args.len() {
            expanded = expanded.replace(&format!("${}", self.args[i]), &args[i]);
        }

        expanded
    }

    pub fn expand_to_ast(
        &self,
        args: &Vec<String>,
        macros: &mut HashMap<String, Self>,
        subfolder: &str,
    ) -> ParseResult<Vec<Node>> {
        let expanded = self.expand_to_string(args);
        let tokens = DatabindParser::parse(Rule::file, &expanded)?
            .next()
            .unwrap();
        let parsed = Compiler::parse_tokens(&mut tokens.into_inner(), macros, subfolder)?;
        Ok(parsed)
    }
}
