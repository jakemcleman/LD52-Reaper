use crate::actions::Actions;
use crate::loading::*;
use crate::GameState;
use bevy::prelude::*;
use bevy::app::AppExit;
use bevy::ui::widget::ImageMode;
use bevy_ecs_ldtk::LevelSelection;
use bevy_ui_navigation::prelude::*;

pub struct MenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ButtonColors>()
            .add_plugins(DefaultNavigationPlugins)
            .add_system(button_system.after(NavRequestSystem))
            .add_system(button_nav_events.after(NavRequestSystem))
            .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(setup_menu))
            .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(cleanup_menu))
            .add_system_set(SystemSet::on_enter(GameState::LevelSelect).with_system(setup_level_select))
            .add_system_set(SystemSet::on_update(GameState::LevelSelect).with_system(back_to_menu))
            .add_system_set(SystemSet::on_exit(GameState::LevelSelect).with_system(cleanup_menu))
            .add_system_set(SystemSet::on_enter(GameState::Paused).with_system(setup_pause_menu))
            .add_system_set(SystemSet::on_update(GameState::Paused).with_system(esc_to_resume))
            .add_system_set(SystemSet::on_exit(GameState::Paused).with_system(cleanup_menu))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(esc_to_menu))
        ;
    }
}

#[derive(Resource)]
struct ButtonColors {
    normal: Color,
    highlight: Color,
    active: Color,
    
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::hex("8a8a8a").unwrap(),
            highlight: Color::hex("58ADBF").unwrap(),
            active: Color::hex("d9b866").unwrap(),
        }
    }
}

#[derive(Component)]
enum MenuButton {
    Play(usize),
    LevelSelect,
    Options,
    Menu,
    Resume,
    Quit,
}

#[derive(Component)]
struct MenuElement;

fn setup_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    button_colors: Res<ButtonColors>,
    sprite_assets: Res<SpriteAssets>,
) {
    spawn_menu_button(&mut commands, &button_colors, &font_assets.press_start, MenuButton::Play(0), Vec2::new(10., 80.), Vec2::new(19., 8.)); 
    spawn_menu_button(&mut commands, &button_colors, &font_assets.press_start, MenuButton::LevelSelect, Vec2::new(30., 80.),Vec2::new(19., 8.)); 
    spawn_menu_button(&mut commands, &button_colors, &font_assets.press_start, MenuButton::Options, Vec2::new(50., 80.),Vec2::new(19., 8.)); 
    spawn_menu_button(&mut commands, &button_colors, &font_assets.press_start, MenuButton::Quit, Vec2::new(70., 80.), Vec2::new(19., 8.)); 
   
    commands.spawn(ImageBundle {
        image: UiImage(sprite_assets.texture_title.clone()),
        image_mode: ImageMode::KeepAspect,
        style: Style {
            size: Size::new(Val::Percent(40.), Val::Auto),
            position: UiRect {
                left: Val::Percent(30.),
                top: Val::Percent(5.),
                ..Default::default()
            },
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        ..Default::default()
   }).insert(MenuElement)
   ;
}

fn esc_to_menu(
    mut actions: ResMut<Actions>,
    mut app_state: ResMut<State<GameState>>,
) {
    if actions.pause {
        actions.pause = false;
        app_state.push(GameState::Paused).unwrap();
    }
}

fn esc_to_resume(
    mut actions: ResMut<Actions>,
    mut state: ResMut<State<GameState>>,
) {
    if actions.pause {
        actions.pause = false;
        state.pop().unwrap();
    }
}

fn back_to_menu(
    mut actions: ResMut<Actions>,
    mut state: ResMut<State<GameState>>,
) {
    if actions.back {
        actions.back = false;
        state.replace(GameState::Menu).unwrap();
    }
}

fn setup_pause_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    button_colors: Res<ButtonColors>,
    sprite_assets: Res<SpriteAssets>,
) {
    commands.spawn(ImageBundle {
        image: UiImage(sprite_assets.texture_pause_background.clone()),
        image_mode: ImageMode::KeepAspect,
        z_index: ZIndex::Global(-1),
        style: Style {
            size: Size::new(Val::Auto, Val::Percent(100.)),
            position: UiRect {
                left: Val::Percent(0.),
                top: Val::Percent(0.),
                ..Default::default()
            },
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        ..Default::default()
   }).insert(MenuElement)
   ;
    
    spawn_menu_button(&mut commands, &button_colors, &font_assets.press_start, 
        MenuButton::Resume, Vec2::new(10., 30.), Vec2::new(19., 8.)); 
    spawn_menu_button(&mut commands, &button_colors, &font_assets.press_start, 
        MenuButton::LevelSelect, Vec2::new(10., 40.),Vec2::new(19., 8.)); 
    spawn_menu_button(&mut commands, &button_colors, &font_assets.press_start, 
        MenuButton::Menu, Vec2::new(10., 50.),Vec2::new(19., 8.)); 
    spawn_menu_button(&mut commands, &button_colors, &font_assets.press_start, 
        MenuButton::Quit, Vec2::new(10., 60.), Vec2::new(19., 8.)); 
}

fn setup_level_select(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    button_colors: Res<ButtonColors>,
) {
    let level_sequence = [0, 1, 2, 3, 5, 4, 6, 7, 8, 9, 12, 11, 10, 13, 14];
    let mut sequence_index = 0;
    let size = Vec2::new(16., 8.);
    let spacing = size + Vec2::new(2., 2.);
    let base_pos = Vec2::new(15., 25.);
    
    let columns = 4;
    
    for level_index in level_sequence {
        let col = sequence_index % columns;
        let row = sequence_index / columns;
        let pos = base_pos + Vec2::new(spacing.x * (col as f32), spacing.y * (row as f32));
        
        spawn_level_select_button(&mut commands, &button_colors, &font_assets.press_start, level_index, sequence_index + 1, pos, size);
        
        sequence_index += 1;
    }
    
    spawn_menu_button(&mut commands, &button_colors, &font_assets.press_start, MenuButton::Menu, Vec2::new(10., 80.), Vec2::new(19., 8.));
}

fn spawn_level_select_button(
    commands: &mut Commands, 
    button_colors: &ButtonColors, 
    font: &Handle<Font>, 
    true_level_index: usize, 
    level_number: i32, 
    position: Vec2, size: Vec2,
) {
    let position = UiRect {
        left: Val::Percent(position.x),
        top: Val::Percent(position.y),
        ..Default::default()
    };
    
    let label_string = "Level ".to_string() + level_number.to_string().as_str();
    
    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Percent(size.x), Val::Percent(size.y)),
                position,
                position_type: PositionType::Absolute,
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                ..Default::default()
            },
            background_color: button_colors.normal.into(),
            ..Default::default()
        })
        .insert(MenuButton::Play(true_level_index))
        .insert(Focusable::default())
        .insert(MenuElement)
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: label_string,
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 24.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    }],
                    alignment: Default::default(),
                },
                ..Default::default()
            });
        });
}

fn spawn_menu_button(
    commands: &mut Commands, 
    button_colors: &ButtonColors, 
    font: &Handle<Font>, 
    button_type: MenuButton, 
    position: Vec2, size: Vec2,
    ) {
    let position = UiRect {
        left: Val::Percent(position.x),
        top: Val::Percent(position.y),
        ..Default::default()
    };
    
    let label_string = match button_type {
        MenuButton::Play(index) => { 
            if index == 0 {
                "New Game"
            }
            else {
                "Resume"
            }
        },
        MenuButton::LevelSelect => "Level\nSelect",
        MenuButton::Options => "Options",
        MenuButton::Menu => "Main Menu",
        MenuButton::Resume => "Resume",
        MenuButton::Quit => "Quit",
    }.to_string();
    
    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Percent(size.x), Val::Percent(size.y)),
                position,
                position_type: PositionType::Absolute,
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                ..Default::default()
            },
            background_color: button_colors.normal.into(),
            ..Default::default()
        })
        .insert(button_type)
        .insert(Focusable::default())
        .insert(MenuElement)
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: label_string,
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 24.0,
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
    button_colors: Res<ButtonColors>,
) {
    for (focus, mut material) in interaction_query.iter_mut() {
        let color = match focus.state() {
            FocusState::Focused => button_colors.highlight,
            FocusState::Active => button_colors.active,
            _ => button_colors.normal,
        };
        *material = color.into();
    }
}

// fn print_nav_events(mut events: EventReader<NavEvent>) {
//     for event in events.iter() {
//         println!("{:?}", event);
//     }
// }

fn button_nav_events(
    mut events: EventReader<NavEvent>,
    buttons: Query<&MenuButton>,
    mut state: ResMut<State<GameState>>,
    mut exit: EventWriter<AppExit>,
    mut level_selection: ResMut<LevelSelection>,
    ) {
    for event in events.iter() {
        match event {
            NavEvent::NoChanges { from, request } => {
                if *request == NavRequest::Action {
                    if let Ok(button) = buttons.get(from.first().clone()) {
                        match button {
                            MenuButton::Play(level_index) => { 
                                state.set(GameState::Playing).unwrap();
                                *level_selection = LevelSelection::Index(*level_index);
                            },
                            MenuButton::LevelSelect => state.set(GameState::LevelSelect).unwrap(),
                            MenuButton::Options => (),
                            MenuButton::Menu => state.replace(GameState::Menu).unwrap(),
                            MenuButton::Resume => state.pop().unwrap(),
                            MenuButton::Quit => exit.send(AppExit),
                        };
                    }
                }
            },
            _ => ()
        }
    }
}

fn cleanup_menu(mut commands: Commands, buttons: Query<Entity, With<MenuElement>>) {
    for button in buttons.iter() {
        commands.entity(button).despawn_recursive();
    }
}
