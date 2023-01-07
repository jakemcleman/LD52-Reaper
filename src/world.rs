use bevy::prelude::*;
use bevy_ecs_ldtk::{LdtkPlugin, LevelSelection, LdtkWorldBundle};

use crate::GameState;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App){
        app
            .insert_resource(LevelSelection::Index(0))
            .add_plugin(LdtkPlugin)
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup_world))
        ;
    }
}

fn setup_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("levels/World.ldtk"),
        ..Default::default()
    });
}