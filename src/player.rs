use crate::actions::Actions;
use crate::{GameState, ghost};
use crate::sprite_anim::SpriteAnimator;
use crate::actor::*;
use crate::world::{ReloadWorldEvent, ChangeLevelEvent};
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
                offset: CharacterLength::Absolute(0.2),
                autostep: None,
                ..Default::default()
            },
            actor,
            actor_status: ActorStatus {
                grounded: false,
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
               death: asset_server.load("audio/death1.ogg"),
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

fn player_win(
    mut player_query: Query<(&KinematicCharacterControllerOutput, &mut ActorStatus), With<Player>>,
    souls_query: Query<(Option<&KinematicCharacterControllerOutput>, &ghost::Soul)>,
    mut next_level_writer: EventWriter<ChangeLevelEvent>,
) {
    for (controller_out, mut status) in &mut player_query { 
        for collision in controller_out.collisions.iter() {
            if let Ok((_, soul)) = souls_query.get(collision.entity) {
                next_level_writer.send(ChangeLevelEvent::Index(soul.next_level));
                println!("reached level end");
                status.event = Some(ActorEvent::Win);
                return;
            }
        }
    }
    
    for (maybe_controller, soul) in &souls_query {
        if let Some(controller) = maybe_controller {
            for collision in controller.collisions.iter() {
                if let Ok((_, mut status)) = player_query.get_mut(collision.entity) {
                    next_level_writer.send(ChangeLevelEvent::Index(soul.next_level));
                    println!("reached level end");
                    status.event = Some(ActorEvent::Win);
                    return;
                }
            }
        }
    }
}

fn player_death(
    mut player_query: Query<(&KinematicCharacterControllerOutput, &mut ActorStatus), With<Player>>,
    enemies_query: Query<(Entity, Option<&KinematicCharacterControllerOutput>), With<TouchDeath>>,
    mut reload_writer: EventWriter<ReloadWorldEvent>,
) {
    for (controller_out, mut status) in &mut player_query { 
        for collision in controller_out.collisions.iter() {
            if enemies_query.contains(collision.entity) {
                println!("ded from player");
                reload_writer.send(ReloadWorldEvent);
                status.event = Some(ActorEvent::Died);
                return;
            }
        }
    }
    
    for (_en, maybe_controller) in &enemies_query {
        if let Some(controller_out) = maybe_controller {
            for collision in controller_out.collisions.iter() {
                if let Ok((_, mut status)) = player_query.get_mut(collision.entity) {
                    println!("ded from enemy");
                    reload_writer.send(ReloadWorldEvent);
                    status.event = Some(ActorEvent::Died);
                    return;
                }
            }
        }
    }
}