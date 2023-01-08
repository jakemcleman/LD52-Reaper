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

#[derive(Component, Default, Clone)]
pub struct Soul {
    can_move: bool,
    move_speed: f32,
    accel: f32,
    velocity: Vec2,
    pub next_level: usize,
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
            .add_system_set(SystemSet::on_update(GameState::Playing)
                .with_system(soul_movement)
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

#[derive(Clone, Default, Bundle)]
pub struct SoulBundle {
    #[bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    pub sprite_animator: SpriteAnimator,
    pub soul: Soul,
    pub rigidbody: RigidBody,
    pub collider: Collider,
    pub controller: KinematicCharacterController,
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
            collider: Collider::capsule_y(5.0,5.0),
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
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle, 
            Vec2::new(20., 20.), 
            4, 1, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        
        let mut soul = Soul {
            can_move: false,
            next_level: 0,
            move_speed: 80.,
            accel: 20.,
            velocity: Vec2::ZERO,
        };
        
        for field in entity_instance.field_instances.iter() {
            match field.identifier.as_str() {
                "Move" => if let FieldValue::Bool(value) = field.value {
                    soul.can_move = value;
                },
                "NextLevel" => if let FieldValue::Int(Some(value)) = field.value {
                    soul.next_level = value as usize;
                },
                "Speed" => if let FieldValue::Float(Some(value)) = field.value {
                    soul.move_speed = value;
                },
                "Acceleration" => if let FieldValue::Float(Some(value)) = field.value {
                    soul.accel = value;
                },
                unknown => println!("Unknown field \"{}\" on LDtk soul object!", unknown),
            }
        }
        
        SoulBundle {
            sprite_sheet_bundle: SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
                ..Default::default()
            },
            sprite_animator: crate::sprite_anim::SpriteAnimator::new(texture_atlas_handle.clone(), 0, 3, 4, 0.2, true),
            soul,
            rigidbody: RigidBody::KinematicPositionBased,
            collider: Collider::ball(5.),
            controller: KinematicCharacterController {
                offset: CharacterLength::Absolute(0.1),
                autostep: None,
                ..Default::default()
            },
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

fn soul_movement(
    time: Res<Time>,
    mut soul_query: Query<(&Transform, &mut Soul, &mut KinematicCharacterController)>,
    player_query: Query<&Transform, With<crate::player::Player>>,
    rapier_context: Res<RapierContext>,
) {
    
    for (transform, mut soul, mut controller) in &mut soul_query {
        if soul.can_move {
            if let Ok(player_transform) = player_query.get_single() {
                let comfortable_distance = 256.;
                let dir_from_player = (transform.translation - player_transform.translation).truncate();
                let dist_from_player = dir_from_player.length();
                let flee_priority = (comfortable_distance / dist_from_player).powi(3);
                let flee_vec = dir_from_player.normalize_or_zero();
                
                let max_space = 256.;
                let max_height = 64.;
                
                let cast_filter = QueryFilter::only_fixed();
                let shape = Collider::ball(4.9);
                let shape_pos = transform.translation.truncate();
                
                let down_cast = rapier_context.cast_shape (shape_pos, 0., Vec2::NEG_Y, &shape, max_space, cast_filter);
                let up_cast = rapier_context.cast_shape   (shape_pos, 0., Vec2::Y,     &shape, max_space, cast_filter);
                let left_cast = rapier_context.cast_shape (shape_pos, 0., Vec2::NEG_X, &shape, max_space, cast_filter);
                let right_cast = rapier_context.cast_shape(shape_pos, 0., Vec2::X,     &shape, max_space, cast_filter);
            
                let down_space = if let Some((_, toi)) = down_cast { toi.toi } else { max_space };
                let up_space = if let Some((_, toi)) = up_cast { toi.toi } else { max_space };
                let left_space = if let Some((_, toi)) = left_cast { toi.toi} else { max_space };
                let right_space = if let Some((_, toi)) = right_cast { toi.toi } else { max_space };
                
                //println!("left: {0} right: {1} up: {2} down: {3}", left_space, right_space, up_space, down_space);
                
                let min_space = down_space.min(up_space).min(left_space).min(right_space);
                
                let centering_vec = Vec2::new(right_space - left_space, up_space - down_space).normalize_or_zero();
                let centering_priority = max_space / (min_space + 1.);
                    
                let height_restoring_vec = Vec2::new(0., -1. );
                let height_priority = (down_space / max_height).powi(4);
                
                let idle_vec = Vec2::new(f32::sin(time.elapsed_seconds()), f32::cos(time.elapsed_seconds())) * 2.;
                
                //println!("flee: {0}, spacing: {1}, height fix: {2}", flee_priority, centering_priority, height_priority);
                //println!("flee vec: {0} spacing vec: {1}", flee_vec, centering_vec);
                
                let total_vec = (idle_vec
                    + (flee_priority * flee_vec)
                    + (centering_priority * centering_vec)
                    + (height_priority * height_restoring_vec)).normalize();
                    
                let accel = soul.accel * time.delta_seconds();
                soul.velocity += total_vec * accel;
                soul.velocity = soul.velocity.clamp_length_max(soul.move_speed);
                    
                controller.translation = Some(soul.velocity * time.delta_seconds());
            }
        }
        else {
            controller.translation = Some(
                time.delta_seconds() 
                * Vec2::new(f32::sin(time.elapsed_seconds()), f32::cos(time.elapsed_seconds()))
                * 2.
                );
        }
    }
}