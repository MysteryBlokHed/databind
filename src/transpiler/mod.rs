use crate::settings::Settings;

pub struct Transpiler<'a> {
    chars: Vec<char>,
    position: usize,
    current_char: char,
    settings: &'a Settings,
}

impl Transpiler<'_> {
    pub fn new<'a>(text: String, settings: &'a Settings) -> Transpiler<'a> {
        let first_char = text.chars().nth(0).unwrap();

        Transpiler {
            chars: text.chars().collect(),
            position: 0,
            current_char: first_char,
            settings: &settings,
        }
    }

    fn next_char(&mut self) {
        self.position += 1;
        if self.position < self.chars.len() {
            self.current_char = self.chars[self.position];
        } else {
            self.current_char = '\u{0}'
        }
    }
}

mod tokenize;
mod transpile;
