use crate::Parser;
use rs_tip_syntax::{SyntaxKind::*, T};

pub(crate) fn program(p: &mut Parser) {
    let m = p.start();
    while !p.at(EOF) {
        item(p)
    }
    m.complete(p, PROGRAM);
}

fn item(p: &mut Parser) {
    match p.current() {
        T![ident] => fun(p),
        _ => {
            p.error("expected an identifier");
            p.bump_any();
        }
    }
}

fn fun(p: &mut Parser) {
    let m = p.start();
    p.bump(T![ident]);
    if p.at(T!['(']) {
        param_list(p);
    } else {
        p.error("expected an identifier")
    }
    m.complete(p, FUN);
}

fn param_list(p: &mut Parser) {
    let m = p.start();
    p.bump(T!['(']);

    if p.at(T![ident]) {
        p.bump(T![ident]);

        while !p.at(T![')']) && !p.at(EOF) {
            if !p.eat(T![,]) {
                p.error("expected `,`");
                if !p.at(T![ident]) {
                    break;
                }
            }
            if !p.eat(T![ident]) {
                p.error("expected parameter name");
                break;
            }
        }
    }
    if !p.eat(T![')']) {
        p.error("expected ')'")
    }
    m.complete(p, PARAM_LIST);
}
