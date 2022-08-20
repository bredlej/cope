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
                    .label("build_world")
                    .with_system(initialize_tiles)
                    .with_system(initialize_actors))
            .add_system_set(
                SystemSet::on_update(GameState::Run)
                    .label("render_world")
                    .with_system(keyboard_input)
            );
    }
}

fn spawn(mut commands: &mut Commands, component: impl Component, name: String, index: usize, atlas: &Res<CharsetAsset>) {
    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: atlas.atlas.clone(),
        sprite: TextureAtlasSprite {
            color: Color::WHITE,
            index: index,
            ..Default::default()
        },
        ..Default::default()
    })
        .insert(component)
        .insert(NameC(name));
}

pub fn initialize_actors(mut commands: Commands, atlas: Res<CharsetAsset>, mut game_state: ResMut<State<GameState>>) {
    spawn(&mut commands, Player, "Player".to_string(), '@' as usize, &atlas);
    spawn(&mut commands, Actor, "Bat".to_string(), 'b' as usize, &atlas);
    game_state.set(GameState::Run).unwrap();
}

pub fn initialize_tiles(mut commands: Commands, atlas: Res<CharsetAsset>, asset_server: Res<AssetServer>) {
    let tilemap_size = TilemapSize { x: 32, y: 32 };
    let tilemap_entity = commands.spawn().id();
    let mut tile_storage = TileStorage::empty(tilemap_size);

    let texture_handle: Handle<Image> = asset_server.load("ascii8x8.png");

    for x in 0..32u32 {
        for y in 0..32u32 {
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
    commands.entity(tile_storage.get(&TilePos { x: 10, y: 10 }).unwrap())
        .insert(TileColor(Color::Rgba {
            red: 1.0,
            green: 0.0,
            blue: 0.0,
            alpha: 1.0,
        }))
        .insert(TileTexture('#' as u32));

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

pub fn keyboard_input(texture_atlases: Res<Assets<TextureAtlas>>,
                      keyboard: ResMut<Input<KeyCode>>,
                      mut q: Query<(&mut TextureAtlasSprite, &Handle<TextureAtlas>, With<Player>)>) {
    if keyboard.pressed(KeyCode::Space) {
        for (mut sprite, texture_handle, _player) in q.iter_mut() {
            let texture_atlas = texture_atlases.get(texture_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}