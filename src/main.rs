use gleezy_lex::Lex;

fn main() {
    let source = r#"
        let abc123boo+goo 999 "boo "88
    "#;
    let source = source.chars().peekable();
    let lex = Lex::new(source);

    for token in lex {
        println!("{token:?}");
    }
}
