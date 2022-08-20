use crate::prelude::*;
use bevy_inspector_egui::{Inspectable};

#[derive(Component, Inspectable)]
pub struct Actor;

#[derive(Component, Inspectable)]
pub struct Player;

#[derive(Component, Inspectable)]
pub struct NameC(pub String);

#[derive(Component, Inspectable)]
pub struct Position {
    x: i32,
    y: i32
}

