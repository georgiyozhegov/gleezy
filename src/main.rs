use gleezy_lex::Lex;

fn main() {
    let source = r#"
        123+ 999-0
    "#;
    let source = source.chars().peekable();
    let lex = Lex::new(source);

    for token in lex {
        println!("{token:?}");
    }
}
