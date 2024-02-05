use crate::GameState;
use bevy::prelude::*;

pub struct SpriteAnimationPlugin;

impl Plugin for SpriteAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
            .with_system(animate_sprite)
            .with_system(destroy_finished_animators)
        );
    }
}

#[derive(Component, Default, Clone)]
pub struct SpriteAnimator {
    start_frame: usize,
    end_frame: usize,
    row_length: usize,
    seconds_per_frame: f32,
    frame_timer: f32,
    pub should_loop: bool,
    playing: bool,
    restart_anim: bool,
    progress_override: Option<f32>,
    cur_frame: usize,
}

#[derive(Component, Default, Clone)]
pub struct DestroyAnimatorOnFinish;

#[derive(Clone, Default, Bundle)]
pub struct EffectBundle {
    #[bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    pub sprite_animator: SpriteAnimator,
    pub destroy: DestroyAnimatorOnFinish,
}

impl SpriteAnimator {
    pub fn new(
        start_frame: usize,
        end_frame: usize,
        row_length: usize,
        seconds_per_frame: f32,
        should_loop: bool,
        start_playing: bool,
    ) -> SpriteAnimator {
        SpriteAnimator {
            start_frame,
            end_frame,
            row_length,
            seconds_per_frame,
            frame_timer: 0.,
            should_loop,
            playing: start_playing,
            restart_anim: false,
            progress_override: None,
            cur_frame: 0,
        }
    }

    pub fn _play(&mut self) {
        self.playing = true;
    }

    pub fn _pause(&mut self) {
        self.playing = false;
    }

    pub fn set_row(&mut self, row_index: usize) {
        let new_start = row_index * self.row_length;
        if self.start_frame != new_start {
            self.start_frame = new_start;
            self.end_frame = self.start_frame + self.row_length - 1;
            self.restart_anim = true;
        }
    }

    pub fn get_row(&self) -> usize {
        self.start_frame / self.row_length
    }

    pub fn get_frame(&self) -> usize {
        self.cur_frame
    }

    pub fn set_frame(&mut self, index: usize) {
        self.progress_override = Some(index as f32 / self.row_length as f32);
    }

    pub fn set_animation_progress(&mut self, t: f32) {
        self.progress_override = Some(t);
    }
  
    pub fn finished(&self) -> bool {
        !self.playing && (self.cur_frame == self.end_frame)
    }

    pub fn get_animation_progress(&self) -> f32 {
        (self.cur_frame as f32 / self.row_length as f32) + (self.frame_timer / self.seconds_per_frame)
    }
}

fn animate_sprite(
    time: Res<Time>,
    mut sprites: Query<(&mut SpriteAnimator, &mut TextureAtlasSprite)>,
) {
    for (mut animator, mut sprite) in sprites.iter_mut() {
        if let Some(t) = animator.progress_override {
            let decimal_frame = (animator.row_length) as f32 * t;
            let frame = decimal_frame as usize;
            sprite.index =
                (animator.start_frame + frame).clamp(animator.start_frame, animator.end_frame);
            animator.frame_timer = (decimal_frame - (frame as f32)) * animator.seconds_per_frame;
            animator.progress_override = None;
        } else if animator.playing {
            animator.frame_timer += time.delta_seconds();

            if animator.restart_anim || animator.frame_timer > animator.seconds_per_frame {
                animator.frame_timer = 0.;

                let mut next_index = sprite.index + 1;

                if animator.restart_anim || next_index > animator.end_frame {
                    next_index = animator.start_frame;
                    animator.restart_anim = false;
                }

                if !animator.should_loop && next_index == animator.end_frame {
                    animator.playing = false;
                    animator.restart_anim = false;
                }

                sprite.index = next_index;
                animator.cur_frame = next_index - animator.start_frame;
            }
        }
    }
}

fn destroy_finished_animators(
    sprites: Query<(Entity, &SpriteAnimator), With<DestroyAnimatorOnFinish>>,
    mut commands: Commands,
) {
    for (entity, animator) in sprites.iter() {
        if animator.finished() {
            commands.entity(entity).despawn();
        }
    }
}
