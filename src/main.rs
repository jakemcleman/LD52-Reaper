// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::window::WindowId;
use bevy::winit::WinitWindows;
use bevy::DefaultPlugins;
use ld52_reaper::GamePlugin;
use std::io::Cursor;
use winit::window::Icon;
// use bevy_pkv::PkvStore;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        // .insert_resource(PkvStore::new("VaguelyDamp", "ld52_reaper_game")) // ToDo
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 1280.,
                height: 800.,
                title: "LD52 - Reaper".to_string(), // ToDo
                canvas: Some("#bevy".to_owned()),
                // TODO: once the game can be quit without alt-f4, change release to use borderless fullscreen
                mode: if cfg!(debug_assertions) { WindowMode::Windowed } else { WindowMode::Windowed }, 
                ..Default::default()
            },
            ..default()
        }).set(ImagePlugin::default_nearest())
        .set(AssetPlugin {
            watch_for_changes: cfg!(debug_assertions),
            ..Default::default()
        }))
        .add_plugin(GamePlugin)
        .add_startup_system(set_window_icon)
        .run();
}

// Sets the icon on windows and X11
fn set_window_icon(windows: NonSend<WinitWindows>) {
    let primary = windows.get_window(WindowId::primary()).unwrap();
    let icon_buf = Cursor::new(include_bytes!(
        "../build/macos/AppIcon.iconset/icon_256x256.png"
    ));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
    };
}
