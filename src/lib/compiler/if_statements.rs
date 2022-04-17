use super::Compiler;
use crate::ast::{AssignmentOp, Node};
use rand::{distributions::Alphanumeric, Rng};

static mut IF_INIT_CREATED: bool = false;

impl Compiler {
    pub fn replace_if_while(ast: &Vec<Node>, subfolder: &str) -> Vec<Node> {
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
                    let if_store_value = |result: bool| {
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
            Compiler::replace_if_while(&new_ast, subfolder)
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
