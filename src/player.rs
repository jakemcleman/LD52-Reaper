use crate::actions::Actions;
use crate::actor::*;
use crate::door::Door;
use crate::sprite_anim::SpriteAnimator;
use crate::world::{ChangeLevelEvent, Labeled, ReloadWorldEvent};
use crate::GameState;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PlayerPlugin;

#[derive(Component, Default, Clone)]
pub struct Player;

#[derive(Component, Debug, Default, Clone)]
pub struct TouchDeath;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(player_inputs)
                .after(actor_status)
                .after(crate::actions::set_movement_actions)
                .before(actor_movement),
        )
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(player_death))
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(player_win));
    }
}

#[derive(Clone, Default, Bundle)]
pub struct PlayerBundle {
    #[bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    pub sprite_animator: SpriteAnimator,
    pub player: Player,
    pub rigidbody: RigidBody,
    pub collider: Collider,
    pub label: Labeled,
    pub active_events: ActiveEvents,
    pub controller: KinematicCharacterController,
    pub actor: Actor,
    pub actor_status: ActorStatus,
    pub actor_anim: ActorAnimationStates,
    pub actor_audio: ActorAudio,
    pub actor_effects: ActorEffects,
    pub pickup_collector: crate::pickup::PickupCollector,
    pub squashy: Squashy,
}

impl LdtkEntity for PlayerBundle {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _layer_instance: &LayerInstance,
        _tileset: Option<&Handle<Image>>,
        _tileset_definition: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        texture_atlases: &mut Assets<TextureAtlas>,
    ) -> Self {
        let texture_handle = asset_server.load("sprites/sam1.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(48., 48.), 4, 4, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        let scythe_texture_handle = asset_server.load("sprites/scythe1.png");
        let scythe_texture_atlas = 
            TextureAtlas::from_grid(scythe_texture_handle, Vec2::new(48., 48.), 4, 6, None, None);
        let scythe_atlas_handle = texture_atlases.add(scythe_texture_atlas);

        let launch_texture_handle = asset_server.load("sprites/launch_effect.png");
        let launch_texture_atlas = 
            TextureAtlas::from_grid(launch_texture_handle, Vec2::new(32., 32.), 4, 1, None, None);
        let launch_atlas_handle = texture_atlases.add(launch_texture_atlas);

        let pickup_texture_handle = asset_server.load("sprites/soulburst.png");
        let pickup_texture_atlas = 
            TextureAtlas::from_grid(pickup_texture_handle, Vec2::new(32., 32.), 4, 1, None, None);
        let pickup_atlas_handle = texture_atlases.add(pickup_texture_atlas);

        let mut actor = Actor::default();

        actor.can_attack = true;
        actor.attack_sprite = Some(scythe_atlas_handle);

        for field in entity_instance.field_instances.iter() {
            match field.identifier.as_str() {
                "Speed" => {
                    if let FieldValue::Float(Some(value)) = field.value {
                        actor.move_speed = value;
                    }
                }
                "Drag" => {
                    if let FieldValue::Float(Some(value)) = field.value {
                        actor.drag = value;
                    }
                }
                "Acceleration" => {
                    if let FieldValue::Float(Some(value)) = field.value {
                        actor.accel = value;
                    }
                }
                "Decceleration" => {
                    if let FieldValue::Float(Some(value)) = field.value {
                        actor.deccel = value;
                    }
                }
                "UpGravity" => {
                    if let FieldValue::Float(Some(value)) = field.value {
                        actor.up_gravity = value;
                    }
                }
                "DownGravity" => {
                    if let FieldValue::Float(Some(value)) = field.value {
                        actor.down_gravity = value;
                    }
                }
                "JumpPower" => {
                    if let FieldValue::Float(Some(value)) = field.value {
                        actor.jump_speed = value;
                    }
                }
                "JumpTime" => {
                    if let FieldValue::Float(Some(value)) = field.value {
                        actor.jump_time = value;
                    }
                }
                "CanAttack" => {
                    if let FieldValue::Bool(value) = field.value {
                        actor.can_attack = value
                    }
                }
                "AttackTime" => {
                    if let FieldValue::Float(Some(value)) = field.value {
                        actor.attack_time = value;
                    }
                }
                "AttackRange" => {
                    if let FieldValue::Float(Some(value)) = field.value {
                        actor.attack_range = value;
                    }
                }
                unknown => println!("Unknown field \"{}\" on LDtk player object!", unknown),
            }
        }

        PlayerBundle {
            sprite_sheet_bundle: SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
                ..Default::default()
            },
            sprite_animator: crate::sprite_anim::SpriteAnimator::new(0, 3, 4, 0.2, true, true),
            player: Player,
            rigidbody: RigidBody::KinematicPositionBased,
            collider: Collider::capsule_y(5., 5.),
            label: Labeled {
                name: String::from("player"),
            },
            active_events: ActiveEvents::COLLISION_EVENTS,
            controller: KinematicCharacterController {
                offset: CharacterLength::Absolute(0.5),
                autostep: None,
                filter_flags: QueryFilterFlags::EXCLUDE_SENSORS,
                ..Default::default()
            },
            actor,
            actor_status: ActorStatus {
                grounded: false,
                facing_left: false,
                velocity: Vec2::ZERO,
                air_timer: 0.,
                attack_direction: None,
                attack_timer: 0.,
                left_wall: false,
                right_wall: false,
                event: None,
                last_dt: 1.,
            },
            actor_anim: ActorAnimationStates {
                idle_row: 0,
                run_row: 1,
                jump_row: 2,
                fall_row: 3,
                attack_row: 4,
            },
            actor_audio: ActorAudio {
                jump: asset_server.load("audio/jump3.ogg"),
                land: asset_server.load("audio/land2.ogg"),
                attack: asset_server.load("audio/attack1.ogg"),
                hit: asset_server.load("audio/hit.ogg"),
                death: asset_server.load("audio/death1.ogg"),
                pickup: asset_server.load("audio/soul_pickup2.ogg"),
                unlocked: asset_server.load("audio/unlocked2.ogg"),
                victory: asset_server.load("audio/victory2.ogg"),
            },
            actor_effects: ActorEffects {
                jump: launch_atlas_handle,
                pickup: pickup_atlas_handle,
            },
            pickup_collector: crate::pickup::PickupCollector,
            squashy: Squashy {
                base_scale: Vec2::new(48., 32.),
                restore_time: 0.15,
                squash_scale: Vec2::new(1.2, 0.8),
                squash_time: 0.05,
                stretch_scale: Vec2::new(0.8, 1.1),
                stretch_time: 0.05,
                state: None,
                state_time: 0.,
                from_pos: Vec2::ONE,
            },
        }
    }
}

fn player_inputs(
    actions: Res<Actions>,
    mut player_query: Query<(&mut Actor, &ActorStatus), With<Player>>,
) {
    let input: Vec2 = Vec2::new(actions.player_movement.x, actions.player_movement.y);
    for (mut actor, status) in &mut player_query {
        actor.jump_input = actions.jump;
        actor.can_jump = status.grounded || status.air_timer < actor.jump_time;

        if actions.attack && status.attack_direction.is_none() {
            actor.attack_input = Some(input);
        }
        else {
            actor.attack_input = None;
        }

        actor.move_input = input.x;
    }
}

fn player_win(
    mut next_level_writer: EventWriter<ChangeLevelEvent>,
    rapier_context: Res<RapierContext>,
    mut doors: Query<&mut Door>,
    mut player_query: Query<(&Transform, &mut ActorStatus), With<Player>>,
) {
    for (transform, mut status) in &mut player_query {
        let shape = Collider::capsule_y(5.5, 5.5);
        let filter = QueryFilter::new();
        let shape_pos = transform.translation.truncate();

        rapier_context.intersections_with_shape(shape_pos, 0., &shape, filter, |entity| -> bool {
            if let Ok(mut door) = doors.get_mut(entity) {
                if door.required_souls == 0 {
                    next_level_writer.send(ChangeLevelEvent {
                        index: door.next_level,
                        completed: true,
                        win_game: door.next_level == 32767,
                    });
                    status.event = Some(ActorEvent::Win);

                    door.required_souls = usize::MAX;

                    return false; // no need to keep looking
                }
            }
            true
        });
    }
}

fn player_death(
    mut player_query: Query<(&Transform, &mut ActorStatus), With<Player>>,
    enemies_query: Query<Entity, With<TouchDeath>>,
    mut reload_writer: EventWriter<ReloadWorldEvent>,
    rapier_context: Res<RapierContext>,
) {
    for (transform, mut status) in &mut player_query {
        let shape = Collider::capsule_y(5.5, 5.5);
        let filter = QueryFilter::new();
        let shape_pos = transform.translation.truncate();

        rapier_context.intersections_with_shape(shape_pos, 0., &shape, filter, |entity| -> bool {
            if let Ok(_touched_ent) = enemies_query.get(entity) {
                reload_writer.send(ReloadWorldEvent);
                status.event = Some(ActorEvent::Died);
                return false; // no need to keep looking
            }
            true
        });
    }
}
