use crate::loading::FontAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy::app::AppExit;
use bevy_ui_navigation::prelude::*;

pub struct MenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ButtonColors>()
            .add_plugins(DefaultNavigationPlugins)
            .add_system(button_system.after(NavRequestSystem))
            .add_system(print_nav_events.after(NavRequestSystem))
            .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(setup_menu))
            .add_system_set(SystemSet::on_update(GameState::Menu).with_system(button_nav_events))
            .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(cleanup_menu));
    }
}

#[derive(Resource)]
struct ButtonColors {
    normal: Color,
    hovered: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::rgb(0.15, 0.15, 0.15),
            hovered: Color::rgb(0.25, 0.25, 0.25),
        }
    }
}

#[derive(Component)]
enum MenuButton {
    Play,
    Options,
    Quit,
}

fn setup_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    button_colors: Res<ButtonColors>,
) {
   spawn_menu_button(&mut commands, &button_colors, &font_assets.press_start, MenuButton::Play, Vec2::new(25., 10.)); 
   spawn_menu_button(&mut commands, &button_colors, &font_assets.press_start, MenuButton::Options, Vec2::new(50., 10.)); 
   spawn_menu_button(&mut commands, &button_colors, &font_assets.press_start, MenuButton::Quit, Vec2::new(75., 10.)); 
}

fn spawn_menu_button(commands: &mut Commands, button_colors: &ButtonColors, font: &Handle<Font>, button_type: MenuButton, position: Vec2) {
    let position = UiRect {
        left: Val::Percent(position.x),
        top: Val::Percent(position.y),
        ..Default::default()
    };
    
    let label_string = match button_type {
        MenuButton::Play => "Play",
        MenuButton::Options => "Options",
        MenuButton::Quit => "Quit",
    }.to_string();
    
    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(280.0), Val::Px(80.0)),
                position,
                position_type: PositionType::Absolute,
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: button_colors.normal.into(),
            ..Default::default()
        })
        .insert(button_type)
        .insert(Focusable::default())
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: label_string,
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 32.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    }],
                    alignment: Default::default(),
                },
                ..Default::default()
            });
        });
}

fn button_system(
    mut interaction_query: Query<(&Focusable, &mut BackgroundColor), Changed<Focusable>>,
) {
    for (focus, mut material) in interaction_query.iter_mut() {
        let color = match focus.state() {
            FocusState::Focused => Color::ORANGE_RED,
            FocusState::Active => Color::GOLD,
            FocusState::Prioritized => Color::GRAY,
            FocusState::Inert => Color::DARK_GRAY,
            FocusState::Blocked => Color::ANTIQUE_WHITE,
        };
        *material = color.into();
    }
}

fn print_nav_events(mut events: EventReader<NavEvent>) {
    for event in events.iter() {
        println!("{:?}", event);
    }
}

fn button_nav_events(
    mut events: EventReader<NavEvent>,
    buttons: Query<&MenuButton>,
    mut state: ResMut<State<GameState>>,
    mut exit: EventWriter<AppExit>,
    ) {
    for event in events.iter() {
        match event {
            NavEvent::NoChanges { from, request } => {
                if *request == NavRequest::Action {
                    if let Ok(button) = buttons.get(from.first().clone()) {
                        match button {
                            MenuButton::Play => state.set(GameState::Playing).unwrap(),
                            MenuButton::Options => (),
                            MenuButton::Quit => exit.send(AppExit),
                        };
                    }
                }
            },
            _ => ()
        }
    }
}
    

fn click_play_button(
    button_colors: Res<ButtonColors>,
    mut state: ResMut<State<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &MenuButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, mut color, button) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                match button {
                    MenuButton::Play => state.set(GameState::Playing).unwrap(),
                    MenuButton::Options => (),
                    MenuButton::Quit => exit.send(AppExit),
                };
                
            }
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, buttons: Query<Entity, With<Button>>) {
    for button in buttons.iter() {
        commands.entity(button).despawn_recursive();
    }
}
