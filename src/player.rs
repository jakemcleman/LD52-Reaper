use crate::actions::Actions;
use crate::{GameState, ghost};
use crate::sprite_anim::SpriteAnimator;
use crate::actor::*;
use crate::world::{ReloadWorldEvent, ChangeLevelEvent, Door};
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
        app
            .add_system_set(SystemSet::on_update(GameState::Playing)
                .with_system(player_inputs)
                .after(actor_status)
                .before(actor_movement)
            )
            .add_system_set(SystemSet::on_update(GameState::Playing)
                .with_system(player_death)
            )
            .add_system_set(SystemSet::on_update(GameState::Playing)
                .with_system(player_collect_soul)
            )
            .add_system_set(SystemSet::on_update(GameState::Playing)
                .with_system(player_win)
            )
        ;
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
    pub active_events: ActiveEvents,
    pub controller: KinematicCharacterController,
    pub actor: Actor,
    pub actor_status: ActorStatus,
    pub actor_anim: ActorAnimationStates,
    pub actor_audio: ActorAudio,
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
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle, 
            Vec2::new(48., 32.), 
            4, 5, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        
        let mut actor = Actor::default();
        
        for field in entity_instance.field_instances.iter() {
            match field.identifier.as_str() {
                "Speed" => if let FieldValue::Float(Some(value)) = field.value {
                    actor.move_speed = value;
                },
                "Drag" => if let FieldValue::Float(Some(value)) = field.value {
                    actor.drag = value;
                },
                "Acceleration" => if let FieldValue::Float(Some(value)) = field.value {
                    actor.accel = value;
                },
                "Decceleration" => if let FieldValue::Float(Some(value)) = field.value {
                    actor.deccel = value;
                },
                "UpGravity" => if let FieldValue::Float(Some(value)) = field.value {
                    actor.up_gravity = value;
                },
                "DownGravity" => if let FieldValue::Float(Some(value)) = field.value {
                    actor.down_gravity = value;
                },
                "JumpPower" => if let FieldValue::Float(Some(value)) = field.value {
                    actor.jump_speed = value;
                },
                "JumpTime" => if let FieldValue::Float(Some(value)) = field.value {
                    actor.jump_time = value;
                },
                "AttackTime" => if let FieldValue::Float(Some(value)) = field.value {
                    actor.attack_time = value;
                },
                "AttackRange" => if let FieldValue::Float(Some(value)) = field.value {
                    actor.attack_range = value;
                },
                unknown => println!("Unknown field \"{}\" on LDtk player object!", unknown),
            }
        }
        
        PlayerBundle {
            sprite_sheet_bundle: SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
                ..Default::default()
            },
            sprite_animator: crate::sprite_anim::SpriteAnimator::new(texture_atlas_handle.clone(), 0, 3, 4, 0.2, true),
            player: Player,
            rigidbody: RigidBody::KinematicPositionBased,
            collider: Collider::capsule_y(5., 5.),
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
                attacking: false,
                attack_timer: 0.,
                left_wall: false,
                right_wall: false,
                event: None,
            },
           actor_anim: ActorAnimationStates {
               idle_row: 0,
               run_row: 1,
               jump_row: 2,
               fall_row: 3,
               attack_row: 4,
           },
           actor_audio: ActorAudio {
               jump: asset_server.load("audio/jump2.ogg"),
               land: asset_server.load("audio/land2.ogg"),
               attack: asset_server.load("audio/attack1.ogg"),
               hit: asset_server.load("audio/hit.ogg"),
               death: asset_server.load("audio/death1.ogg"),
               pickup: asset_server.load("audio/pickup1.ogg"),
               unlocked: asset_server.load("audio/unlocked.ogg"),
               victory: asset_server.load("audio/victory.ogg"),
           },
        }
    }
}

fn player_inputs(
    actions: Res<Actions>,
    mut player_query: Query<(&mut Actor, &ActorStatus), With<Player>>,
) {
    let input = Vec2::new(
        actions.player_movement.x,
        actions.player_movement.y,
    );
    for (mut actor, status) in &mut player_query {
        actor.jump_input = actions.jump;
        actor.can_jump = status.grounded || status.air_timer < actor.jump_time;
        
        actor.attack_input = actions.attack && !status.attacking;

        actor.move_input = input.x;
    }
}

fn player_collect_soul(
    mut player_query: Query<(&KinematicCharacterControllerOutput, &mut ActorStatus), With<Player>>,
    souls_query: Query<(Entity, Option<&KinematicCharacterControllerOutput>), With<ghost::Soul>>,
    mut doors: Query<&mut Door>,
    mut commands: Commands,
) {
    for (controller_out, mut status) in &mut player_query { 
        for collision in controller_out.collisions.iter() {
            if let Ok((entity, _)) = souls_query.get(collision.entity) {
                //next_level_writer.send(ChangeLevelEvent::Index(soul.next_level));
                let mut door = doors.single_mut();
                door.required_souls -= 1;
                
                if door.required_souls > 0 {
                    status.event = Some(ActorEvent::Pickup); 
                }
                else {
                    status.event = Some(ActorEvent::Unlock);
                }
                
                commands.entity(entity).despawn_recursive();
                return;
            }
        }
    }
    
    for (entity, maybe_controller) in &souls_query {
        if let Some(controller) = maybe_controller {
            for collision in controller.collisions.iter() {
                if let Ok((_, mut status)) = player_query.get_mut(collision.entity) {
                    //next_level_writer.send(ChangeLevelEvent::Index(soul.next_level));
                    status.event = Some(ActorEvent::Pickup);
                    commands.entity(entity).despawn_recursive();
                    doors.single_mut().required_souls -= 1;
                    return;
                }
            }
        }
    }
}

fn player_win(
    mut next_level_writer: EventWriter<ChangeLevelEvent>,
    rapier_context: Res<RapierContext>,
    doors: Query<&Door>,
    mut player_query: Query<(&Transform, &mut ActorStatus), With<Player>>,
) {
    for (transform, mut status) in &mut player_query { 
        let shape = Collider::capsule_y(5.5, 5.5);
        let filter = QueryFilter::new();
        let shape_pos = transform.translation.truncate();
        
        rapier_context.intersections_with_shape(shape_pos, 0., &shape, filter, |entity| -> bool {
            if let Ok(door) = doors.get(entity) {
                if door.required_souls == 0 {
                    println!("win player");
                    next_level_writer.send(ChangeLevelEvent::Index(door.next_level));
                    status.event = Some(ActorEvent::Win);
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
                println!("ded player");
                reload_writer.send(ReloadWorldEvent);
                status.event = Some(ActorEvent::Died);
                return false; // no need to keep looking
            }
            true
        });
    }
}