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
    /// Define a function
    DefineFunc,
    /// The name of a function
    FuncName(String),
    /// End a function definition
    EndFunc,
    /// Call a funcition
    CallFunc,
    /// A variable's name
    VarName(String),
    /// An objective's name
    ObjectiveName(String),
    /// An objective's type (eg. deathCount)
    ObjectiveType(String),
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
    /// Commands, etc. that are not by databind
    ///
    /// In the command `execute if :tvar variable #etc`
    /// `execute if ` would be tokenized as NonDatabind.
    NonDatabind(String),
    /// A new line
    NewLine,
    None,
}
