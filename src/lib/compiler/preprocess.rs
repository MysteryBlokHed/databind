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
use super::{macros::Macro, parse::Rule, Compiler};
use crate::ast::*;
use rand::{distributions::Alphanumeric, Rng};
use std::collections::HashMap;

/// Whether a function to set up an if statement objective has been created
static mut IF_INIT_CREATED: bool = false;

impl Compiler {
    /// Replace macro calls with the contents of a macro.
    /// Recursively calls itself until no macro calls are left
    /// in case a macro definition contains a macro call
    ///
    /// # Arguments
    ///
    /// - `ast` - The AST to look for macro calls in
    /// - `return_macros` - Whether to return the HashMap of macros.
    ///   Used for global macros (files beginning with `!`)
    /// - `existing_macros` - Global macros to use
    pub fn parse_macros(
        &self,
        ast: &Vec<Node>,
        return_macros: bool,
        existing_macros: &HashMap<String, Macro>,
    ) -> Result<(Vec<Node>, Option<HashMap<String, Macro>>), pest::error::Error<Rule>> {
        let mut new_ast = Vec::new();
        let mut macros = existing_macros.clone();

        println!("new call to parse macros with ast: {:#?}", ast);

        for node in ast {
            println!("node: {:#?}", node);

            match node {
                Node::MacroDefinition {
                    name,
                    args,
                    contents,
                } => {
                    println!("adding macro {}", name);
                    macros.insert(name.clone(), Macro::new(args.clone(), contents.clone()));
                }
                Node::MacroCall { name, args } => {
                    println!("macro call");
                    if !macros.contains_key(name) {
                        println!("{} does not exist yet", name);
                        println!("passed ast: {:#?}", ast);
                        println!("new ast: {:#?}", new_ast);

                        let node_call_check = |x: &Node| {
                            if let Node::MacroDefinition { name: def_name, .. } = x {
                                println!("definition!");
                                if def_name == name {
                                    true
                                } else {
                                    false
                                }
                            } else {
                                println!("not a definition...");
                                false
                            }
                        };

                        if Node::check_for_node(&new_ast, &node_call_check)
                            || Node::check_for_node(ast, &node_call_check)
                        {
                            println!("moving to next ast");
                            new_ast.push(node.clone());
                            continue;
                        } else {
                            panic!("no macro found with name {}", name);
                        }
                    }

                    println!("available macros: {:#?}", macros);
                    let text = macros[name].replace(args);
                    println!("parsing text: {}", text);
                    let mut macro_ast = self.parse(&text)?;
                    // let mut macro_ast = {
                    //     let compiler = Compiler::new(None);
                    //     compiler.parse(&text)?
                    // };
                    println!("did it work?");
                    println!("ADDING TO NEW AST: {:#?}", macro_ast);
                    new_ast.append(&mut macro_ast);
                }
                _ => new_ast.push(node.clone()),
            }
        }

        let contains_macros = Node::check_for_node(&new_ast, &|x| match x {
            Node::MacroDefinition { .. } | Node::MacroCall { .. } => true,
            _ => false,
        });

        if contains_macros {
            Node::run_all_nodes(&mut new_ast, &mut |nodes| {
                let (new_nodes, new_macros) = self.parse_macros(nodes, true, &macros).unwrap();
                *nodes = new_nodes;
                macros.extend(new_macros.unwrap());
            });
        }

        Ok((new_ast, if return_macros { Some(macros) } else { None }))
    }

    /// Replace while loops and if statements. Called recursively until none are left
    ///
    /// # Arguments
    ///
    /// - `tokens` - A list of tokens to look for while loops or if statements in
    /// - `subfolder` - If the while loop is in a subfolder, the prefix
    ///   to put before the function name (eg. `"cmd/"` for a subfolder named `cmd`)
    pub fn replace_if_and_while(&self, ast: &Vec<Node>, subfolder: &str) -> Vec<Node> {
        let mut new_ast = Vec::new();

        let mut chars = Compiler::random_chars();

        /// Turn an `&str` into `ast::Node::CommandArg` for shorter lines
        macro_rules! command_arg {
            ($str: expr) => {
                Node::CommandArg($str.into())
            };
        }

        for node in ast {
            match node {
                Node::WhileLoop {
                    condition,
                    contents,
                } => {
                    // Args for execute command in main while loop function
                    let loop_main_args = {
                        let mut vec = vec![command_arg!("if")];
                        vec.append(&mut condition.clone());
                        vec.push(Node::CallFunction(format!(
                            "{}condition_{}",
                            subfolder, chars
                        )));
                        vec
                    };

                    // Main while loop function
                    let loop_main = Node::Function {
                        name: format!("while_{}", chars),
                        contents: vec![Node::MinecraftCommand {
                            name: "execute".into(),
                            args: loop_main_args,
                        }],
                    };

                    // Contents of function for while loop condition
                    let loop_condition_contents = {
                        let mut vec = vec![];
                        vec.append(&mut contents.clone());
                        vec.push(Node::CallFunction(format!("{}while_{}", subfolder, chars)));
                        vec
                    };

                    // While loop condition function
                    let loop_condition = Node::Function {
                        name: format!("condition_{}", chars),
                        contents: loop_condition_contents,
                    };

                    // Call to while loop function
                    let call = Node::CallFunction(format!("{}while_{}", subfolder, chars));

                    new_ast.append(&mut vec![loop_main, loop_condition, call]);
                    chars = Compiler::random_chars();
                }
                Node::IfStatement {
                    condition,
                    if_block,
                    else_block,
                } => {
                    // Unsafe due to check of IF_INIT_CREATED
                    // A function that simply creates a dummy objective called db_if_res,
                    // used to store the results of if statements
                    let if_init_function = unsafe {
                        if !IF_INIT_CREATED {
                            let if_init_function = Node::Function {
                                name: "if_init".into(),
                                contents: vec![
                                    Node::Tag("load".into()),
                                    Node::NewObjective {
                                        name: "db_if_res".into(),
                                        objective: "dummy".into(),
                                    },
                                ],
                            };

                            IF_INIT_CREATED = true;

                            Some(if_init_function)
                        } else {
                            None
                        }
                    };

                    // Returns an execute command that evaluates the if condition and stores the result
                    let mut if_store_value = |result: bool| {
                        let mut args = vec![command_arg!(if result { "if" } else { "unless" })];
                        args.append(&mut condition.clone());
                        args.push(command_arg!("run"));
                        args.push(Node::SetObjective {
                            target: format!("--databind-{}", chars),
                            name: "db_if_res".into(),
                            operator: AssignmentOp::Set,
                            value: if result { 1 } else { 0 },
                        });
                        Node::MinecraftCommand {
                            name: "execute".into(),
                            args,
                        }
                    };

                    let check_true = if_store_value(true);
                    let check_false = if_store_value(false);

                    let if_call_function = |result: bool| {
                        let args = vec![
                            command_arg!("if"),
                            command_arg!("score"),
                            Node::CommandArg(format!("--databind-{}", chars)),
                            command_arg!("db_if_res"),
                            command_arg!("matches"),
                            command_arg!(if result { "1" } else { "0" }),
                            command_arg!("run"),
                            command_arg!("call"),
                            Node::CommandArg(format!("{}if_{}_{}", subfolder, result, chars)),
                        ];

                        Node::MinecraftCommand {
                            name: "execute".into(),
                            args,
                        }
                    };

                    let if_true_call = if_call_function(true);
                    let if_false_call = if_call_function(false);

                    let if_true_function = Node::Function {
                        name: format!("{}if_true_{}", subfolder, chars),
                        contents: if_block.clone(),
                    };

                    let if_false_function = if !else_block.is_empty() {
                        Some(Node::Function {
                            name: format!("{}if_false_{}", subfolder, chars),
                            contents: else_block.clone(),
                        })
                    } else {
                        None
                    };

                    if let Some(if_init) = if_init_function {
                        new_ast.push(if_init)
                    }
                    new_ast.push(check_true);
                    new_ast.push(check_false);
                    new_ast.push(if_true_function);
                    new_ast.push(if_true_call);
                    if let Some(if_false) = if_false_function {
                        new_ast.push(if_false);
                        new_ast.push(if_false_call);
                    }
                    chars = Compiler::random_chars();
                }
                _ => new_ast.push(node.clone()),
            }
        }

        // Run recursively until no if statements, while loops,
        // or scoreboard operations are left
        if new_ast.iter().any(|x| match x {
            Node::IfStatement { .. } | Node::WhileLoop { .. } => true,
            _ => false,
        }) {
            self.replace_if_and_while(&new_ast, subfolder)
        } else {
            new_ast
        }
    }

    /// Return a random string of 4 lowercase alphanumeric characters
    fn random_chars() -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(4)
            .map(char::from)
            .collect::<String>()
            .to_lowercase()
    }
}
