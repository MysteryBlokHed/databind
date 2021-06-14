use serde_derive::Deserialize;

/// Settings for the transpiler
#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub random_var_names: bool,
    pub var_display_names: bool,
    pub func_tag_inclusions: Vec<String>,
    pub inclusions: Vec<String>,
    pub exclusions: Vec<String>,
    pub function_out_exclusions: Vec<String>,
    pub output: Option<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            random_var_names: false,
            var_display_names: false,
            func_tag_inclusions: vec![String::from("tick"), String::from("load")],
            inclusions: vec![String::from("**/*.databind")],
            exclusions: Vec::new(),
            function_out_exclusions: vec![String::from("**/main.databind")],
            output: None,
        }
    }
}
