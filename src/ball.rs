pub mod ball {

    use crate::Player;
    use macroquad::prelude::*;
    pub const BALL_SIZE: f32 = 20f32;
    pub const BALL_SPEED: f32 = 250f32;
    #[derive(PartialEq)]
    pub enum BallState {
        AttachedToPlayer,
        Free,
    }

    pub struct Ball {
        pub rect: Rect,
        pub velocity: Vec2,
        pub ball_state: BallState,
    }

    impl Ball {
        pub fn new(pos: Vec2, velocity: Vec2, ball_state: BallState) -> Self {
            Self {
                rect: Rect::new(pos.x, pos.y, BALL_SIZE, BALL_SIZE),
                velocity: velocity,
                ball_state: ball_state,
            }
        }
        pub fn update(&mut self, dt: f32, player: &Player) {
            if self.ball_state == BallState::AttachedToPlayer {
                self.rect.x = player.rect.x + player.rect.w * 0.5;
                self.rect.y = player.rect.y - self.rect.h * 0.5;
            }

            self.rect.x += self.velocity.x * dt * BALL_SPEED;
            self.rect.y += self.velocity.y * dt * BALL_SPEED;
            if self.rect.x < 0f32 {
                self.velocity.x = 1f32;
            }
            if self.rect.x > screen_width() - self.rect.w {
                self.velocity.x = -1f32;
            }
            if self.rect.y < 0f32 {
                self.velocity.y = 1f32;
            }
        }

        pub fn draw(&self) {
            draw_circle(self.rect.x, self.rect.y, self.rect.w * 0.5, BLACK)
        }
    }
}
