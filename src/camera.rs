use bevy::prelude::*;
use bevy_ecs_ldtk::{LdtkLevel, LevelSelection};
use crate::player::Player;
use crate::GameState;

pub struct CameraPlugin;

#[derive(Resource, Default, Debug)]
pub struct WindowInfo {
    aspect_ratio: f32,
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ClearColor(Color::BLACK))
            .insert_resource(WindowInfo { aspect_ratio: 16. / 9. })
            .add_startup_system(spawn_camera)
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(camera_fit_inside_current_level))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(update_aspect_ratio))
        ;
    }
}

fn spawn_camera(mut commands: Commands) {
    //commands.spawn(PixelCameraBundle::from_resolution(VIEW_WIDTH, VIEW_HEIGHT));
    let camera = Camera2dBundle::default();
    commands.spawn(camera);
}

fn update_aspect_ratio(windows: ResMut<Windows>, mut window_info: ResMut<WindowInfo>) {
       let window = windows.primary();
       
       window_info.aspect_ratio = window.width() / window.height();
}

pub fn camera_fit_inside_current_level(
    mut camera_query: Query<
        (
            &mut bevy::render::camera::OrthographicProjection,
            &mut Transform,
        ),
        Without<Player>,
    >,
    player_query: Query<&Transform, With<Player>>,
    level_query: Query<
        (&Transform, &Handle<LdtkLevel>),
        (Without<OrthographicProjection>, Without<Player>),
    >,
    level_selection: Res<LevelSelection>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
    window_info: Res<WindowInfo>
) {
    if let Ok(Transform {
        translation: player_translation,
        ..
    }) = player_query.get_single()
    {
        let player_translation = *player_translation;
        let (mut orthographic_projection, mut camera_transform) = camera_query.single_mut();
        for (level_transform, level_handle) in &level_query {
            if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
                let level = &ldtk_level.level;
                if level_selection.is_match(&0, level) {
                    let level_ratio = level.px_wid as f32 / ldtk_level.level.px_hei as f32;

                    orthographic_projection.scaling_mode = bevy::render::camera::ScalingMode::None;
                    orthographic_projection.bottom = 0.;
                    orthographic_projection.left = 0.;
                    if level_ratio > window_info.aspect_ratio {
                        // level is wider than the screen
                        orthographic_projection.top = level.px_hei as f32;
                        orthographic_projection.right = orthographic_projection.top * window_info.aspect_ratio;
                        camera_transform.translation.x = (player_translation.x
                            - level_transform.translation.x
                            - orthographic_projection.right / 2.)
                            .clamp(0., level.px_wid as f32 - orthographic_projection.right);
                        camera_transform.translation.y = 0.;
                    } else {
                        // level is taller than the screen
                        orthographic_projection.right = level.px_wid as f32;
                        orthographic_projection.top = orthographic_projection.right / window_info.aspect_ratio;
                        camera_transform.translation.y = (player_translation.y
                            - level_transform.translation.y
                            - orthographic_projection.top / 2.)
                            .clamp(0., level.px_hei as f32 - orthographic_projection.top);
                        camera_transform.translation.x = 0.;
                    }

                    camera_transform.translation.x += level_transform.translation.x;
                    camera_transform.translation.y += level_transform.translation.y;
                }
            }
        }
    }
}