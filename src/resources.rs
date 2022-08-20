use crate::prelude::*;

#[derive(Clone, Eq, PartialEq, Hash, Debug )]
pub enum GameState {
    Initial,
    BuildWorld,
    Run
}

pub struct CharsetAsset {
    pub atlas: Handle<TextureAtlas>,
}