mod cursor;

#[cfg(test)]
mod tests;

pub use crate::cursor::Cursor;

use self::TokenKind::*;

/// Parsed token.
#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub len: u32,
}

impl Token {
    fn new(kind: TokenKind, len: u32) -> Token {
        Token { kind, len }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    /// A sequence of whitespace characters.
    Whitespace,

    /// An identifier.
    Ident,

    /// An integer.
    Int,

    // Keywords:
    /// "input"
    Input,
    /// "output"
    Output,
    /// "if"
    If,
    /// "else"
    Else,
    /// "while"
    While,
    /// "return"
    Return,
    /// "var"
    Var,
    /// "alloc"
    Alloc,
    /// "null"
    Null,

    // Symbols:
    /// ","
    Comma,
    /// "."
    Dot,
    /// ";"
    Semi,
    /// ":"
    Colon,
    /// "&"
    And,
    /// "+"
    Plus,
    /// "-"
    Minus,
    /// "*"
    Star,
    /// "/"
    Slash,
    /// ">"
    Gt,
    /// "="
    Eq,
    /// "=="
    EqEq,
    /// "("
    OpenParen,
    /// ")"
    CloseParen,
    /// "{"
    OpenBrace,
    /// "}"
    CloseBrace,

    /// Unknown token, unrecognized by the lexer.
    Unknown,

    /// End of input.
    Eof,
}

pub fn tokenize(input: &str) -> impl Iterator<Item = Token> + '_ {
    let mut cursor = Cursor::new(input);
    std::iter::from_fn(move || {
        let token = cursor.advance_token();
        if token.kind != Eof {
            Some(token)
        } else {
            None
        }
    })
}

pub fn is_whitespace(c: char) -> bool {
    matches!(c, ' ' | '\n' | '\t' | '\r')
}

impl Cursor<'_> {
    pub fn advance_token(&mut self) -> Token {
        let first_char = match self.bump() {
            Some(c) => c,
            None => return Token::new(TokenKind::Eof, 0),
        };
        let token_kind = match first_char {
            // Whitespace sequence.
            c if is_whitespace(c) => self.whitespace(),

            // Identifier or keyword.
            'a'..='z' => self.ident_or_keyword(),

            // One-character symbols.
            ',' => Comma,
            '.' => Dot,
            ':' => Colon,
            ';' => Semi,
            '&' => And,
            '+' => Plus,
            '-' => Minus,
            '*' => Star,
            '/' => Slash,
            '>' => Gt,
            '(' => OpenParen,
            '{' => OpenBrace,
            ')' => CloseParen,
            '}' => CloseBrace,

            // "==" is the only possible two-character symbol.
            '=' => {
                if self.first() == '=' {
                    self.bump();
                    EqEq
                } else {
                    Eq
                }
            }

            _ => Unknown,
        };
        let token = Token::new(token_kind, self.pos_within_token());
        self.reset_pos_within_token();
        token
    }

    fn whitespace(&mut self) -> TokenKind {
        self.eat_while(is_whitespace);
        Whitespace
    }

    fn ident_or_keyword(&mut self) -> TokenKind {
        self.eat_while(|c| matches!(c, 'a'..='z'));
        match self.str_until_pos_within_token() {
            "input" => Input,
            "output" => Output,
            "if" => If,
            "else" => Else,
            "while" => While,
            "return" => Return,
            "var" => Var,
            "alloc" => Alloc,
            "null" => Null,
            _ => Ident,
        }
    }
}
