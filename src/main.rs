mod lexer;
mod settings;
mod token;
mod transpile;

fn main() {
    let transpile_settings = settings::Settings {
        randomize_var_names: true,
        var_display_name: true,
    };

    let totally_real_file = String::from(
        ":var example .= 1\r\n:var newVar .= 3\r\n:var example += 2\r\n:var newVar -= 2\r\nexecute if :tvar newVar matches 1 run say It worked\r\n",
    );

    let mut lex = lexer::Lexer::new(totally_real_file);
    let tokens = lex.tokenize();
    println!("{:?}", tokens);

    let transpiled = transpile::transpile(tokens, transpile_settings);
    println!("{}", transpiled);
}
