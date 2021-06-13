use serde_derive::Deserialize;

/// Settings for the transpiler
#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub random_var_names: bool,
    pub var_display_names: bool,
    pub func_tag_inclusions: Vec<String>,
    pub to_transpile: Vec<String>,
    pub output: Option<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            random_var_names: false,
            var_display_names: false,
            func_tag_inclusions: vec![String::from("tick"), String::from("load")],
            to_transpile: vec![String::from("**/*.databind")],
            output: None,
        }
    }
}
