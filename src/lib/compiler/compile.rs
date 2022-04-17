use super::{parse::ParseResult, Compiler};
use crate::ast::{AssignmentOp, Node};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Compiled {
    files: HashMap<String, String>,
    tags: HashMap<String, Vec<String>>,
}

impl Compiler {
    pub fn nodes_to_text(nodes: &Vec<Node>, namespace: Option<&str>) -> String {
        Compiler::compile_ast(
            nodes,
            &mut HashMap::new(),
            &mut HashMap::new(),
            &mut vec![String::new()],
            "",
            namespace,
        )["".into()]
        .clone()
    }

    pub fn compile_ast<'a>(
        ast: &Vec<Node>,
        files: &'a mut HashMap<String, String>,
        tag_map: &'a mut HashMap<String, Vec<String>>,
        nested_funcs: &mut Vec<String>,
        subfolder: &str,
        namespace: Option<&str>,
    ) -> &'a mut HashMap<String, String> {
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

        let ast = Compiler::replace_if_while(ast, subfolder);

        for node in ast {
            match node {
                #[rustfmt::skip]
                Node::NewVar { name, value } => {
                    current_file!().push_str( &format!("scoreboard objectives add {} dummy\n", name));
                    current_file!().push_str( &format!("scoreboard players set --databind {} {}\n", name, value));
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
                    current_file!().push_str(&format!(" score --databind {} {} ", name, test));
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

                Node::GetVar(name) => current_file!().push_str(&format!("--databind {} ", name)),

                Node::Function { name, contents } => {
                    nested_funcs.push(name.clone());
                    files.insert(name.clone(), String::new());
                    Compiler::compile_ast(
                        &contents,
                        files,
                        tag_map,
                        nested_funcs,
                        subfolder,
                        namespace,
                    );
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

                Node::MinecraftCommand { name, args } => {
                    current_file!().push_str(&format!(
                        "{}{}\n",
                        name,
                        Compiler::nodes_to_text(&args, namespace)
                    ));
                }

                Node::CommandArg(arg) => current_file!().push_str(&format!(" {}", arg)),

                Node::TrustMe(content) => current_file!().push_str(&content),

                Node::IfStatement { .. }
                | Node::WhileLoop { .. }
                | Node::MacroDefinition { .. }
                | Node::MacroCall { .. } => {
                    panic!("An if statement, while loop, or macro definition/call was accidentally passed directly to compile");
                }
            }
        }

        files
    }

    pub fn compile(raw_file: &str, subfolder: &str) -> ParseResult<Compiled> {
        let mut files: HashMap<String, String> = HashMap::new();
        let mut tags: HashMap<String, Vec<String>> = HashMap::new();

        let parsed = Compiler::parse(raw_file)?;
        Compiler::compile_ast(
            &parsed,
            &mut files,
            &mut tags,
            &mut Vec::new(),
            subfolder,
            None,
        );

        Ok(Compiled { files, tags })
    }
}
