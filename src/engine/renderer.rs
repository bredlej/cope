use bevy_ecs_tilemap::prelude::{TileTexture};
use bevy_ecs_tilemap::tiles::{TilePos, TileStorage};
use crate::prelude::*;
use crate::engine::components::*;

pub fn render(mut tiles: Query<&OccupiedBy>,
              player_query: Query<&Player>,
              position_query: Query<&Position>,
              glyph_query: Query<&Glyph>,
              mut tile_storage_q: Query<&TileStorage>,
              mut commands: Commands)
{
    for occupied_by in tiles.iter_mut() {
        let entity: Entity = *occupied_by.entities.last().unwrap();
        if let Ok(_) = player_query.get(entity) {
            if let Ok(position) = position_query.get(entity) {
                let x: u32 = position.x as u32;
                let y: u32 = position.y as u32;
                let glyph = glyph_query.get(entity).expect("Player has no glyph");
                for tile in tile_storage_q.iter_mut() {
                    commands.entity(tile.get(&TilePos { x, y }).unwrap())
                        .insert(TileTexture(glyph.0));
                }
            }
        }
    }
}