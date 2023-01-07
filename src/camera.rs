use bevy::prelude::*;
use bevy_ecs_ldtk::{LdtkLevel, LevelSelection};
use crate::ASPECT_RATIO;
use crate::player::Player;
use bevy_pixel_camera::{PixelCameraBundle, PixelProjection};
use crate::{GameState, VIEW_WIDTH, VIEW_HEIGHT};
use bevy_pixel_camera::{
    PixelBorderPlugin, PixelCameraPlugin
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ClearColor(Color::BLACK))
            .add_plugin(PixelCameraPlugin)
            .add_plugin(PixelBorderPlugin {
                color: Color::BLACK,
            })
            .add_startup_system(spawn_camera)
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(camera_fit_inside_current_level))
        ;
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(PixelCameraBundle::from_resolution(VIEW_WIDTH, VIEW_HEIGHT));
}

fn camera_fit_inside_current_level(
    mut camera_query: Query<
        (
            &mut PixelProjection,
            &mut Transform,
        ),
        Without<Player>,
    >,
    player_query: Query<&Transform, With<Player>>,
    level_query: Query<
        (&Transform, &Handle<LdtkLevel>),
        (Without<PixelProjection>, Without<Player>),
    >,
    _level_selection: Res<LevelSelection>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
) {
    if let Ok(Transform {
        translation: player_translation,
        ..
    }) = player_query.get_single()
    {
        let _player_translation = *player_translation;

        let (mut _projection, mut camera_transform) = camera_query.single_mut();

        for (level_transform, level_handle) in &level_query {
            if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
                let level = &ldtk_level.level;
                camera_transform.translation.x = level_transform.translation.x + (level.px_wid as f32) / 2.;
                camera_transform.translation.y = level_transform.translation.y + (level.px_hei as f32) / 2.;
                
                
            }
        }
    }
}