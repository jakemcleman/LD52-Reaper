use crate::loading::*;
use crate::GameState;
use bevy::prelude::*;
use bevy::app::AppExit;
use bevy::ui::widget::ImageMode;
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
    Play,
    LevelSelect,
    Options,
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
   spawn_menu_button(&mut commands, &button_colors, &font_assets.press_start, MenuButton::Play, Vec2::new(10., 80.), Vec2::new(19., 8.)); 
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
        MenuButton::Play => "Play",
        MenuButton::LevelSelect => "Level\nSelect",
        MenuButton::Options => "Options",
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
                            MenuButton::LevelSelect => (),
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

fn cleanup_menu(mut commands: Commands, buttons: Query<Entity, With<MenuElement>>) {
    for button in buttons.iter() {
        commands.entity(button).despawn_recursive();
    }
}
