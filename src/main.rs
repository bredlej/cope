use bevy::prelude::*;
use bevy::render::texture::ImageSettings;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_ecs_tilemap::tiles::TileTexture;
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};

mod components;
mod world;
mod resources;

mod prelude {
    pub use bevy::prelude::*;
    pub use crate::components::*;
    pub use crate::world::*;
    pub use crate::resources::*;
}

use prelude::*;

fn initialize(mut commands: Commands,
              asset_server: Res<AssetServer>,
              mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    println!("Main initialize");
    let texture_handle = asset_server.load("ascii8x8.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(8.0, 8.0), 16, 16);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.insert_resource(CharsetAsset { atlas: texture_atlas_handle.clone() });
    commands.insert_resource(ImageSettings::default_nearest());
    commands.spawn_bundle(Camera2dBundle::default());
}

fn run(mut game_state: ResMut<State<GameState>>) {
    game_state.set(GameState::BuildWorld).unwrap();
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "COPE".to_string(),
            width: 800.0,
            height: 800.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::from([0.0, 0.0, 0.0])))
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(TilemapPlugin)
        .add_startup_system(initialize)
        .add_state(GameState::Initial)
        .add_system_set(
            SystemSet::on_update(GameState::Initial)
                .with_system(run)
        )
        .register_inspectable::<NameC>()
        .register_inspectable::<components::Position>()
        .run();
}