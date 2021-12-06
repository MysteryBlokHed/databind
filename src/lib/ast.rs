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
}
