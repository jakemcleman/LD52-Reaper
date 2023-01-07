use bevy::{prelude::*, app::AppExit};
use bevy_pkv::PkvStore;
use crate::GameState;

#[derive(Debug)]
pub enum UiEvent {
    QuitGame,
    NewGame,
    LoadGame,
    Boolean1Changed(bool),
    Boolean2Changed(bool),
}

pub struct UiEventPlugin;

impl Plugin for UiEventPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register a event that can be called from the action handler
            .add_event::<UiEvent>()
            .add_system(event_reader)
        ;
    }
}

/// This reacts to actions fired from UI with custom bevy resources or eventwriters or queries.
fn event_reader(
    mut event_reader: EventReader<UiEvent>, 
    mut exit: EventWriter<AppExit>,
    mut state: ResMut<State<GameState>>,
    mut pkv: ResMut<PkvStore>,
    ) {
    for event in event_reader.iter() {
        match event {
            UiEvent::QuitGame => exit.send(AppExit),
            UiEvent::NewGame => state.set(GameState::Playing).unwrap(),
            UiEvent::LoadGame => state.set(GameState::Playing).unwrap(),
            UiEvent::Boolean1Changed(new_val) => pkv.set("boolean1", new_val).expect("Failed to store setting"),
            UiEvent::Boolean2Changed(new_val) => pkv.set("boolean2", new_val).expect("Failed to store setting"),
        }
    }
}