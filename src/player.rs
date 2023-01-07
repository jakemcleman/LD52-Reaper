use crate::actions::Actions;
use crate::GameState;
use crate::sprite_anim::SpriteAnimator;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub struct PlayerPlugin;

#[derive(Component, Default, Clone)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(move_player));
    }
}

#[derive(Clone, Default, Bundle)]
pub struct PlayerBundle {
    #[bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    pub sprite_animator: SpriteAnimator,
    pub player: Player,
}

impl LdtkEntity for PlayerBundle {
    fn bundle_entity(
        _entity_instance: &EntityInstance,
        _layer_instance: &LayerInstance,
        _tileset: Option<&Handle<Image>>,
        _tileset_definition: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        texture_atlases: &mut Assets<TextureAtlas>,
    ) -> Self {
        let texture_handle = asset_server.load("sprites/sam1.png");
        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(42., 32.), 4, 1, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        
        PlayerBundle {
            sprite_sheet_bundle: SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
                ..Default::default()
            },
            sprite_animator: crate::sprite_anim::SpriteAnimator::new(texture_atlas_handle.clone(), 0, 3, 4, 0.2, true),
            player: Player,
        }
    }
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if actions.player_movement.is_none() {
        return;
    }
    let speed = 150.;
    let movement = Vec3::new(
        actions.player_movement.unwrap().x * speed * time.delta_seconds(),
        actions.player_movement.unwrap().y * speed * time.delta_seconds(),
        0.,
    );
    for mut player_transform in &mut player_query {
        player_transform.translation += movement;
    }
}
