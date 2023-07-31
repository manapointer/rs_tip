use crate::{marker::Marker, step::StepEvent};
use rowan::{Checkpoint, GreenNode, GreenNodeBuilder};
use rs_tip_syntax::SyntaxKind::{self, *};

mod grammar;
mod marker;
mod step;

pub fn parse(input: &str) -> Parse {
    let mut builder = Builder::new(input);
    // let mut parser = Parser::new(&mut builder);
    // grammar::program(&mut parser);
    Parse {
        green_node: builder.builder.finish(),
        errors: Vec::new(),
    }
}

pub struct Parse {
    green_node: GreenNode,
    errors: Vec<String>,
}

struct Builder<'a> {
    builder: GreenNodeBuilder<'static>,
    input: &'a str,
    input_pos: usize,
    tokens: Vec<(SyntaxKind, u32)>,
    tokens_pos: usize,
    // Tokens without whitespace, used by the parser.
    tokens_no_whitespace: Vec<(SyntaxKind, u32)>,
    tokens_no_whitespace_pos: usize,
}

pub struct Input {
    tokens: Vec<(SyntaxKind, u32)>,
}

impl<'a> Builder<'a> {
    fn new(input: &'a str) -> Self {
        let tokens: Vec<(SyntaxKind, u32)> = rs_tip_lexer::tokenize(input)
            .map(|token| (token.kind.into(), token.len))
            .collect();
        let tokens_no_whitespace = tokens
            .iter()
            .filter(|(kind, _len)| *kind == WHITESPACE)
            .cloned()
            .collect();
        Self {
            builder: GreenNodeBuilder::new(),
            input,
            input_pos: 0,
            tokens,
            tokens_pos: 0,
            tokens_no_whitespace,
            tokens_no_whitespace_pos: 0,
        }
    }

    fn nth(&self, n: usize) -> SyntaxKind {
        let pos = self.tokens_pos + n;
        if pos >= self.tokens.len() {
            EOF
        } else {
            self.tokens[pos].0
        }
    }

    fn token(&mut self) {
        self.eat_whitespace();
        self.do_token();
        self.tokens_pos += 1;
    }

    fn eat_whitespace(&mut self) {
        while self.tokens_no_whitespace_pos < self.tokens_no_whitespace.len() {
            if self.tokens_no_whitespace[self.tokens_no_whitespace_pos].0 != WHITESPACE {
                break;
            }
            self.do_token()
        }
    }

    fn do_token(&mut self) {
        let (kind, len) = self.tokens_no_whitespace[self.tokens_no_whitespace_pos];
        self.builder.token(
            kind.into(),
            &self.input[self.input_pos..self.input_pos + len as usize],
        );
        self.tokens_no_whitespace_pos += 1;
        self.input_pos += len as usize;
    }

    fn checkpoint(&self) -> Checkpoint {
        self.builder.checkpoint()
    }

    fn start_node(&mut self, kind: SyntaxKind) {
        self.builder.start_node(kind.into())
    }

    fn start_node_at(&mut self, checkpoint: Checkpoint, kind: SyntaxKind) {
        self.builder.start_node_at(checkpoint, kind.into())
    }

    fn finish_node(&mut self) {
        self.builder.finish_node()
    }
}

/// A parser for TIP code. It takes a stream of non-trivia tokens as input and processes that stream
/// to construct a parse tree. Because the parser only operates on token types and has no knowledge of
/// text offsets, etc., it instead outputs a series of steps that can be consumed by an actual parse
/// tree builder to construct the final tree.
pub(crate) struct Parser {
    tokens: Vec<(SyntaxKind, u32)>,
    events: Vec<StepEvent>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<(SyntaxKind, u32)>) -> Self {
        Self {
            tokens,
            events: Vec::new(),
            pos: 0,
        }
    }

    pub(crate) fn current(&self) -> SyntaxKind {
        self.nth(0)
    }

    pub(crate) fn nth(&self, n: usize) -> SyntaxKind {
        let pos = self.pos + n;
        if pos >= self.tokens.len() {
            EOF
        } else {
            self.tokens[n].0
        }
    }

    pub(crate) fn at(&self, kind: SyntaxKind) -> bool {
        self.nth_at(0, kind)
    }

    pub(crate) fn nth_at(&self, n: usize, kind: SyntaxKind) -> bool {
        self.nth(n) == kind
    }

    pub(crate) fn eat(&mut self, kind: SyntaxKind) -> bool {
        if !self.at(kind) {
            false
        } else {
            self.push_event(StepEvent::Token { kind });
            self.pos += 1;
            true
        }
    }

    pub(crate) fn bump(&mut self, kind: SyntaxKind) {
        assert!(self.eat(kind))
    }

    pub(crate) fn bump_any(&mut self) {
        if !self.at(EOF) {
            // TODO(manapointer): Emit a step to consume the token.
            self.pos += 1;
        }
    }

    /// Starts a new node in the syntax tree. All nodes and tokens
    /// consumed between the `start` and the corresponding `Marker::complete`
    /// belong to the same node.
    pub(crate) fn start(&mut self) -> Marker {
        let pos = self.events.len() as u32;
        self.push_event(StepEvent::Tombstone);
        Marker::new(pos)
    }

    pub(crate) fn push_event(&mut self, event: StepEvent) {
        self.events.push(event);
    }

    pub(crate) fn error<T: Into<String>>(&mut self, message: T) {
        self.events.push(StepEvent::Error {
            message: message.into(),
        })
    }
}
