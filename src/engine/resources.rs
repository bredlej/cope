use crate::actions::Action;
use crate::prelude::*;

#[derive(Clone, Eq, PartialEq, Hash, Debug )]
pub enum GameState {
    Initial,
    BuildWorld,
    WaitForUserInput,
    ProcessActions,
}

pub enum DualInputMode {
    Attack,
    Shoot
}
pub enum InputState {
    Single,
    Dual(DualInputMode),
    Target
}

pub struct CurrentInputState(pub InputState);

pub struct PlayerAction {
    pub action: Action,
}

pub struct CharsetAsset {
    pub atlas: Handle<TextureAtlas>,
}