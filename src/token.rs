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
//! Contains the enum of tokens used for tokenization of Databind source files
#[derive(Clone, Debug, PartialEq)]
/// The enum of Databind tokens used for tokenization of Databind source files
pub enum Token {
    /// Used before a mention of a variable (`Token::VarName`)
    Var,
    /// Used before an objective definition
    Objective,
    /// Used before an objective modification
    SetObjective,
    /// Used before a variable (`Token::VarName`) to test
    TestVar,
    /// Used before a variable (`Token::VarName`) to get.
    /// Meant for scoreboard operations (`sbop`)
    GetVar,
    /// Define a function
    DefineFunc,
    /// The name of a function
    FuncName(String),
    /// End a function definition
    EndFunc,
    /// Add a tag to a function
    Tag,
    /// The name of a tag
    TagName(String),
    /// Call a funcition
    CallFunc,
    /// Start an if statement
    IfStatement,
    /// The condition for an if statement
    IfCondition(String),
    /// The contents of an if or else statement
    IfContents(String),
    /// An else statement if the if statement condition
    /// was not true
    ElseStatement,
    /// Close an if statement
    EndIf,
    /// Start a while loop
    WhileLoop,
    /// The condition for a while loop
    WhileCondition(String),
    /// The string contents of a while loop
    WhileContents(String),
    /// Close a while loop
    EndWhileLoop,
    /// A variable's name for modifying
    ModVarName(String),
    /// A variable's name for testing
    TestVarName(String),
    /// A variable's name for deleting
    DelVarName(String),
    /// A variable's name for scoreboard operations
    OpVarName(String),
    /// An objective's name
    ObjectiveName(String),
    /// An objective's type (eg. deathCount)
    ObjectiveType(String),
    /// Delete a variable or objective
    DeleteVar,
    /// A targeted entity (eg. `Username` or `@a`)
    Target(String),
    /// Set the initial value of a variable
    InitialSet,
    /// Set the value of a variable or objective
    VarSet,
    /// Add to the value of a variable or objective
    VarAdd,
    /// Subtract from the value of a variable or objective
    VarSub,
    /// An integer
    Int(i32),
    /// Define a Databind macro
    DefineMacro,
    /// Call a Databind macro
    CallMacro,
    /// The name of a Databind macro
    ///
    /// The first `usize` is for the line with the macro name,
    /// and the second is for the column
    MacroName(String, usize, usize),
    /// The contents of a macro
    MacroContents(String),
    /// Close a Databind macro definition
    EndMacro,
    /// A list of either argument names for a macro definition
    DefArgList(Vec<String>),
    /// A list of argument values for a macro call
    CallArgList(Vec<String>),
    /// Commands, etc. that are not by databind
    ///
    /// In the command `execute if :tvar variable #etc`
    /// `execute if ` would be tokenized as NonDatabind.
    NonDatabind(String),
    /// A new line
    NewLine,
    None,
}
