use gleezy_lex::Lex;
use gleezy_parse::Parse;

fn main() {
    let source = r#"
let name = 2
let bruh = 4
"#;
    let source = source.chars().peekable();
    let lex = Lex::new(source);
    let parse = Parse::new(lex.peekable());
    for statement in parse {
        println!("{statement:?}");
    }
}
