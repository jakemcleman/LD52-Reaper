mod actions;
mod loading;
mod player;
// mod mainmenu;
mod actor;
mod camera;
mod door;
mod ghost;
mod menu;
mod pickup;
mod settings;
mod soul;
mod sprite_anim;
mod ui_events;
mod world;

use crate::actions::ActionsPlugin;
use crate::camera::CameraPlugin;
use crate::ghost::GhostPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::pickup::PickupPlugin;
use crate::player::PlayerPlugin;
use crate::settings::SettingsPlugin;
use crate::world::WorldPlugin;

use actor::ActorPlugin;
use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_rapier2d::render::RapierDebugRenderPlugin;
use soul::SoulPlugin;
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
    // To be used over the playing state
    Paused,
    // Shows level selection menu
    LevelSelect,
    // Shows win screen, links back to main menu
    WinScreen,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Loading)
            .add_plugin(SettingsPlugin)
            .add_plugin(WorldPlugin)
            .add_plugin(LoadingPlugin)
            .add_plugin(UiEventPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(PickupPlugin)
            .add_plugin(GhostPlugin)
            .add_plugin(SoulPlugin)
            .add_plugin(ActorPlugin)
            .add_plugin(SpriteAnimationPlugin)
            .add_plugin(CameraPlugin);

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default())
                .add_plugin(RapierDebugRenderPlugin::default());
        }
    }
}
