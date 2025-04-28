use gleezy_lex::Lex;
use gleezy_parse::Parse;

fn main() {
    let source = r#"
let a = 2
let mutable b = 3
"#;
    let source = source.chars().peekable();
    let lex = Lex::new(source);
    let parse = Parse::new(lex.peekable());
    for statement in parse {
        println!("{statement:#?}");
    }
}
