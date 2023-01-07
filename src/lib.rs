mod actions;
mod loading;
mod player;
mod mainmenu;
mod ui_events;
mod sprite_anim;
mod world;
mod camera;

use crate::actions::ActionsPlugin;
use crate::loading::LoadingPlugin;
use crate::mainmenu::MainMenuPlugin;
use crate::player::PlayerPlugin;
use crate::world::WorldPlugin;
use crate::camera::CameraPlugin;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use sprite_anim::SpriteAnimationPlugin;
use ui_events::UiEventPlugin;


// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub const VIEW_WIDTH: i32 = 320;
pub const VIEW_HEIGHT: i32 = 240;
pub const ASPECT_RATIO: f32 = (VIEW_WIDTH as f32) / (VIEW_HEIGHT as f32);

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Loading)
            .add_plugin(WorldPlugin)
            .add_plugin(LoadingPlugin)
            .add_plugin(UiEventPlugin)
            .add_plugin(MainMenuPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(SpriteAnimationPlugin)
            .add_plugin(CameraPlugin)
        ;
            

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}
