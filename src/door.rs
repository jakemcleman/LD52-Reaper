use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::loading::AudioAssets;

use crate::{soul::CollectedSoulEvent, sprite_anim::SpriteAnimator, world::Labeled, GameState};

pub struct DoorPlugin;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Door {
    pub next_level: usize,
    pub required_souls: usize,
}

#[derive(Clone, Default, Bundle)]
pub struct DoorBundle {
    #[bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    pub sprite_animator: SpriteAnimator,
    pub collider: Collider,
    pub label: Labeled,
    pub sensor: Sensor,
    pub active_events: ActiveEvents,
    pub door: Door,
}

impl Plugin for DoorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(add_souls_needed_text),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(update_souls_needed_text),
        );
    }
}

impl LdtkEntity for DoorBundle {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _layer_instance: &LayerInstance,
        _tileset: Option<&Handle<Image>>,
        _tileset_definition: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        texture_atlases: &mut Assets<TextureAtlas>,
    ) -> Self {
        let mut door = Door::default();

        for field in entity_instance.field_instances.iter() {
            match field.identifier.as_str() {
                "NextLevel" => {
                    if let FieldValue::Int(Some(value)) = field.value {
                        door.next_level = value as usize;
                    }
                }
                "SoulsNeeded" => {
                    if let FieldValue::Int(Some(value)) = field.value {
                        door.required_souls = value as usize;
                    }
                }
                unknown => println!("Unknown field \"{}\" on LDtk door object!", unknown),
            }
        }

        let texture_handle = asset_server.load("sprites/door_closed.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(16., 32.), 4, 1, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        DoorBundle {
            sprite_sheet_bundle: SpriteSheetBundle {
                transform: Transform::from_translation(Vec3::new(0., 0., 0.5)),
                texture_atlas: texture_atlas_handle,
                ..Default::default()
            },
            sprite_animator: SpriteAnimator::new(0, 3, 4, 0.2, true),
            collider: Collider::cuboid(8., 16.),
            label: Labeled {
                name: String::from("door to ") + door.next_level.to_string().as_str(),
            },
            sensor: Sensor,
            active_events: ActiveEvents::COLLISION_EVENTS,
            door,
        }
    }
}

fn add_souls_needed_text(
    doors: Query<(Entity, &Door), Added<Door>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/PressStart2P.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 16.,
        color: Color::WHITE,
    };

    for (entity, door) in &doors {
        commands
            .spawn(Text2dBundle {
                text: Text::from_section(door.required_souls.to_string(), text_style.clone())
                    .with_alignment(TextAlignment::CENTER),
                transform: Transform::from_xyz(0., 28., -1.),
                ..Default::default()
            })
            .set_parent(entity);
    }
}

fn update_souls_needed_text(
    mut soul_events: EventReader<CollectedSoulEvent>,
    mut text: Query<(&Parent, &mut Text)>,
    mut doors: Query<(&mut Door, &mut Handle<TextureAtlas>)>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
) {
    for _ in soul_events.iter() {
        for (parent, mut text) in text.iter_mut() {
            if let Ok((mut door, mut image_handle)) = doors.get_mut(parent.get()) {
                if door.required_souls > 0 {
                    // Should do nothing if already open
                    door.required_souls -= 1; // Apply new value for each soul event
                    text.sections[0].value = door.required_souls.to_string();

                    if door.required_souls == 0 {
                        let texture_handle = asset_server.load("sprites/door_open.png");
                        let texture_atlas = TextureAtlas::from_grid(
                            texture_handle,
                            Vec2::new(16., 32.),
                            4,
                            1,
                            None,
                            None,
                        );
                        let texture_atlas_handle = texture_atlases.add(texture_atlas);
                        *image_handle = texture_atlas_handle.clone();

                        audio.play(audio_assets.unlocked.clone());
                    }
                }
            }
        }
    }
}
