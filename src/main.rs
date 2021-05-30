mod settings;
mod token;
mod transpiler;

fn main() {
    let transpile_settings = settings::Settings {
        randomize_var_names: true,
        var_display_name: true,
    };

    let totally_real_file = String::from(
        ":var example .= 1\r\n\
:var newVar .= 3\r\n\
:var example += 5\r\n\
:var example = 3\r\n\
:var newVar -= 2\r\n\
# Test if things worked
execute if :tvar newVar matches 1 run say newVar subtraction worked\r\n\
execute if :tvar example matches 3 run say example setting worked\r\n",
    );

    let mut transpile = transpiler::Transpiler::new(totally_real_file);
    let tokens = transpile.tokenize();
    println!("{:?}", tokens);

    let transpiled = transpiler::Transpiler::transpile(tokens, transpile_settings);
    println!("{}", transpiled);
}
