use bevy::prelude::*;
use bevy_rapier2d::prelude::KinematicCharacterControllerOutput;
use crate::GameState;
use bevy_rapier2d::prelude::*;

pub struct ActorPlugin;

#[derive(Component, Clone)]
pub struct Actor {
    pub move_speed: f32,
    pub drag: f32,
    pub accel: f32,
    pub deccel: f32,
    pub gravity: f32,
    pub jump_speed: f32,
    pub jump_time: f32,
    pub move_input: f32,
    pub jump_input: bool,
}

#[derive(Component, Default, Clone)]
pub struct ActorStatus {
    pub grounded: bool,
    pub velocity: Vec2,
    pub air_timer: f32,
}

impl Plugin for ActorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(actor_status))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(actor_movement))
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
            gravity: 300.,
            jump_speed: 800.,
            jump_time: 0.2,
            move_input: 0.,
            jump_input: false,
        }
    }
}

fn actor_status(
    time: Res<Time>,
    mut actor_query: Query<(&mut ActorStatus, &KinematicCharacterControllerOutput)>
) {
    for (mut actor_status, controller_output) in &mut actor_query {
        actor_status.grounded = controller_output.grounded;
        actor_status.velocity = controller_output.effective_translation / time.delta_seconds();
        
        if actor_status.grounded {
            actor_status.air_timer = 0.;
        }
        else {
            actor_status.air_timer += time.delta_seconds();
        }
    }
}

fn actor_movement(
    time: Res<Time>,
    mut actor_query: Query<(&Actor, &mut ActorStatus, &mut KinematicCharacterController)>,
) {
    for (actor, mut status, mut controller) in &mut actor_query {
        let dir_match = actor.move_input.signum() == status.velocity.x.signum();
        let accel = if dir_match { actor.accel } else { actor.deccel };
        status.velocity.x += actor.move_input * accel * time.delta_seconds();
        
        if actor.move_input.abs() < 0.1 {
            status.velocity.x *= 1.0 - actor.drag;
        }
        
        status.velocity.x = status.velocity.x.clamp(-actor.move_speed, actor.move_speed);
        
        if actor.jump_input {
            status.velocity.y += actor.jump_speed * time.delta_seconds();
        }    
        else {
            status.velocity.y -= actor.gravity * time.delta_seconds();
        }
        
        controller.translation = Some(time.delta_seconds() * status.velocity);
    }
}