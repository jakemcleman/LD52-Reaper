use bevy::prelude::*;
use bevy_pixel_camera::{
    PixelBorderPlugin, PixelCameraBundle, PixelCameraPlugin
};

mod sprite_anim;
use sprite_anim::SpriteAnimator;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(PixelCameraPlugin)
        .add_plugin(PixelBorderPlugin {
            color: Color::BLACK,
        })
        .add_startup_system(setup)
        .add_system(sprite_anim::animate_sprite)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>,) {
    // commands.spawn(Camera2dBundle {
    //     camera_2d: Camera2d {
    //         clear_color: ClearColorConfig::Custom(Color::BLACK),
    //     },
    //     ..Default::default()
    // });
    
    commands.spawn(PixelCameraBundle::from_resolution(160, 120));

    let texture_handle =  asset_server.load("sam1.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(42., 32.), 4, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    
    commands.spawn(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle.clone(),
        ..Default::default()
    }).insert(SpriteAnimator::new(texture_atlas_handle.clone(), 0, 3, 4, 0.2, true));
}
