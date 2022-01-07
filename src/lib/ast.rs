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
//! Contains enums for the AST of Databind
#[derive(Clone, Debug, PartialEq)]
pub enum AssignmentOp {
    Add,
    Subtract,
    Set,
}

/// The main enum for Databind's AST
#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    NewVar {
        name: String,
        value: i32,
    },
    SetVar {
        name: String,
        operator: AssignmentOp,
        value: i32,
    },
    TestVar {
        name: String,
        test: String,
    },
    DeleteVar(String),
    NewObjective {
        name: String,
        objective: String,
    },
    SetObjective {
        target: String,
        name: String,
        operator: AssignmentOp,
        value: i32,
    },
    GetVar(String),
    Function {
        name: String,
        contents: Vec<Node>,
    },
    Tag(String),
    CallFunction(String),
    IfStatement {
        condition: Vec<Node>,
        if_block: Vec<Node>,
        else_block: Vec<Node>,
    },
    WhileLoop {
        condition: Vec<Node>,
        contents: Vec<Node>,
    },
    MacroDefinition {
        name: String,
        args: Vec<String>,
        contents: String,
    },
    MacroCall {
        name: String,
        args: Vec<String>,
    },
    MinecraftCommand {
        name: String,
        args: Vec<Node>,
    },
    CommandArg(String),
    TrustMe(String),
}

impl Node {
    pub fn run_all_nodes(ast: &Vec<Self>, target_fn: &mut dyn FnMut(&Vec<Self>)) {
        macro_rules! run {
            ($ast: expr) => {{
                target_fn($ast);
                Self::run_all_nodes($ast, target_fn);
            }};
        }

        for node in ast {
            match node {
                Node::Function { contents, .. } => run!(contents),
                Node::IfStatement {
                    condition,
                    if_block,
                    else_block,
                } => {
                    run!(condition);
                    run!(if_block);
                    run!(else_block);
                }
                Node::WhileLoop {
                    condition,
                    contents,
                } => {
                    run!(condition);
                    run!(contents);
                }
                Node::MinecraftCommand { args, .. } => run!(args),
                _ => {}
            }
        }
    }

    /// Mutably runs a given function on all lists of nodes in the AST
    ///
    /// ## Arguments
    ///
    /// * `ast` - The AST to run the function on
    /// * `target_fn` - The function to pass the node lists to
    pub fn run_all_nodes_mut(ast: &mut Vec<Self>, target_fn: &mut dyn FnMut(&mut Vec<Self>)) {
        for node in ast {
            macro_rules! run {
                ($target_ast: expr) => {{
                    println!("CALLING WITH AST {:#?}", $target_ast);
                    target_fn($target_ast);
                    Self::run_all_nodes_mut($target_ast, target_fn);
                }};
            }

            println!("ON NODE {:#?}", &node);
            match node {
                Node::Function { contents, .. } => run!(contents),
                Node::IfStatement {
                    condition,
                    if_block,
                    else_block,
                } => {
                    run!(condition);
                    run!(if_block);
                    run!(else_block);
                }
                Node::WhileLoop {
                    condition,
                    contents,
                } => {
                    run!(condition);
                    run!(contents);
                }
                Node::MinecraftCommand { args, .. } => run!(args),
                _ => {}
            }
        }
    }

    /// Removes all instances of a node in an AST
    ///
    /// ## Arguments
    ///
    /// * `ast` - The AST to remove from
    /// * `check_fn` - A function you must implement, probably with a `match` or `if let` statement.
    ///   Should check the passed Node and return true if it is a Node you're looking for,
    ///   and false otherwise
    pub fn remove_nodes(ast: &Vec<Self>, check_fn: &dyn Fn(&Self) -> bool) -> Vec<Self> {
        let mut new_ast: Vec<_> = ast.iter().filter(|&x| check_fn(x)).cloned().collect();

        macro_rules! add_removed {
            ($ast: expr) => {
                new_ast.append(&mut Self::remove_nodes($ast, check_fn))
            };
        }

        for node in ast.iter() {
            match node {
                Node::Function { contents, .. } => {
                    add_removed!(contents);
                }
                Node::IfStatement {
                    condition,
                    if_block,
                    else_block,
                } => {
                    add_removed!(condition);
                    add_removed!(if_block);
                    add_removed!(else_block);
                }
                Node::WhileLoop {
                    condition,
                    contents,
                } => {
                    add_removed!(condition);
                    add_removed!(contents);
                }
                Node::MinecraftCommand { args, .. } => add_removed!(args),
                _ => {}
            }
        }

        new_ast
    }

    /// Checks all nodes in an AST
    ///
    /// ## Arguments
    ///
    /// * `ast` - The AST to check
    /// * `check_fn` - A function you must implement, probably with a `match` or `if let` statement.
    ///   Should check the passed Node and return true if it is a Node you're looking for,
    ///   and false otherwise
    pub fn check_for_node(ast: &Vec<Self>, check_fn: &dyn Fn(&Self) -> bool) -> bool {
        if ast.iter().any(check_fn) {
            return true;
        }

        macro_rules! check {
            ($ast: expr) => {
                if Self::check_for_node($ast, check_fn) {
                    return true;
                }
            };
        }

        for node in ast {
            match node {
                Node::Function { contents, .. } => check!(contents),
                Node::IfStatement {
                    condition,
                    if_block,
                    else_block,
                } => {
                    check!(condition);
                    check!(if_block);
                    check!(else_block);
                }
                Node::WhileLoop {
                    condition,
                    contents,
                } => {
                    check!(condition);
                    check!(contents);
                }
                Node::MinecraftCommand { args, .. } => check!(args),
                _ => {}
            }
        }

        false
    }
}
