use crate::prelude::*;

#[derive(Clone, Eq, PartialEq, Hash, Debug )]
pub enum GameState {
    Initial,
    BuildWorld,
    WaitForUserInput
}

pub struct CharsetAsset {
    pub atlas: Handle<TextureAtlas>,
}