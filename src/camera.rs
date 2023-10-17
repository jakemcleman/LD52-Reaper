use crate::player::Player;
use crate::GameState;
use bevy::prelude::*;
use bevy_ecs_ldtk::LdtkLevel;

pub struct CameraPlugin;

#[derive(Resource, Default, Debug)]
pub struct WindowInfo {
    aspect_ratio: f32,
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK))
            .insert_resource(WindowInfo {
                aspect_ratio: 16. / 9.,
            })
            .add_startup_system(spawn_camera)
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(camera_fit_inside_current_level),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(update_aspect_ratio),
            );
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
    ldtk_levels: Res<Assets<LdtkLevel>>,
    window_info: Res<WindowInfo>,
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
                let cam_height = 360.0;
                let cam_width = cam_height * window_info.aspect_ratio;
                orthographic_projection.scaling_mode = bevy::render::camera::ScalingMode::None;
                orthographic_projection.bottom = cam_height / -2.;
                orthographic_projection.top = cam_height / 2.;
                orthographic_projection.right = cam_width / 2.;
                orthographic_projection.left = cam_width / -2.;

                camera_transform.translation.x = player_translation.x;
                camera_transform.translation.y = player_translation.y;

                let level_height = level.px_hei as f32;
                let level_bottom = level_transform.translation.y;
                let level_top = level_bottom + level_height;

                let level_width = level.px_wid as f32;
                let level_left = level_transform.translation.x;
                let level_right = level_left + level_width;

                let camera_top = orthographic_projection.top + camera_transform.translation.y;
                let camera_bottom = orthographic_projection.bottom + camera_transform.translation.y;

                let camera_left = orthographic_projection.left + camera_transform.translation.x;
                let camera_right = orthographic_projection.right + camera_transform.translation.x;

                if level_height < cam_height {
                    camera_transform.translation.y =
                        level_transform.translation.y + (level_height / 2.);
                } else {
                    if camera_bottom < level_bottom {
                        camera_transform.translation.y += level_bottom - camera_bottom;
                    }
                    if camera_top > level_top {
                        camera_transform.translation.y += level_top - camera_top;
                    }
                }

                if level_width < cam_width {
                    camera_transform.translation.x =
                        level_transform.translation.x + (level_width / 2.);
                } else {
                    if camera_left < level_left {
                        camera_transform.translation.x += level_left - camera_left;
                    }
                    if camera_right > level_right {
                        camera_transform.translation.x += level_right - camera_right;
                    }
                }
            }
        }
    }
}
