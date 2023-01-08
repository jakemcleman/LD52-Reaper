use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct LoadingPlugin;

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .with_collection::<FontAssets>()
                .with_collection::<AudioAssets>()
                .with_collection::<SpriteAssets>()
                .continue_to_state(GameState::Playing),
        );
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/jump2.ogg")]
    pub jump: Handle<AudioSource>,
    #[asset(path = "audio/land2.ogg")]
    pub land: Handle<AudioSource>,
    #[asset(path = "audio/attack1.ogg")]
    pub attack: Handle<AudioSource>,
    #[asset(path = "audio/hit.ogg")]
    pub hit: Handle<AudioSource>,
    #[asset(path = "audio/death1.ogg")]
    pub death: Handle<AudioSource>,
    #[asset(path = "audio/victory.ogg")]
    pub win: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct SpriteAssets {
    #[asset(path = "sprites/bevy.png")]
    pub texture_bevy: Handle<Image>,
    #[asset(path = "sprites/dampboi.png")]
    pub texture_dampboi: Handle<Image>,
    #[asset(path = "sprites/sam1.png")]
    pub texture_sam: Handle<Image>,
    #[asset(path = "sprites/ghost.png")]
    pub texture_ghost: Handle<Image>,
    #[asset(path = "sprites/soul.png")]
    pub texture_soul: Handle<Image>,
    #[asset(path = "sprites/wheat_chopped.png")]
    pub texture_wheat_chopped: Handle<Image>,
    #[asset(path = "sprites/wheat_grown.png")]
    pub texture_wheat_grown: Handle<Image>,
}
