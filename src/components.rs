use crate::prelude::*;

#[derive(Component)]
pub struct Actor;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct NameC(pub String);

#[derive(Component)]
pub struct Position {
    x: i32,
    y: i32
}

