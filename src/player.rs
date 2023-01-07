use crate::actions::Actions;
use crate::GameState;
use crate::sprite_anim::SpriteAnimator;
use crate::actor::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PlayerPlugin;

#[derive(Component, Default, Clone)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(player_inputs))
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
    pub controller: KinematicCharacterController,
    pub actor: Actor,
    pub actor_status: ActorStatus,
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
        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(48., 32.), 4, 1, None, None);
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
                "Gravity" => if let FieldValue::Float(Some(value)) = field.value {
                    actor.gravity = value;
                },
                "Jump Power" => if let FieldValue::Float(Some(value)) = field.value {
                    actor.jump_speed = value;
                },
                "Jump Time" => if let FieldValue::Float(Some(value)) = field.value {
                    actor.jump_time = value;
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
            collider: Collider::capsule_y(7., 6.),
            controller: KinematicCharacterController {
                offset: CharacterLength::Relative(0.05),
                ..Default::default()
            },
            actor,
            actor_status: ActorStatus {
                grounded: false,
                velocity: Vec2::ZERO,
                air_timer: 0.,
            }
           
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
        actor.move_input = input.x;
        
        if status.grounded || status.air_timer < actor.jump_time {
            actor.jump_input = actions.jump;
        }
        else {
            actor.jump_input = false;
        }
    }
}