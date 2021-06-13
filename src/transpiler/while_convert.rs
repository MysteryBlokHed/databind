use super::Transpiler;
use crate::settings::Settings;
use crate::token::Token;
use rand::{distributions::Alphanumeric, Rng};

fn get_chars() -> String {
    let chars = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(4)
        .map(char::from)
        .collect::<String>()
        .to_lowercase();

    chars
}

impl Transpiler<'_> {
    /// Replace while loops with databind function definitions
    ///
    /// # Arguments
    ///
    /// - `tokens` - A list of tokens that may include while loops
    pub fn while_convert(&self, tokens: Vec<Token>) -> Vec<Token> {
        let mut new_tokens = tokens.clone();

        let mut while_index: usize = 0;
        let mut index_offset: usize = 0;
        let mut looping = true;
        let mut new_contents = String::new();
        let mut chars = get_chars();

        for i in 0..tokens.len() {
            let token = tokens.get(i).unwrap();
            if looping {
                match token {
                    Token::WhileCondition(condition) => new_contents.push_str(
                        &format!(
                            ":func while_{chars}\n\
                             execute if {condition} run :call condition_{chars}\n\
                             :endfunc\n",
                            chars = chars,
                            condition = condition
                        )[..],
                    ),
                    Token::WhileContents(contents) => new_contents.push_str(
                        &format!(
                            ":func condition_{chars}\n\
                             {contents}\n\
                             :call while_{chars}\n\
                             :endfunc\n",
                            chars = chars,
                            contents = contents
                        )[..],
                    ),
                    Token::EndWhileLoop => {
                        new_contents.push_str(&format!(":call while_{}\n", chars)[..]);
                        chars = get_chars();

                        // Tokenize new contents
                        let tks =
                            Transpiler::new(new_contents.clone(), &Settings::default()).tokenize();

                        // When gettings indexes in the new tokens vector,
                        // the length and position of elements will have changed
                        // due to new things being added
                        new_tokens.splice(
                            while_index + index_offset..while_index + index_offset + 4,
                            tks.iter().cloned(),
                        );
                        index_offset += tks.len() - 4;
                    }
                    _ => {}
                }
            }

            if token == &Token::WhileLoop {
                looping = true;
                while_index = i;
            }
        }

        new_tokens
    }
}
