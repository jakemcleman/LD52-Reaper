use crate::GameState;
use crate::sprite_anim::SpriteAnimator;
use crate::player::TouchDeath;
use crate::actor::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct GhostPlugin;

#[derive(Component, Default, Clone)]
pub struct Ghost {
    move_left: bool,
}

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for GhostPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_update(GameState::Playing)
                .with_system(ghost_movement)
                .after(actor_status)
                .before(actor_movement)
            )
        ;
    }
}

#[derive(Clone, Default, Bundle)]
pub struct GhostBundle {
    #[bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    pub sprite_animator: SpriteAnimator,
    pub ghost: Ghost,
    pub rigidbody: RigidBody,
    pub collider: Collider,
    pub controller: KinematicCharacterController,
    pub actor: Actor,
    pub actor_status: ActorStatus,
    pub death: TouchDeath,
}

impl LdtkEntity for GhostBundle {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _layer_instance: &LayerInstance,
        _tileset: Option<&Handle<Image>>,
        _tileset_definition: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        texture_atlases: &mut Assets<TextureAtlas>,
    ) -> Self {
        let texture_handle = asset_server.load("sprites/ghost.png");
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle, 
            Vec2::new(24., 24.), 
            4, 1, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        
        let mut actor = Actor::default();
        let mut ghost = Ghost::default();
        
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
                "StartLeft" => if let FieldValue::Bool(value) = field.value {
                    ghost.move_left = value;
                },
                unknown => println!("Unknown field \"{}\" on LDtk ghost object!", unknown),
            }
        }
        
        GhostBundle {
            sprite_sheet_bundle: SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
                ..Default::default()
            },
            sprite_animator: crate::sprite_anim::SpriteAnimator::new(texture_atlas_handle.clone(), 0, 3, 4, 0.2, true),
            ghost,
            rigidbody: RigidBody::KinematicPositionBased,
            collider: Collider::capsule_y(5., 5.),
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
            death: TouchDeath,
        }
    }
}

fn ghost_movement(
    mut ghost_query: Query<(&mut Ghost, &mut Actor, &ActorStatus)>,
    ) {
    for (mut ghost, mut actor, status) in &mut ghost_query {
        if ghost.move_left && status.left_wall {
            ghost.move_left = false;
        }
        else if !ghost.move_left && status.right_wall {
            ghost.move_left = true;
        }
        
        actor.move_input = if ghost.move_left { -1. } else { 1. };
    }
}