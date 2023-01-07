use bevy::prelude::*;

pub struct SpriteAnimationPlugin;

impl Plugin for SpriteAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(animate_sprite);
    }
}

#[derive(Component)]
pub struct SpriteAnimator {
    _atlas_handle: Handle<TextureAtlas>,
    start_frame: usize,
    end_frame: usize,
    _row_length: usize,
    seconds_per_frame: f32,
    frame_timer: f32,
    pub should_loop: bool,
    playing: bool,
    restart_anim: bool,
}

impl SpriteAnimator {
    pub fn new(
        _atlas_handle: Handle<TextureAtlas>,
        start_frame: usize,
        end_frame: usize,
        row_length: usize,
        seconds_per_frame: f32,
        should_loop: bool,
    ) -> SpriteAnimator {
        SpriteAnimator {
            _atlas_handle,
            start_frame,
            end_frame,
            _row_length: row_length,
            seconds_per_frame,
            frame_timer: 0.,
            should_loop,
            playing: true,
            restart_anim: false,
        }
    }

    pub fn _play(&mut self) {
        self.playing = true;
    }

    pub fn _pause(&mut self) {
        self.playing = false;
    }

    pub fn _set_row(&mut self, row_index: usize) {
        self.start_frame = row_index * self._row_length;
        self.end_frame = self.start_frame + self._row_length - 1;
        self.restart_anim = true;
    }
}

fn animate_sprite(
    time: Res<Time>,
    mut sprites: Query<(
        &mut SpriteAnimator, 
        &mut TextureAtlasSprite,
    )>
) {
    for (mut animator, mut sprite) in sprites.iter_mut() {
        if animator.playing {
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
            }
        }
        
    }
}