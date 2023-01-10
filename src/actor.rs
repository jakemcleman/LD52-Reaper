use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::{GameState, sprite_anim::SpriteAnimator};

pub struct ActorPlugin;

#[derive(Component, Clone)]
pub struct Actor {
    pub move_speed: f32,
    pub drag: f32,
    pub accel: f32,
    pub deccel: f32,
    pub up_gravity: f32,
    pub down_gravity: f32,
    pub jump_speed: f32,
    pub jump_time: f32,
    pub move_input: f32,
    pub jump_input: bool,
    pub can_jump: bool,
    pub attack_input: bool,
    pub attack_time: f32,
    pub attack_range: f32,
}

#[derive(Component, Default, Clone)]
pub struct ActorStatus {
    pub grounded: bool,
    pub velocity: Vec2,
    pub facing_left: bool,
    pub air_timer: f32,
    pub attacking: bool,
    pub attack_timer: f32,
    pub left_wall: bool,
    pub right_wall: bool,
    pub event: Option<ActorEvent>,
}

#[derive(Component, Default, Clone)]
pub struct ActorAnimationStates {
    pub idle_row: usize,
    pub run_row: usize,
    pub jump_row: usize,
    pub fall_row: usize,
    pub attack_row: usize,
}

#[derive(Component, Default, Clone)]
pub struct ActorAudio {
    pub jump: Handle<AudioSource>,
    pub land: Handle<AudioSource>,
    pub attack: Handle<AudioSource>,
    pub hit: Handle<AudioSource>,
    pub death: Handle<AudioSource>,
    pub pickup: Handle<AudioSource>,
    pub unlocked: Handle<AudioSource>,
    pub victory: Handle<AudioSource>,
}

#[derive(Component, Debug, Default, Clone)]
pub struct Scythable {
    pub scythed: bool,
    pub hit_from: Option<Vec2>,
}

#[derive(Clone)]
pub enum ActorEvent {
    Launched,
    Landed,
    Attack,
    Hit,
    Died,
    Win,
    Pickup,
    Unlock,
}

impl Plugin for ActorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(actor_status).before(actor_movement))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(actor_movement))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(actor_attack))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(actor_animations))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(actor_audio))
        ;
    }
}

impl Default for Actor {
    fn default() -> Self {
        Actor {
            move_speed: 120.,
            drag: 0.3,
            accel: 1000., 
            deccel: 2000.,
            up_gravity: 300.,
            down_gravity: 500.,
            jump_speed: 800.,
            jump_time: 0.2,
            move_input: 0.,
            jump_input: false,
            attack_time: 0.2,
            attack_input: false,
            attack_range: 16.0,
            can_jump: false,
        }
    }
}

pub fn actor_status(
    time: Res<Time>,
    mut actor_query: Query<(Entity, &Transform, &mut ActorStatus, &KinematicCharacterControllerOutput)>,
    rapier_context: Res<RapierContext>,
) {
    for (entity, transform, mut actor_status, controller_output) in &mut actor_query {
        if !actor_status.grounded && controller_output.grounded {
            actor_status.event = Some(ActorEvent::Landed);
        }
        
        if actor_status.attacking {
            actor_status.attack_timer += time.delta_seconds();
        }
        
        actor_status.grounded = controller_output.grounded;
        actor_status.velocity = controller_output.effective_translation / time.delta_seconds();
        
        if actor_status.grounded {
            actor_status.air_timer = 0.;
            actor_status.velocity.y = 0.;
        }
        else {
            actor_status.air_timer += time.delta_seconds();
        }
        
        let shape = Collider::capsule_y(4.5, 4.5);
        let shape_pos = transform.translation.truncate();
        let filter = QueryFilter::new().exclude_sensors().exclude_collider(entity);
        let distance = 1.0;
        
        if let Some((_, _)) = rapier_context.cast_shape(
            shape_pos, 0., Vec2::new(distance, 0.), &shape, 1., filter
            ) {
            actor_status.right_wall = true;
        }
        else {
            actor_status.right_wall = false;
        }
        
        if let Some((_, _)) = rapier_context.cast_shape(
            shape_pos, 0., Vec2::new(-distance, 0.), &shape, 1., filter
            ) {
            actor_status.left_wall = true;
        }
        else {
            actor_status.left_wall = false;
        }
    }
}

pub fn actor_attack(
    mut actor_query: Query<(&Transform, &Actor, &mut ActorStatus)>,
    mut target_query: Query<&mut Scythable>,
    rapier_context: Res<RapierContext>,
) {
    for (transform, actor, mut status) in &mut actor_query {
        if status.attacking {
            if status.attack_timer >= actor.attack_time {
                status.attacking = false;
            }
            
            let shape = Collider::ball(actor.attack_range);
            let attack_distance = actor.attack_range + 5.01;
            let attack_offset = if status.facing_left { -1. } else { 1. } * attack_distance * Vec2::X;
            let filter = QueryFilter::new();
            
            rapier_context.intersections_with_shape(
                transform.translation.truncate() + attack_offset, 0., &shape, filter, |entity| -> bool {
                    if let Ok(mut target) = target_query.get_mut(entity) {
                        target.scythed = true;
                        target.hit_from = Some(transform.translation.truncate());
                        status.event = Some(ActorEvent::Hit);
                    }
                    true
                });
        }
        else if actor.attack_input {
            status.attacking = true;
            status.attack_timer = 0.;
            status.event = Some(ActorEvent::Attack);
        }
    }
}

pub fn actor_movement(
    time: Res<Time>,
    mut actor_query: Query<(&Actor, &mut ActorStatus, &mut KinematicCharacterController)>,
    
) {
    for (actor, mut status, mut controller) in &mut actor_query {
        let dir_match = actor.move_input.signum() == status.velocity.x.signum();
        let accel = if dir_match { actor.accel } else { actor.deccel };
        status.velocity.x += actor.move_input * accel * time.delta_seconds();
        
        // Track facing based on input seperately
        if actor.move_input > 0.1 {
            status.facing_left = false;
        }
        else if actor.move_input < -0.1 {
            status.facing_left = true;
        }
        
        if actor.move_input.abs() < 0.1 {
            status.velocity.x *= 1.0 - actor.drag;
        }
        
        status.velocity.x = status.velocity.x.clamp(-actor.move_speed, actor.move_speed);
        
        if (status.velocity.x > 0. && status.right_wall) 
            || (status.velocity.x < 0. && status.left_wall) {
            status.velocity.x = 0.;
        }
        
        if actor.can_jump && actor.jump_input {
            status.velocity.y += actor.jump_speed * time.delta_seconds();
                
            if status.grounded {
                status.event = Some(ActorEvent::Launched);
            }
        }    
        else if !status.grounded {
            status.velocity.y -= if status.velocity.y > 0. { actor.down_gravity } else { actor.up_gravity } * time.delta_seconds();
        }
        
        controller.translation = Some(time.delta_seconds() * status.velocity);
    }
}

fn actor_animations(
    mut actor_query: Query<(&Actor, &ActorStatus, &ActorAnimationStates, &mut SpriteAnimator, &mut TextureAtlasSprite)>,
) {
    for (actor, status, anim_states, mut animator, mut sprite) in &mut actor_query {
        if status.attacking {
            let t = status.attack_timer / actor.attack_time;
            animator.set_row(anim_states.attack_row);
            animator.set_animation_progress(t);
        }
        else if status.grounded {
            if status.velocity.x.abs() > 20. {
                animator.set_row(anim_states.run_row);
            }
            else {
                animator.set_row(anim_states.idle_row);
            }
        }
        else {
            if status.velocity.y > -10. {
                animator.set_row(anim_states.jump_row);
            }
            else {
                animator.set_row(anim_states.fall_row);
            }
        }
        
        sprite.flip_x = status.facing_left;
    }
}

fn actor_audio(
    mut actor_query: Query<(&mut ActorStatus, &ActorAudio)>,
    audio: Res<Audio>
) {
    for (mut status, actor_sounds) in &mut actor_query {
        if let Some(event) = &status.event {
            match event {
                ActorEvent::Launched => audio.play(actor_sounds.jump.clone()),
                ActorEvent::Landed => audio.play(actor_sounds.land.clone()),
                ActorEvent::Attack => audio.play(actor_sounds.attack.clone()),
                ActorEvent::Hit => audio.play(actor_sounds.hit.clone()),
                ActorEvent::Died => audio.play(actor_sounds.death.clone()),
                ActorEvent::Pickup => audio.play(actor_sounds.pickup.clone()),
                ActorEvent::Unlock => audio.play(actor_sounds.unlocked.clone()),
                ActorEvent::Win => audio.play(actor_sounds.victory.clone()),
            };
            
            // Clear event now its been processed
            status.event = None;
        }
    }
}