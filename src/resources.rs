use crate::prelude::*;

#[derive(Clone, Eq, PartialEq, Hash, Debug )]
pub enum GameState {
    Initial,
    BuildWorld
}

pub struct CharsetAsset {
    pub atlas: Handle<TextureAtlas>,
}