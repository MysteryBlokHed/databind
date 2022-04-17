use super::{macros::Macro, Compiler};
use crate::ast::{AssignmentOp, Node};
use pest::{iterators::Pairs, Parser};
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "databind.pest"]
/// Pest's parser
pub(crate) struct DatabindParser;

pub type ParseResult<T> = Result<T, pest::error::Error<Rule>>;

impl Compiler {
    /// Convert the provided file contents into an AST
    pub fn parse(raw_file: &str) -> ParseResult<Vec<Node>> {
        let tokens = DatabindParser::parse(Rule::file, &raw_file)?
            .next()
            .unwrap();
        Compiler::parse_tokens(&mut tokens.into_inner(), &mut HashMap::new())
    }

    /// Convert the provided tokens into an AST
    pub(crate) fn parse_tokens(
        tokens: &mut Pairs<Rule>,
        macros: &mut HashMap<String, Macro>,
    ) -> ParseResult<Vec<Node>> {
        let mut ast = vec![];

        macro_rules! percent_escape {
            ($str: expr) => {
                if let Some(stripped) = $str.strip_prefix("%") {
                    stripped.into()
                } else {
                    $str.into()
                }
            };
        }

        macro_rules! unwrap_name {
            ($inner: expr) => {{
                // Get the name as str
                let as_str = $inner.next().unwrap().as_str();
                // Remove % prefix if present
                percent_escape!(as_str)
            }};
        }

        macro_rules! fix_escapes {
            ($str: expr) => {
                $str.replace("\\\\", "\\")
                    .replace("\\/", "/")
                    .replace("\\\"", "\"")
                    .replace("\\n", "\n")
                    .replace("\\r", "\r")
                    .replace("\\t", "\t")
            };
        }

        for token in tokens {
            println!("active token: {}", token);

            match token.as_rule() {
                /* Variables and objectives */
                Rule::new_var => {
                    let mut inner = token.into_inner();
                    let name = unwrap_name!(inner);
                    let value: i32 = inner.next().unwrap().as_str().parse().unwrap();
                    ast.push(Node::NewVar { name, value });
                }
                Rule::set_var => {
                    let mut inner = token.into_inner();
                    let name = unwrap_name!(inner);
                    let operator = match inner.next().unwrap().as_str() {
                        "=" => AssignmentOp::Set,
                        "+=" => AssignmentOp::Add,
                        "-=" => AssignmentOp::Subtract,
                        _ => unimplemented!(),
                    };
                    let value: i32 = inner.next().unwrap().as_str().parse().unwrap();
                    ast.push(Node::SetVar {
                        name,
                        operator,
                        value,
                    });
                }
                Rule::test_var => {
                    let mut inner = token.into_inner();
                    let name = unwrap_name!(inner);
                    let test = unwrap_name!(inner);
                    ast.push(Node::TestVar { name, test });
                }
                Rule::delete_var => {
                    let mut inner = token.into_inner();
                    let name = unwrap_name!(inner);
                    ast.push(Node::DeleteVar(name));
                }
                Rule::new_obj => {
                    let mut inner = token.into_inner();
                    let name = unwrap_name!(inner);
                    let objective = unwrap_name!(inner);
                    ast.push(Node::NewObjective { name, objective });
                }
                Rule::set_obj => {
                    let mut inner = token.into_inner();
                    let target = unwrap_name!(inner);
                    let name = unwrap_name!(inner);
                    let operator = match inner.next().unwrap().as_str() {
                        "=" => AssignmentOp::Set,
                        "+=" => AssignmentOp::Add,
                        "-=" => AssignmentOp::Subtract,
                        _ => unimplemented!(),
                    };
                    let value: i32 = inner.next().unwrap().as_str().parse().unwrap();
                    ast.push(Node::SetObjective {
                        target,
                        name,
                        operator,
                        value,
                    });
                }
                Rule::get_var => {
                    let mut inner = token.into_inner();
                    let name = unwrap_name!(inner);
                    ast.push(Node::GetVar(name));
                }
                /* Commands and functions */
                Rule::sbop | Rule::mc_command => {
                    let rule = token.as_rule();
                    let mut inner = token.into_inner();
                    let name = if let Rule::mc_command = rule {
                        unwrap_name!(inner)
                    } else {
                        "scoreboard".into()
                    };
                    let args = {
                        let mut args = if let Rule::sbop = rule {
                            vec![
                                Node::CommandArg("players".into()),
                                Node::CommandArg("operation".into()),
                            ]
                        } else {
                            vec![]
                        };
                        args.append(&mut Compiler::parse_tokens(&mut inner, macros)?);
                        args
                    };
                    ast.push(Node::MinecraftCommand { name, args });
                    println!("COMMAND ADDED JUST NOW: {:#?}", ast[ast.len() - 1]);
                }
                Rule::command_arg => {
                    let as_str = token.as_str();
                    println!("DA ARGY: {}", as_str);
                    ast.push(Node::CommandArg(if as_str == "%=" {
                        as_str.into()
                    } else {
                        percent_escape!(as_str)
                    }));
                }
                Rule::function => {
                    let mut inner = token.into_inner();
                    let name = unwrap_name!(inner);
                    let contents = Compiler::parse_tokens(&mut inner, macros)?;
                    ast.push(Node::Function { name, contents });
                }
                Rule::tag => {
                    let mut inner = token.into_inner();
                    ast.push(Node::Tag(unwrap_name!(inner)));
                }
                Rule::call_function => {
                    let mut inner = token.into_inner();
                    ast.push(Node::CallFunction(unwrap_name!(inner)));
                }
                /* Statements/loops */
                Rule::if_statement => {
                    let mut inner = token.into_inner();
                    let condition =
                        Compiler::parse_tokens(&mut inner.next().unwrap().into_inner(), macros)?;
                    let if_block =
                        Compiler::parse_tokens(&mut inner.next().unwrap().into_inner(), macros)?;
                    let else_block = if let Some(tokens) = inner.next() {
                        Compiler::parse_tokens(&mut tokens.into_inner(), macros)?
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
                    let condition =
                        Compiler::parse_tokens(&mut inner.next().unwrap().into_inner(), macros)?;
                    let contents =
                        Compiler::parse_tokens(&mut inner.next().unwrap().into_inner(), macros)?;
                    ast.push(Node::WhileLoop {
                        condition,
                        contents,
                    });
                }
                Rule::macro_def => {
                    let mut inner = token.into_inner();
                    let name = unwrap_name!(inner);
                    let args = inner
                        .next()
                        .unwrap()
                        .into_inner()
                        .map(|x| x.as_str().into())
                        .collect();
                    let contents = unwrap_name!(inner);

                    // Add def to list of macros
                    macros.insert(name, Macro::new(args, contents));
                }
                Rule::macro_call => {
                    let mut inner = token.into_inner();
                    let name: String = unwrap_name!(inner);
                    let args: Vec<String> = inner
                        .map(|x| fix_escapes!(x.into_inner().as_str()).into())
                        .collect();

                    let macro_def = macros
                        .get(&name)
                        .clone()
                        .expect(&format!("No macro definition found for call of {}", name))
                        .clone();

                    // Expand macro call
                    // This ends up happening recursively since parse_tokens is recalled for every new nested call
                    // Also, we don't have to worry about adding definitions since we pass the reference to
                    // the HashMap of macro definitions!
                    println!("expanding call");
                    let mut expanded = macro_def.expand_to_ast(&args, macros)?;
                    println!("expanded: {:#?}", expanded);
                    ast.append(&mut expanded);
                }
                Rule::trustme => {
                    let mut inner = token.into_inner();
                    let content = unwrap_name!(inner);
                    ast.push(Node::TrustMe(content));
                }
                Rule::EOI => break,
                _ => todo!(),
            }
        }

        Ok(ast)
    }
}
