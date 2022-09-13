use bevy::prelude::*;
use bevy::render::texture::ImageSettings;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};
mod engine;
mod plugins;

mod prelude {
    pub use bevy::prelude::*;
    pub use crate::engine::components::*;
    pub use crate::world::*;
    pub use crate::engine::resources::*;
    pub use crate::engine::*;
    pub use crate::plugins::*;
}

use prelude::*;
use crate::actions::Action;

fn initialize(mut commands: Commands,
              asset_server: Res<AssetServer>,
              mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    println!("Main initialize");
    let texture_handle = asset_server.load("ascii8x8.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(8.0, 8.0), 16, 16);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.insert_resource(CharsetAsset { atlas: texture_atlas_handle.clone() });
    commands.insert_resource(PlayerAction{action: Action::None});
    commands.insert_resource(CurrentInputState(InputState::Single));
    commands.insert_resource(ImageSettings::default_nearest());
    let camera_scale = Transform {
        translation: Default::default(),
        rotation: Default::default(),
        scale: Vec3 {
            x: 0.4,
            y: 0.4,
            z: 1.0
        }
    };
    commands.spawn_bundle(Camera2dBundle {
        transform: camera_scale,
        ..default()
    });

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
        .register_inspectable::<components::Glyph>()

        .run();
}