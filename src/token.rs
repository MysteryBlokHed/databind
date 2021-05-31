#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    /// Define a variable (`:var`)
    Var,
    /// Define a replacement (`:def`)
    CreateDef,
    GetDef,
    Objective,
    SetObjective,
    TestVar,
    DefineFunc,
    FuncName(String),
    EndFunc,
    CallFunc,
    /// The created def
    CreatedDef([String; 2]),
    /// Get a variable
    VarName(String),
    /// Get a replacement
    DefName(String),
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
    /// Left parenthesis
    LeftParen,
    /// Right parenthesis
    RightParen,
    /// Commands, etc. that are not by databind
    NonDatabind(String),
    NewLine,
    None,
}
