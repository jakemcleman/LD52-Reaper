use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::pickup::PickupEvent;
use crate::{pickup, GameState};
use crate::{
    pickup::{Pickup, PickupType},
    sprite_anim::SpriteAnimator,
    world::Labeled,
};

pub struct SoulPlugin;

#[derive(Component, Default, Clone)]
pub struct Soul {
    pub can_move: bool,
    pub move_speed: f32,
    pub accel: f32,
    pub velocity: Vec2,
    pub from_ghost: bool,
}

#[derive(Clone, Default, Bundle)]
pub struct SoulBundle {
    #[bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    pub sprite_animator: SpriteAnimator,
    pub soul: Soul,
    pub rigidbody: RigidBody,
    pub collider: Collider,
    pub sensor: Sensor,
    pub label: Labeled,
    pub controller: KinematicCharacterController,
    pub pickup: Pickup,
}

pub struct CollectedSoulEvent;

impl Plugin for SoulPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollectedSoulEvent>().add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(soul_movement)
                .with_system(soul_pickups)
                .after(crate::pickup::check_for_pickups),
        );
    }
}

fn soul_pickups(
    mut pickup_reader: EventReader<PickupEvent>,
    mut soul_writer: EventWriter<CollectedSoulEvent>,
) {
    for pickup_ev in pickup_reader.iter() {
        if pickup_ev.pickup_type == pickup::PickupType::Soul {
            soul_writer.send(CollectedSoulEvent);
        }
    }
}

impl LdtkEntity for SoulBundle {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _layer_instance: &LayerInstance,
        _tileset: Option<&Handle<Image>>,
        _tileset_definition: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        texture_atlases: &mut Assets<TextureAtlas>,
    ) -> Self {
        let texture_handle = asset_server.load("sprites/soul.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(20., 20.), 4, 1, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        let mut soul = Soul {
            can_move: false,
            move_speed: 120.,
            accel: 40.,
            velocity: Vec2::ZERO,
            from_ghost: false,
        };

        for field in entity_instance.field_instances.iter() {
            match field.identifier.as_str() {
                "Move" => {
                    if let FieldValue::Bool(value) = field.value {
                        soul.can_move = value;
                    }
                }
                "Speed" => {
                    if let FieldValue::Float(Some(value)) = field.value {
                        soul.move_speed = value;
                    }
                }
                "Acceleration" => {
                    if let FieldValue::Float(Some(value)) = field.value {
                        soul.accel = value;
                    }
                }
                unknown => println!("Unknown field \"{}\" on LDtk soul object!", unknown),
            }
        }

        SoulBundle {
            sprite_sheet_bundle: SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
                ..Default::default()
            },
            sprite_animator: crate::sprite_anim::SpriteAnimator::new(0, 3, 4, 0.2, true),
            soul,
            rigidbody: RigidBody::KinematicPositionBased,
            collider: Collider::ball(5.),
            sensor: Sensor,
            label: Labeled {
                name: String::from("soul"),
            },
            controller: KinematicCharacterController {
                offset: CharacterLength::Absolute(0.1),
                autostep: None,
                filter_flags: QueryFilterFlags::EXCLUDE_SENSORS,
                ..Default::default()
            },
            pickup: Pickup {
                pickup_type: Some(PickupType::Soul),
            },
        }
    }
}

fn soul_movement(
    time: Res<Time>,
    mut soul_query: Query<(
        Entity,
        &Transform,
        &mut Soul,
        &mut KinematicCharacterController,
    )>,
    spike_query: Query<&crate::player::TouchDeath, Without<crate::ghost::Ghost>>,
    player_query: Query<&Transform, With<crate::player::Player>>,
    rapier_context: Res<RapierContext>,
) {
    for (entity, transform, mut soul, mut controller) in &mut soul_query {
        if soul.can_move {
            if let Ok(player_transform) = player_query.get_single() {
                let comfortable_distance = 256.;
                let dir_from_player =
                    (transform.translation - player_transform.translation).truncate();
                let dist_from_player = dir_from_player.length();
                let flee_priority = (comfortable_distance / dist_from_player).powi(3);
                let flee_vec = dir_from_player.normalize_or_zero();

                let max_space = 256.;
                let max_height = 64.;

                let cast_filter = QueryFilter::new()
                    .exclude_sensors()
                    .exclude_collider(entity);
                let shape = Collider::ball(4.9);
                let shape_pos = transform.translation.truncate();

                let down_cast = rapier_context.cast_shape(
                    shape_pos,
                    0.,
                    Vec2::NEG_Y,
                    &shape,
                    max_space,
                    cast_filter,
                );
                let up_cast = rapier_context.cast_shape(
                    shape_pos,
                    0.,
                    Vec2::Y,
                    &shape,
                    max_space,
                    cast_filter,
                );
                let left_cast = rapier_context.cast_shape(
                    shape_pos,
                    0.,
                    Vec2::NEG_X,
                    &shape,
                    max_space,
                    cast_filter,
                );
                let right_cast = rapier_context.cast_shape(
                    shape_pos,
                    0.,
                    Vec2::X,
                    &shape,
                    max_space,
                    cast_filter,
                );

                let down_space = if let Some((entity, toi)) = down_cast {
                    if spike_query.contains(entity) {
                        0.25 * toi.toi
                    } else {
                        toi.toi
                    }
                } else {
                    max_space
                };
                let up_space = if let Some((entity, toi)) = up_cast {
                    if spike_query.contains(entity) {
                        0.25 * toi.toi
                    } else {
                        toi.toi
                    }
                } else {
                    max_space
                };
                let left_space = if let Some((entity, toi)) = left_cast {
                    if spike_query.contains(entity) {
                        0.25 * toi.toi
                    } else {
                        toi.toi
                    }
                } else {
                    max_space
                };
                let right_space = if let Some((entity, toi)) = right_cast {
                    if spike_query.contains(entity) {
                        0.25 * toi.toi
                    } else {
                        toi.toi
                    }
                } else {
                    max_space
                };

                //println!("left: {0} right: {1} up: {2} down: {3}", left_space, right_space, up_space, down_space);

                let min_space = down_space.min(up_space).min(left_space).min(right_space);

                let centering_vec =
                    Vec2::new(right_space - left_space, up_space - down_space).normalize_or_zero();
                let centering_priority = max_space / (min_space + 1.);

                let height_restoring_vec = Vec2::new(0., -1.);
                let height_priority = (down_space / max_height).powi(3);

                let idle_vec = Vec2::new(
                    f32::sin(time.elapsed_seconds()),
                    f32::cos(time.elapsed_seconds()),
                ) * 2.;

                //println!("flee: {0}, spacing: {1}, height fix: {2}", flee_priority, centering_priority, height_priority);
                //println!("flee vec: {0} spacing vec: {1}", flee_vec, centering_vec);

                let total_vec = (idle_vec
                    + (flee_priority * flee_vec)
                    + (centering_priority * centering_vec)
                    + (height_priority * height_restoring_vec))
                    .normalize();

                let accel = soul.accel * time.delta_seconds();
                soul.velocity += total_vec * accel;
                soul.velocity = soul.velocity.clamp_length_max(soul.move_speed);

                if (up_space < 1. && soul.velocity.y > 0.1)
                    || (down_space < 1. && soul.velocity.y < -0.1)
                {
                    soul.velocity.y = -0.5 * soul.velocity.y;
                }

                if (right_space < 1. && soul.velocity.x > 0.1)
                    || (left_space < 1. && soul.velocity.x < -0.1)
                {
                    soul.velocity.x = -0.5 * soul.velocity.x;
                }

                controller.translation = Some(soul.velocity * time.delta_seconds());
            }
        } else {
            controller.translation = Some(
                time.delta_seconds()
                    * Vec2::new(
                        f32::sin(time.elapsed_seconds()),
                        f32::cos(time.elapsed_seconds()),
                    )
                    * 2.,
            );
        }
    }
}
