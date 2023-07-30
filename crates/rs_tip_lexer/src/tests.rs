use super::*;

use expect_test::{expect, Expect};

fn check_lexing(input: &str, expect: Expect) {
    let actual: String = tokenize(input)
        .map(|token| format!("{:?}\n", token))
        .collect();
    expect.assert_eq(&actual)
}

#[test]
fn smoke_test() {
    check_lexing(
        "iterate(n) {
    var f;
    f = 1;
    while (n > 0) {
        f = f * n;
        n = n - 1;
    }
    return f;",
        expect![[r#"
            Token { kind: Ident, len: 7 }
            Token { kind: OpenParen, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: CloseParen, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: OpenBrace, len: 1 }
            Token { kind: Whitespace, len: 5 }
            Token { kind: Var, len: 3 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Semi, len: 1 }
            Token { kind: Whitespace, len: 5 }
            Token { kind: Ident, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Semi, len: 1 }
            Token { kind: Whitespace, len: 5 }
            Token { kind: While, len: 5 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: OpenParen, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Gt, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: CloseParen, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: OpenBrace, len: 1 }
            Token { kind: Whitespace, len: 9 }
            Token { kind: Ident, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Star, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Semi, len: 1 }
            Token { kind: Whitespace, len: 9 }
            Token { kind: Ident, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Minus, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Semi, len: 1 }
            Token { kind: Whitespace, len: 5 }
            Token { kind: CloseBrace, len: 1 }
            Token { kind: Whitespace, len: 5 }
            Token { kind: Return, len: 6 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Semi, len: 1 }
        "#]],
    )
}
