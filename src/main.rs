use gleezy_lex::Lex;

fn main() {
    let source = r#"
        let name = 2 + 3
    "#;
    let source = source.chars().peekable();
    let lex = Lex::new(source);

    for token in lex {
        println!("{token:?}");
    }
}
