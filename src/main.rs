mod lexer;
mod settings;
mod token;

fn main() {
    let settings = settings::Settings {
        randomize_var_names: true,
        var_display_name: true,
    };

    let totally_real_file = String::from(
        ":var example .= 1\r\n:var newVar .= 3\r\n:var example += 2\r\n:var newVar -= 6\r\n",
    );

    let mut lex = lexer::Lexer::new(totally_real_file);
    let tokens = lex.tokenize();

    println!("{:?}", tokens);
}
