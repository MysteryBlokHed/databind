use serde_derive::Deserialize;

/// Settings for the transpiler
#[derive(Debug, Deserialize)]
pub struct Settings {
    pub random_var_names: bool,
    pub var_display_names: bool,
    pub generate_func_tags: bool,
    pub func_tag_inclusions: Vec<String>,
    pub to_transpile: Vec<String>,
}
