use crate::prelude::*;
use crate::components::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(GameState::BuildWorld)
                    .label("build_world")
                    .with_system(initialize));
    }
}

pub fn initialize(mut commands: Commands, atlas: Res<CharsetAsset>) {
    println!("World plugin initialize");
    commands.spawn().insert(Actor).insert(NameC("Player".to_string())).insert(Player);
    commands.spawn().insert(Actor).insert(NameC("Bat".to_string()));
    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: atlas.atlas.clone(),
        sprite: TextureAtlasSprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(10.0, 10.0)),
            index: 1,
            ..Default::default()
        },
        ..Default::default()
    });
}

pub fn list_actors(query: Query<&NameC, With<Actor>>) {
    /*println!("Actors:");
    for name in query.iter() {
        println!(" - {}", name.0);
    }*/
}