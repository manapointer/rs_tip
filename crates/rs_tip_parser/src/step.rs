use std::mem;

use crate::SyntaxKind::{self, *};

/// A step in the process of building a syntax tree.
pub enum Step {
    Start(SyntaxKind),
    Finish,
    Token(SyntaxKind),
    Error(String),
}

/// Raw events produced by the processor. They contain additional fields, such
/// as forward_parent, and thus are unsuitable for direct usage; instead, we need
/// to run postprocessing steps to clean up the events.
pub(crate) enum StepEvent {
    Start {
        kind: SyntaxKind,
        forward_parent: Option<u32>,
    },
    Finish,
    Token {
        kind: SyntaxKind,
    },
    Error {
        message: String,
    },
    Tombstone,
}
