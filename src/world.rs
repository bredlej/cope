use crate::prelude::*;
use crate::components::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(GameState::BuildWorld)
                    .label("build_world")
                    .with_system(initialize))
            .add_system_set(
                SystemSet::on_update(GameState::BuildWorld)
                    .label("render_world")
                    .with_system(keyboard_input)
            );
    }
}

fn spawn<> (mut commands: &mut Commands, component: impl Component, name: String, index: usize, atlas: &Res<CharsetAsset>){
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

pub fn initialize(mut commands: Commands, atlas: Res<CharsetAsset>) {
    println!("World plugin initialize");
    spawn(&mut commands, Actor, "Bat".to_string(), 2, &atlas);
    spawn(&mut commands, Player, "Player".to_string(), 2, &atlas);
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