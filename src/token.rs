#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    /// Used before a mention of a variable (`Token::VarName`)
    Var,
    /// Define a replacement (`:def`)
    CreateDef,
    GetDef,
    /// Used before an objective definition
    Objective,
    /// Used before an objective modification
    SetObjective,
    /// Used before a variable (`Token::VarName`) to test
    TestVar,
    /// Define a function
    DefineFunc,
    /// The name of a function
    FuncName(String),
    /// End a function definition
    EndFunc,
    CallFunc,
    // /// The created def
    // CreatedDef([String; 2]),
    VarName(String),
    // /// Get a replacement
    // DefName(String),
    ObjectiveName(String),
    ObjectiveType(String),
    Target(String),
    /// Set the initial value of a variable (`.=`)
    InitialSet,
    /// Set the value of a variable (`=`)
    VarSet,
    /// Add to the value of a variable (`+=`)
    VarAdd,
    /// Subtract from the value of a variable (`-=`)
    VarSub,
    Int(i32),
    /// Commands, etc. that are not by databind
    /// (eg. `execute if `)
    NonDatabind(String),
    NewLine,
    None,
}
