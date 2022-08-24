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
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct OccupiedBy {
    pub entities: Vec<Entity>,
}

