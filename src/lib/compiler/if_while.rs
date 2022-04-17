use super::Compiler;
use crate::ast::{AssignmentOp, Node};
use rand::{distributions::Alphanumeric, Rng};

static mut IF_INIT_CREATED: bool = false;

/// Macro to turn `&str` into `Node::CommandArg` for readability
macro_rules! command_arg {
    ($str: expr) => {
        Node::CommandArg($str.into())
    };
}

pub(crate) struct IfStatement {
    condition: Vec<Node>,
    if_block: Vec<Node>,
    else_block: Option<Vec<Node>>,
}

pub(crate) struct WhileLoop {
    condition: Vec<Node>,
    contents: Vec<Node>,
}

impl Compiler {
    /// Convert an if statement into a
    pub(crate) fn convert_if(statement: &IfStatement, subfolder: &str) -> Vec<Node> {
        let condition = statement.condition;
        let if_block = statement.if_block;
        let else_block = statement.else_block;

        let mut ast = vec![];

        let chars = Compiler::random_chars();

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

        let if_false_function = if let Some(else_contents) = else_block {
            Some(Node::Function {
                name: format!("{}if_false_{}", subfolder, chars),
                contents: else_contents.clone(),
            })
        } else {
            None
        };

        if let Some(if_init) = if_init_function {
            ast.push(if_init)
        }
        ast.push(check_true);
        ast.push(check_false);
        ast.push(if_true_function);
        ast.push(if_true_call);
        if let Some(if_false) = if_false_function {
            ast.push(if_false);
            ast.push(if_false_call);
        }

        ast
    }

    pub(crate) fn convert_while(while_loop: &WhileLoop, subfolder: &str) -> Vec<Node> {
        let condition = while_loop.condition;
        let contents = while_loop.contents;

        let mut ast = vec![];

        let chars = Compiler::random_chars();

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

        ast.append(&mut vec![loop_main, loop_condition, call]);

        ast
    }

    /// Return a random string of 4 lowercase alphanumeric characters
    pub(crate) fn random_chars() -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(4)
            .map(char::from)
            .collect::<String>()
            .to_lowercase()
    }
}
