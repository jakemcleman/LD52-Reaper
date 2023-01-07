use bevy::prelude::*;
use bevy_pkv::PkvStore;
use bevy_quickmenu::{
    style::Stylesheet, ActionTrait, Menu, MenuItem, MenuState, QuickMenuPlugin,
    ScreenTrait,
};


use crate::GameState;
use crate::loading::SpriteAssets;
use crate::ui_events::UiEvent;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(QuickMenuPlugin::<Screens>::new())
            .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(setup_menu))
            .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(cleanup_menu))
            ;
    }
}

#[derive(Debug, Clone, Default)]
pub struct MainMenuState {
    game_logo: Handle<Image>,
    boolean1: bool,
    boolean2: bool,
    save_slots: u32,
}

/// The possible actions in our main menu/settings
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Actions {
    NewGame,
    LoadGame,
    Quit,
    Toggle1,
    Toggle2,
}

fn setup_menu(mut commands: Commands, pkv: Res<PkvStore>, sprites: Res<SpriteAssets>) {
    let sheet = Stylesheet::default().with_background(BackgroundColor(Color::BLACK));

    commands.insert_resource(MenuState::new(
        MainMenuState { 
            game_logo: sprites.texture_dampboi.clone(),
            boolean1: pkv.get("boolean1").unwrap_or(false), 
            boolean2: pkv.get("boolean2").unwrap_or(false),
            save_slots: pkv.get("save_slot_count").unwrap_or(0),
            },
        Screens::Root,
        Some(sheet),
    ))
}

/// Handle the possible actions
impl ActionTrait for Actions {
    type State = MainMenuState;
    type Event = UiEvent;
    fn handle(&self, state: &mut MainMenuState, event_writer: &mut EventWriter<UiEvent>) {
        match self {
            Actions::NewGame => event_writer.send(UiEvent::NewGame),
            Actions::LoadGame => event_writer.send(UiEvent::LoadGame),
            Actions::Quit => event_writer.send(UiEvent::QuitGame),
            Actions::Toggle1 => { 
                state.boolean1 = !state.boolean1;
                event_writer.send(UiEvent::Boolean1Changed(state.boolean1));
                },
            Actions::Toggle2 => {
                state.boolean2 = !state.boolean2;
                event_writer.send(UiEvent::Boolean2Changed(state.boolean2));
            }    
        }
    }
}

/// All possible screens in our example
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Screens {
    Root,
    Booleans,
}

/// Map from from `Screens` to the actual menu
impl ScreenTrait for Screens {
    type Action = Actions;
    type State = MainMenuState;
    fn resolve(&self, state: &MainMenuState) -> Menu<Screens> {
        match self {
            Screens::Root => root_menu(state),
            Screens::Booleans => boolean_menu(state),
        }
    }
}

/// The `root` menu that is displayed first
fn root_menu(state: &MainMenuState) -> Menu<Screens> {
    let mut entries = vec![
        MenuItem::image(state.game_logo.clone()),
        MenuItem::headline("Bevy Starter Game"),
        MenuItem::label("Main Menu"),
    ];
    
    if state.save_slots > 0 {
        entries.push(MenuItem::action("Resume Game", Actions::LoadGame));
    }
    
    entries.push(MenuItem::action("New Game", Actions::NewGame));
    entries.push(MenuItem::screen("Settings", Screens::Booleans));
    entries.push(MenuItem::action("Exit", Actions::Quit));
    
    Menu::new(
        "main",
        entries,
    )
}

/// The boolean menu which is accessed from the `Screens::Boolean` entry in the root_menu
fn boolean_menu(state: &MainMenuState) -> Menu<Screens> {
    Menu::new(
        "boolean",
        vec![
            MenuItem::label("Toggles some booleans"),
            MenuItem::action("Toggle Boolean 1", Actions::Toggle1).checked(state.boolean1),
            MenuItem::action("Toggle Boolean 2", Actions::Toggle2).checked(state.boolean2),
        ],
    )
}



fn cleanup_menu(
    mut commands: Commands
) {
    bevy_quickmenu::cleanup(&mut commands);
}