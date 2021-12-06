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
use super::Compiler;
use crate::ast::*;
use pest::{iterators::Pairs, Parser};

#[derive(Parser)]
#[grammar = "databind.pest"]
pub struct DatabindParser;

pub type ParseReturn = Result<Vec<Node>, pest::error::Error<Rule>>;

impl Compiler {
    /// Convert the provided file contents into an AST
    pub fn parse(&self, raw_file: &str) -> ParseReturn {
        let tokens = DatabindParser::parse(Rule::file, &raw_file)?
            .next()
            .unwrap();
        self.parse_tokens(&mut tokens.into_inner())
    }

    /// Convert the provided tokens into an AST
    pub fn parse_tokens(&self, tokens: &mut Pairs<Rule>) -> ParseReturn {
        let mut ast = vec![];

        println!("{:?}", tokens);

        for token in tokens {
            match token.as_rule() {
                Rule::mc_command => {
                    let mut inner = token.into_inner();
                    let name = inner.next().unwrap().as_str().into();
                    let args = self.parse_tokens(&mut inner)?;
                    ast.push(Node::MinecraftCommand { name, args });
                }
                Rule::function => {
                    let mut inner = token.into_inner();
                    let name = inner.next().unwrap().as_str().into();
                    let contents = self.parse_tokens(&mut inner)?;
                    ast.push(Node::Function { name, contents });
                }
                Rule::tag => {
                    let mut inner = token.into_inner();
                    ast.push(Node::Tag(inner.next().unwrap().as_str().into()));
                }
                Rule::call_function => {
                    let mut inner = token.into_inner();
                    ast.push(Node::CallFunction(inner.next().unwrap().as_str().into()));
                }
                Rule::if_statement => {
                    let mut inner = token.into_inner();
                    let condition = self.parse_tokens(&mut inner.next().unwrap().into_inner())?;
                    let if_block = self.parse_tokens(&mut inner.next().unwrap().into_inner())?;
                    let else_block = if let Some(tokens) = inner.next() {
                        self.parse_tokens(&mut tokens.into_inner())?
                    } else {
                        vec![]
                    };
                    ast.push(Node::IfStatement {
                        condition,
                        if_block,
                        else_block,
                    });
                }
                Rule::while_loop => {
                    let mut inner = token.into_inner();
                    let condition = self.parse_tokens(&mut inner.next().unwrap().into_inner())?;
                    let contents = self.parse_tokens(&mut inner)?;
                    ast.push(Node::WhileLoop {
                        condition,
                        contents,
                    });
                }
                Rule::command_arg => {
                    // let mut inner = token.into_inner();
                    ast.push(Node::CommandArg(token.as_str().into()));
                }
                Rule::EOI => break,
                _ => todo!(),
            }
        }

        Ok(ast)
    }
}
