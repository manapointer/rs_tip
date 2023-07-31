use crate::{step::StepEvent, Parser};
use drop_bomb::DropBomb;
use rs_tip_syntax::SyntaxKind;

pub(crate) struct Marker {
    pos: u32,
    bomb: DropBomb,
}

impl Marker {
    pub(crate) fn new(pos: u32) -> Marker {
        Marker {
            pos,
            bomb: DropBomb::new("marker must be either completed or abandoned"),
        }
    }

    pub(crate) fn complete(mut self, p: &mut Parser, kind: SyntaxKind) -> CompletedMarker {
        self.bomb.defuse();
        p.events[self.pos as usize] = StepEvent::Start {
            kind,
            forward_parent: None,
        };
        p.push_event(StepEvent::Finish);
        CompletedMarker::new(self.pos, kind)
    }

    pub(crate) fn abandon(mut self, p: &mut Parser) {
        self.bomb.defuse();

        // Optimization: If this marker corresponds to the most recent event, we can actually
        // get rid of it altogether, saving us some space.
        if self.pos as usize == p.events.len() - 1 {
            match p.events.pop() {
                Some(StepEvent::Tombstone) => (),
                _ => unreachable!(),
            }
        }
    }
}

pub(crate) struct CompletedMarker {
    pos: u32,
    kind: SyntaxKind,
}

impl CompletedMarker {
    fn new(pos: u32, kind: SyntaxKind) -> Self {
        Self { pos, kind }
    }
}
