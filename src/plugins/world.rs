use bevy_ecs_tilemap::map::{TilemapGridSize, TilemapId, TilemapTexture, TilemapTileSize};
use bevy_ecs_tilemap::prelude::{TilemapSize, TileTexture};
use bevy_ecs_tilemap::TilemapBundle;
use bevy_ecs_tilemap::tiles::{TileBundle, TileColor, TilePos, TileStorage};
use itertools::Itertools;
use crate::actions::{Action};
use crate::actions::Action::Movement;
use crate::actions::Direction::{East, North, South, West};
use crate::bundles::ActorBundle;
use crate::prelude::*;
use crate::engine::components::*;
use crate::GameState::WaitForUserInput;

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
                SystemSet::on_update(GameState::BuildWorld)
                    .label("set_stuff_at_positions")
                    .with_system(set_tile_occupation).after(initialize_actors)
            )
            .add_system_set(
                SystemSet::on_update(GameState::WaitForUserInput)
                    .label("user_input")
                    .with_system(user_input)
            )
            .add_system_set(
                SystemSet::on_update(GameState::WaitForUserInput)
                    .label("render")
                    .with_system(renderer::render).after(user_input)
            )
            .add_system_set(SystemSet::on_update(GameState::ProcessActions)
                .label("process_actions")
                .with_system(process_actions))
            .add_system_set(SystemSet::on_exit(GameState::ProcessActions)
                .label("post_process")
                .with_system(post_process));
    }
}

fn process_actions(commands: Commands, player_actions: Res<PlayerAction>, mut game_state: ResMut<State<GameState>>) {
    println!("{:?}", player_actions.action);
    game_state.set(WaitForUserInput);
}

fn post_process(mut player_actions: ResMut<PlayerAction>) {
    player_actions.action = Action::None;
}

fn place_wall_at(commands: &mut Commands, tile_storage: &TileStorage, x: u32, y: u32) {
    commands.entity(tile_storage.get(&TilePos { x, y }).unwrap())
        .insert(Blocked {})
        .insert(Wall {});
}

pub fn initialize_tiles(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("Initialize tiles");
    let texture_handle: Handle<Image> = asset_server.load("ascii8x8.png");
    let tilemap_size = TilemapSize { x: 13, y: 3 };
    let mut tile_storage = TileStorage::empty(tilemap_size);

    let tilemap_entity = create_tilemap(&mut commands, tilemap_size, &mut tile_storage);

    commands.entity(tile_storage.get(&TilePos { x: 0, y: 0 }).unwrap())
        .insert(TileColor(Color::Rgba {
            red: 1.0,
            green: 0.0,
            blue: 0.0,
            alpha: 1.0,
        }))
        .insert(TileTexture('#' as u32));

    let tile_size = TilemapTileSize { x: 8.0, y: 8.0 };
    let player_entity = commands.spawn().insert_bundle(ActorBundle {
        actor: Actor,
        name: NameC("Player".to_string()),
        glyph: Glyph('@' as u32),
        position: Position { x: 1, y: 1 },
    }).insert(Player {}).id();

    commands.entity(tile_storage.get(&TilePos { x: 1, y: 1 }).unwrap()).insert(OccupiedBy { entities: vec!(player_entity) });

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

fn create_tilemap(commands: &mut Commands, tilemap_size: TilemapSize, tile_storage: &mut TileStorage) -> Entity {
    let tilemap_entity = commands.spawn().id();
    for (x, y) in (0..tilemap_size.x).cartesian_product(0..tilemap_size.y) {
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
    tilemap_entity
}

pub fn initialize_actors()
{
    println!("Initialize actors");
}

pub fn set_tile_occupation(player_query: Query<(Entity, &Player)>,
                           mut game_state: ResMut<State<GameState>>)
{
    println!("Set tile occupations");
    for (entity, _player) in player_query.iter() {
        println!("FOUND");
        println!("{:?}", entity)
    }
    game_state.set(GameState::WaitForUserInput).unwrap();
}

pub fn user_input(texture_atlases: Res<Assets<TextureAtlas>>,
                  mut keyboard: ResMut<Input<KeyCode>>,
                  mut player_action: ResMut<PlayerAction>,
                  mut q: Query<(&mut TextureAtlasSprite, &Handle<TextureAtlas>, With<Player>)>,
                  mut game_state: ResMut<State<GameState>>,
) {
    let key = keyboard.get_pressed().next().cloned();
    let mut action = Action::None;
    if let Some(key) = key {
        action = match key {
            KeyCode::Space => {Action::None},
            KeyCode::A => { Action::Movement(West)},
            KeyCode::W => { Action::Movement(North)},
            KeyCode::S => { Action::Movement(South)},
            KeyCode::D => { Action::Movement(East)},
            _ => Action::None
        };
        keyboard.reset(key);
    }

    if action != Action::None {
        player_action.action = action;
        game_state.set(GameState::ProcessActions);
    }
}

