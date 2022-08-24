use bevy::ecs::system::Command;
use bevy_ecs_tilemap::map::{TilemapGridSize, TilemapId, TilemapTexture, TilemapTileSize};
use bevy_ecs_tilemap::prelude::{TilemapSize, TileTexture};
use bevy_ecs_tilemap::TilemapBundle;
use bevy_ecs_tilemap::tiles::{TileBundle, TileColor, TilePos, TileStorage};
use crate::prelude::*;
use crate::components::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_update(GameState::BuildWorld)
                    .label("initialize_tiles")
                    .with_system(initialize_tiles).before(initialize_actors)
            )
            .add_system_set(
                SystemSet::on_update(GameState::BuildWorld)
                    .label("initialize_actors")
                    .with_system(initialize_actors).after(initialize_tiles)
            )
            .add_system_set(
                SystemSet::on_update(GameState::Run)
                    .label("render_world")
                    .with_system(keyboard_input)
            );
    }
}

pub fn initialize_tiles(mut commands: Commands, atlas: Res<CharsetAsset>, asset_server: Res<AssetServer>) {
    println!("Initialize tiles");
    let tilemap_size = TilemapSize { x: 3, y: 3 };
    let tilemap_entity = commands.spawn().id();
    let mut tile_storage = TileStorage::empty(tilemap_size);

    let texture_handle: Handle<Image> = asset_server.load("ascii8x8.png");

    for x in 0..3u32 {
        for y in 0..3u32 {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn()
                .insert_bundle(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture: TileTexture('.' as u32),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, Some(tile_entity));
        }
    }
    /*commands.entity(tile_storage.get(&TilePos { x: 0, y: 0 }).unwrap())
        .insert(TileColor(Color::Rgba {
            red: 1.0,
            green: 0.0,
            blue: 0.0,
            alpha: 1.0,
        }))
        .insert(TileTexture('#' as u32));*/

    let tile_size = TilemapTileSize { x: 8.0, y: 8.0 };

    commands
        .entity(tilemap_entity)
        .insert_bundle(TilemapBundle {
            grid_size: TilemapGridSize { x: 16.0, y: 16.0 },
            size: tilemap_size,
            storage: tile_storage,
            texture: TilemapTexture(texture_handle),
            tile_size,
            transform: bevy_ecs_tilemap::helpers::get_centered_transform_2d(
                &tilemap_size,
                &tile_size,
                0.0,
            ),
            visibility: Visibility { is_visible: true },
            ..Default::default()
        });
}

pub fn initialize_actors(mut commands: Commands,
                         atlas: Res<CharsetAsset>,
                         mut game_state: ResMut<State<GameState>>,
                         mut tiles: Query<&TileStorage>) {
    println!("Initialize actors");
    let player = commands.spawn().insert(Player).insert(NameC("Player".to_string())).id();


    game_state.set(GameState::Run).unwrap();
}

pub fn keyboard_input(texture_atlases: Res<Assets<TextureAtlas>>,
                      keyboard: ResMut<Input<KeyCode>>,
                      mut q: Query<(&mut TextureAtlasSprite, &Handle<TextureAtlas>, With<Player>)>,
                      mut tiles: Query<&OccupiedBy>,
                      mut player_query: Query<(Entity, &Player)>,
                      mut position_query: Query<&Position>,
                      mut tile_storage_q: Query<&TileStorage>,
                      mut commands: Commands) {
    if keyboard.pressed(KeyCode::Space) {
        for (mut sprite, texture_handle, _player) in q.iter_mut() {
            let texture_atlas = texture_atlases.get(texture_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
    for occupied_by in tiles.iter_mut() {
        let entity: Entity = *occupied_by.entities.last().unwrap();
        // need to get a specific component of `entity` here
        if let Ok(player) = player_query.get(entity) {
            if let Ok(position) = position_query.get(entity) {
                let x:u32 = position.x as u32;
                let y:u32 = position.y as u32;
                println!("Player");
                for tile in tile_storage_q.iter_mut() {
                    commands.entity(tile.get(&TilePos { x, y}).unwrap())
                        .insert(TileTexture('#' as u32));
                }

            }
        }
    }
    for (entity, player) in player_query.iter_mut() {
        for tile in tile_storage_q.iter_mut() {
            commands.entity(tile.get(&TilePos { x: 0, y: 0 }).unwrap()).insert(OccupiedBy { entities: vec![entity] });
        }
    }


}