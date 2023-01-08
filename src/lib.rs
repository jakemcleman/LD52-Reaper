mod actions;
mod loading;
mod player;
// mod mainmenu;
mod ui_events;
mod sprite_anim;
mod world;
mod camera;
mod actor;
mod ghost;

use crate::actions::ActionsPlugin;
use crate::loading::LoadingPlugin;
// use crate::mainmenu::MainMenuPlugin;
use crate::player::PlayerPlugin;
use crate::ghost::GhostPlugin;
use crate::world::WorldPlugin;
use crate::camera::CameraPlugin;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use sprite_anim::SpriteAnimationPlugin;
use ui_events::UiEventPlugin;
use actor::ActorPlugin;
use bevy_rapier2d::render::RapierDebugRenderPlugin;


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

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Loading)
            .add_plugin(WorldPlugin)
            .add_plugin(LoadingPlugin)
            .add_plugin(UiEventPlugin)
            // .add_plugin(MainMenuPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(GhostPlugin)
            .add_plugin(ActorPlugin)
            .add_plugin(SpriteAnimationPlugin)
            .add_plugin(CameraPlugin)
            
        ;
            

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default())
                .add_plugin(RapierDebugRenderPlugin::default())
            ;
        }
    }
}
