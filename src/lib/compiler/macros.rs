/*
 * Databind - Expand the functionality of Minecraft Datapacks.
 * Copyright (C) 2021  Adam Thompson-Sharpe
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
use super::{
    parse::{DatabindParser, ParseResult, Rule},
    Compiler,
};
use crate::ast::Node;
use pest::Parser;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Macro {
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
