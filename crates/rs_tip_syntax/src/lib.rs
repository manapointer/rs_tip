use self::SyntaxKind::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
#[repr(u16)]
pub enum SyntaxKind {
    // Tokens.
    ERROR,
    EOF,
    WHITESPACE,
    IDENT,
    INT,
    INPUT,       // "input"
    OUTPUT,      // "output"
    IF,          // "if"
    ELSE,        // "else"
    WHILE,       // "while"
    RETURN,      // "return"
    VAR,         // "var"
    ALLOC,       // "alloc"
    NULL,        // "null"
    COMMA,       // ","
    DOT,         // "."
    SEMI,        // ";"
    COLON,       // ":"
    AND,         // "&"
    PLUS,        // "+"
    MINUS,       // "-"
    STAR,        // "*"
    SLASH,       // "/"
    GT,          // ">"
    EQ,          // "="
    EQEQ,        // "=="
    OPEN_PAREN,  // "("
    CLOSE_PAREN, // ")"
    OPEN_BRACE,  // "{"
    CLOSE_BRACE, // "}"

    // Composite nodes.
    FUN,

    ASSIGN_STM,
    OUTPUT_STM,
    IF_STM,
    WHILE_STM,

    BIN_EXP,
    PAREN_EXP,
    CALL_EXP,
    ALLOC_EXP,
    REF_EXP,
    PREFIX_EXP,
    RECORD_EXP,
    FIELD_EXP,

    ARG_LIST,
    PARAM_LIST,
    DECL_LIST,
    FIELD_LIST,
    STM_LIST,

    FIELD,

    PROGRAM,
}

#[macro_export]
macro_rules! T { [ident] => { $ crate :: SyntaxKind :: IDENT } ; [int] => { $ crate :: SyntaxKind :: INT }; [input] => { $ crate :: SyntaxKind :: INPUT }; [output] => { $ crate :: SyntaxKind :: OUTPUT }; [if] => { $ crate :: SyntaxKind :: IF }; [else] => { $ crate :: SyntaxKind :: ELSE }; [while] => { $ crate :: SyntaxKind :: WHILE }; [return] => { $ crate :: SyntaxKind :: RETURN }; [var] => { $ crate :: SyntaxKind :: VAR }; [alloc] => { $ crate :: SyntaxKind :: ALLOC }; [null] => { $ crate :: SyntaxKind :: NULL }; [,] => { $ crate :: SyntaxKind :: COMMA }; [.] => { $ crate :: SyntaxKind :: DOT }; [;] => { $ crate :: SyntaxKind :: SEMI }; [:] => { $ crate :: SyntaxKind :: COLON }; [&] => { $ crate :: SyntaxKind :: AND }; [+] => { $ crate :: SyntaxKind :: PLUS }; [-] => { $ crate :: SyntaxKind :: MINUS }; [*] => { $ crate :: SyntaxKind :: STAR }; [/] => { $ crate :: SyntaxKind :: SLASH }; [>] => { $ crate :: SyntaxKind :: GT }; [=] => { $ crate :: SyntaxKind :: EQ }; [==] => { $ crate :: SyntaxKind :: EQEQ }; ['('] => { $ crate :: SyntaxKind :: OPEN_PAREN }; [')'] => { $ crate :: SyntaxKind :: CLOSE_PAREN }; ['{'] => { $ crate :: SyntaxKind :: OPEN_BRACE }; ['}'] => { $ crate :: SyntaxKind :: CLOSE_BRACE } ; }

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TipLanguage {}

impl rowan::Language for TipLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        debug_assert!(raw.0 < PROGRAM as u16);
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}

impl From<rs_tip_lexer::TokenKind> for SyntaxKind {
    fn from(token_kind: rs_tip_lexer::TokenKind) -> Self {
        use rs_tip_lexer::TokenKind::*;

        match token_kind {
            Whitespace => WHITESPACE,
            Ident => IDENT,
            Int => INT,
            Input => INPUT,
            Output => OUTPUT,
            If => IF,
            Else => ELSE,
            While => WHILE,
            Return => RETURN,
            Var => VAR,
            Alloc => ALLOC,
            Null => NULL,
            Comma => T![,],
            Dot => T![.],
            Semi => T![;],
            Colon => T![:],
            And => T![&],
            Plus => T![+],
            Minus => T![-],
            Star => T![*],
            Slash => T![/],
            Gt => T![>],
            Eq => T![=],
            EqEq => T![==],
            OpenParen => T!['('],
            CloseParen => T![')'],
            OpenBrace => T!['{'],
            CloseBrace => T!['}'],
            Unknown | Eof => ERROR, // TODO(manapointer): Is this mapping correct for Eof?
        }
    }
}
