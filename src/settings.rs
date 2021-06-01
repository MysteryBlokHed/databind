use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub random_var_names: bool,
    pub var_display_names: bool,
    pub generate_func_json: bool,
    pub func_json_exclusions: Vec<String>,
}
