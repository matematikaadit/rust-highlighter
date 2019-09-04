use rustc_lexer::TokenKind;

fn main() {
    let source = include_str!("main.rs");
    let mut rest = source;
    for token in rustc_lexer::tokenize(source) {
        match token.kind {
            TokenKind::Whitespace => {},
            _ => {
                println!("str: {}, kind: {:?}", &rest[..token.len], token.kind);
            }
        }
        rest = &rest[token.len..]
    }
}
