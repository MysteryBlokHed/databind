use crate::settings::Settings;
use std::collections::HashMap;

/// Returns from the transpiler
pub enum TranspileReturn {
    /// The contents of a single file
    ///
    /// # Arguments
    ///
    /// - `String` - The contents of the file
    SingleContents(String),
    /// The contents of a single file as well as a variable name map
    ///
    /// # Arguments
    ///
    /// - `String` - The contents of the file
    /// - `HashMap<String, usize>` - A map of filenames to indexes
    SingleContentsAndMap(String, HashMap<String, String>),
    /// The contents of multiple files as well as a map of tags to functions
    ///
    /// # Arguments
    ///
    /// - `Vec<String>` - A list of file contents
    /// - `HashMap<String, usize>` - A map of filenames to indexes
    ///    The first key will always be `""`, which is the main file transpiled
    /// - `HashMap<String, Vec<String>>` - A map of tags to functions
    MultiFile(
        Vec<String>,
        HashMap<String, usize>,
        HashMap<String, Vec<String>>,
    ),
    /// The contents of multiple files as well as a variable map and a map of tags to functions
    ///
    /// # Arguments
    ///
    /// - `Vec<String>` - A list of file contents
    /// - `HashMap<String, usize>` - A map of filenames to indexes
    ///    The first key will always be `""`, which is the main file transpiled
    /// - `HashMap<String, String>` - A map of variable names used in files to randomized names
    /// - `HashMap<String, Vec<String>>` - A map of tags to functions
    MultiFileAndMap(
        Vec<String>,
        HashMap<String, usize>,
        HashMap<String, String>,
        HashMap<String, Vec<String>>,
    ),
}

pub struct Transpiler<'a> {
    chars: Vec<char>,
    position: usize,
    current_char: char,
    settings: &'a Settings,
}

impl Transpiler<'_> {
    /// Create a new Transpiler.
    ///
    /// # Arguments
    ///
    /// - `text` - The contents of the file to transpile
    /// - `settings` - The settings for the transpiler
    /// - `replacement` - Whether to replace :def's. Required to avoid stack overflow
    pub fn new<'a>(text: String, settings: &'a Settings, replacement: bool) -> Transpiler<'a> {
        let text = if replacement {
            Transpiler::replace_definitions(&text)
        } else {
            text
        };

        let first_char = if text.len() > 0 {
            text.chars().nth(0).unwrap()
        } else {
            '\u{0}'
        };

        Transpiler {
            chars: text.chars().collect(),
            position: 0,
            current_char: first_char,
            settings: &settings,
        }
    }

    /// Go to the next character in the char list
    fn next_char(&mut self) {
        self.position += 1;
        if self.position < self.chars.len() {
            self.current_char = self.chars[self.position];
        } else {
            self.current_char = '\u{0}'
        }
    }
}

mod preprocess;
mod tokenize;
mod transpile;
