use crate::engine::components::*;
use crate::prelude::*;

#[derive(Bundle)]
pub struct ActorBundle {
    pub name: NameC,
    pub glyph: Glyph,
    pub position: Position
}