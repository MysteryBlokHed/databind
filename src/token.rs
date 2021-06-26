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
#[derive(Clone, Debug, PartialEq)]
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
    /// Meant for scoreboard operations (`:varop`)
    GetVar,
    /// Define a text replacement
    DefineReplace,
    /// The name of a replacement
    ReplaceName(String),
    /// The contents of a replacement
    ReplaceContents(String),
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
    /// Shorthand for `scoreboard players operation`
    ScoreboardOperation,
    /// An integer
    Int(i32),
    /// Commands, etc. that are not by databind
    ///
    /// In the command `execute if :tvar variable #etc`
    /// `execute if ` would be tokenized as NonDatabind.
    NonDatabind(String),
    /// A new line
    NewLine,
    None,
}
