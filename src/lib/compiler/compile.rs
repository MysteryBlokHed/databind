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
use crate::ast::{AssignmentOp, Node};
use crate::compiler::macros::Macro;
use crate::types::{GlobalMacros, TagMap};
use regex::Regex;
use std::collections::HashMap;

/// Return from the compiler
///
/// # Arguments
///
/// - `filename_map` - A map of filenames and file contents
/// - `tag_map` - A map of tags to functions
#[derive(Debug, Clone)]
pub struct CompileReturn {
    pub files: HashMap<String, String>,
    pub tag_map: TagMap,
    pub global_macros: Option<GlobalMacros>,
}

impl Compiler {
    /// Calls a function for every node in the AST containing a list of other nodes,
    /// such as functions
    ///
    /// ## Arguments
    ///
    /// * `ast` - The AST to modify
    /// * `to_run` - The function to pass the lists of nodes. Should return a potentially modified version
    ///   of the original argument passed
    pub fn update_node_lists(
        &self,
        ast: &mut Vec<Node>,
        to_run: &mut dyn FnMut(&Vec<Node>) -> Vec<Node>,
    ) {
        *ast = to_run(ast);

        for node in ast {
            match node {
                Node::Function { name, contents } => *contents = to_run(&contents),
                Node::IfStatement {
                    condition,
                    if_block,
                    else_block,
                } => {
                    *condition = to_run(&condition);
                    *if_block = to_run(&if_block);
                    *else_block = to_run(&else_block);
                }
                Node::WhileLoop {
                    condition,
                    contents,
                } => {
                    *condition = to_run(&condition);
                    *contents = to_run(&contents);
                }
                Node::MinecraftCommand { name, args } => *args = to_run(&args),
                _ => {}
            }
        }
    }

    /// Convert an AST to text. Called recursively for nodes containing other nodes
    ///
    /// ## Arguments
    ///
    /// * `ast` - The AST to compile to text
    /// * `tag_map` - A map of function names to their tags
    /// * `files` - A map of files and their contents
    /// * `nested_funcs` - A list with the current nesting of functions
    fn ast_to_text<'a>(
        &self,
        ast: &Vec<Node>,
        tag_map: &mut HashMap<String, Vec<String>>,
        files: &'a mut HashMap<String, String>,
        nested_funcs: &mut Vec<String>,
        namespace: Option<&str>,
    ) -> &'a mut HashMap<String, String> {
        println!("RUNNING WITH AST {:#?}", ast);

        if files.is_empty() {
            files.insert(String::new(), String::new());
        }

        /// Get the name of the current function
        macro_rules! current_func {
            () => {
                &nested_funcs[nested_funcs.len() - 1]
            };
        }

        /// Get the contents of the current file
        macro_rules! current_file {
            () => {
                files.get_mut(current_func!()).unwrap()
            };
        }

        /// Get text instead of a list of files from ast_to_text
        macro_rules! nodes_to_single_text {
            ($nodes: expr) => {
                self.ast_to_text(
                    $nodes,
                    &mut HashMap::new(),
                    &mut HashMap::new(),
                    &mut vec!["".into()],
                    namespace,
                )["".into()]
            };
        }

        for node in ast {
            match node {
                #[rustfmt::skip]
                Node::NewVar { name, value } => {
                    current_file!().push_str( &format!("scoreboard objectives add {} dummy\n", name));
                    current_file!() .push_str( &format!("scoreboard players set --databind {} {}\n", name, value));
                }
                Node::SetVar {
                    name,
                    operator,
                    value,
                } => {
                    let action = match operator {
                        AssignmentOp::Add => "add",
                        AssignmentOp::Subtract => "remove",
                        AssignmentOp::Set => "set",
                    };

                    current_file!().push_str(&format!(
                        "scoreboard players {} --databind {} {}\n",
                        action, name, value
                    ));
                }
                Node::TestVar { name, test } => {
                    current_file!().push_str(&format!("score --databind {} {} ", name, test));
                }
                Node::DeleteVar(name) => {
                    current_file!().push_str(&format!("scoreboard objectives remove {}\n", name));
                }
                Node::NewObjective { name, objective } => {
                    current_file!().push_str(&format!(
                        "scoreboard objectives add {} {}\n",
                        name, objective
                    ));
                }
                Node::SetObjective {
                    target,
                    name,
                    operator,
                    value,
                } => {
                    let action = match operator {
                        AssignmentOp::Add => "add",
                        AssignmentOp::Subtract => "remove",
                        AssignmentOp::Set => "set",
                    };

                    current_file!().push_str(&format!(
                        "scoreboard players {} {} {} {}\n",
                        action, target, name, value,
                    ));
                }
                Node::Sbop => current_file!().push_str("scoreboard players operation "),
                Node::GetVar(name) => current_file!().push_str(&format!("--databind {} ", name)),
                Node::Function { name, contents } => {
                    nested_funcs.push(name.clone());
                    files.insert(name.clone(), String::new());
                    self.ast_to_text(contents, tag_map, files, nested_funcs, namespace);
                    nested_funcs.pop();
                }
                Node::Tag(tag) => tag_map
                    .entry(tag.clone())
                    .or_insert(Vec::new())
                    .push(current_func!().clone()),
                Node::CallFunction(name) => {
                    // Function contains namespace
                    let has_namespace = name.contains(':');

                    if has_namespace {
                        current_file!().push_str(&format!("function {}\n", name));
                    } else if let Some(ns) = namespace {
                        current_file!().push_str(&format!("function {}:{}\n", ns, name));
                    } else {
                        panic!("internal: no namespace provided for function call");
                    }
                }
                Node::IfStatement { .. }
                | Node::WhileLoop { .. }
                | Node::MacroDefinition { .. }
                | Node::MacroCall { .. } => unimplemented!(),
                Node::MinecraftCommand { name, args } => {
                    current_file!().push_str(&format!("{}{}\n", name, nodes_to_single_text!(args)));
                }
                Node::CommandArg(arg) => current_file!().push_str(&format!(" {}", arg)),
            }
        }

        files
    }

    /// Convert an AST to a compiled file or files
    ///
    /// # Arguments
    ///
    /// - `ast` - The AST to compile
    /// - `namespace` - The namespace to use for functions, if relevant
    pub fn compile(
        &self,
        ast: &mut Vec<Node>,
        namespace: Option<&str>,
        subfolder: &str,
        global_macros: &HashMap<String, Macro>,
        return_macros: bool,
    ) -> CompileReturn {
        let mut returned_macros = HashMap::new();

        println!("COMPILE START: {:#?}", ast);

        // Replace all while loops/if statements with functions
        self.update_node_lists(ast, &mut |nodes| {
            self.replace_if_and_while(nodes, subfolder)
        });

        println!("WHILES REPLACED: {:#?}", ast);

        // Replace all macros with expanded versions
        self.update_node_lists(ast, &mut |nodes| {
            let ret = self
                .parse_macros(nodes, return_macros, global_macros)
                .unwrap();
            if let Some(macros) = ret.1 {
                returned_macros.extend(macros);
            }
            ret.0
        });

        // // Parse macros if there are any calls
        // let tokens = if ast.contains(&Node::CallMacro) || return_macros {
        //     let ret = self.parse_macros(ast, return_macros, global_macros);
        //     returned_macros = if return_macros { ret.1 } else { None };
        //     ret.0
        // } else {
        //     returned_macros = None;
        //     ast
        // };

        let mut tag_map = HashMap::new();
        let mut files = HashMap::new();
        let mut nested_funcs = vec![];

        self.ast_to_text(&ast, &mut tag_map, &mut files, &mut nested_funcs, namespace);

        // Remove leading/trailing whitespace from files as well as empty lines
        let re = Regex::new(r"\n{2,}").unwrap();
        for (_, contents) in files.iter_mut() {
            *contents = contents.trim().to_string();
            if contents.contains("\n\n") {
                *contents = re.replace_all(contents, "\n").into();
            }
        }

        files.remove("".into());

        CompileReturn {
            files,
            tag_map,
            global_macros: if return_macros {
                Some(returned_macros)
            } else {
                None
            },
        }
    }
}
