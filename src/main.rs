use macroquad::input::*;
use macroquad::prelude::*;

const PLAYER_SIZE: Vec2 = Vec2::from_array([150f32, 20f32]);

const BLOCK_SIZE: Vec2 = Vec2::from_array([100f32, 40f32]);
const BALL_SIZE: f32 = 20f32;
const BALL_SPEED: f32 = 250f32;
struct Player {
    rect: Rect,
}

pub fn draw_title_text(text: &str, font: Font) {
    let dims = measure_text(text, Some(font), 30u16, 1.0);
    draw_text_ex(
        text,
        screen_width() * 0.5f32 - dims.width * 0.5,
        screen_height() * 0.5f32 - dims.height * 0.5,
        TextParams {
            font,
            font_size: 30u16,
            color: BLACK,
            ..Default::default()
        },
    );
}

fn game_text(score: &str, font: Font, player_lives: &usize) {
    let score_text = format!("Score: {}", score);
    let score_text_dim = measure_text(&score_text, Some(font), 30u16, 1.0);
    draw_text_ex(
        &score_text,
        screen_width() * 0.5f32 - score_text_dim.width * 0.5,
        40.0,
        TextParams {
            font,
            font_size: 30u16,
            color: BLACK,
            ..Default::default()
        },
    );
    let lives_text = format!("Lives: {}", player_lives);
    let lives_text_dim = measure_text(&lives_text, Some(font), 30u16, 1.0);
    draw_text_ex(
        &lives_text,
        screen_width() - lives_text_dim.width - 20.0,
        40.0,
        TextParams {
            font,
            font_size: 30u16,
            color: BLACK,
            ..Default::default()
        },
    );
}

fn reset_game(
    score: &mut i32,
    player_lives: &mut usize,
    player: &mut Player,
    blocks: &mut Vec<Block>,
    balls: &mut Vec<Ball>,
) {
    *score = 0;
    *player_lives = 3;
    *player = Player::new();
    blocks.clear();
    balls.clear();
    init_blocks(blocks);
    init_balls(balls, &player);
}

fn init_blocks(blocks: &mut Vec<Block>) {
    let (width, height) = (6, 6);
    let padding = 5f32;
    let total_block_size = BLOCK_SIZE + vec2(padding, padding);
    let board_start = vec2(
        (screen_width() - (BLOCK_SIZE.x * width as f32)) * 0.5,
        50f32,
    );
    for i in 0..width * height {
        let block_x = (i % width) as f32 * total_block_size.x;
        let block_y = (i / width) as f32 * total_block_size.y;
        let rand_index = rand::gen_range(0, blocks.len());
        let block_type = match rand_index {
            0 => BlockType::Regular,
            1 => BlockType::SpawnBallOnDeath,
            _ => BlockType::Regular,
        };
        blocks.push(Block::new(board_start + vec2(block_x, block_y), block_type));
    }
}

fn init_balls(balls: &mut Vec<Ball>, player: &Player) {
    balls.push(Ball::new(
        player.rect.point() + vec2(0f32, -BALL_SIZE),
        vec2(0f32, 0f32),
        BallState::AttachedToPlayer,
    ));
}

pub enum GameState {
    Menu,
    Game,
    LevelCompleted,
    Dead,
}
#[derive(PartialEq)]
enum BlockType {
    Regular,
    SpawnBallOnDeath,
}
#[derive(PartialEq)]
enum BallState {
    AttachedToPlayer,
    Free,
}

struct Block {
    rect: Rect,
    lives: i32,
    block_type: BlockType,
}

struct Ball {
    rect: Rect,
    velocity: Vec2,
    ball_state: BallState,
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
        if (self.ball_state == BallState::AttachedToPlayer) {
            self.rect.x = player.rect.x + player.rect.w * 0.5 - self.rect.w * 0.5;
            self.rect.y = player.rect.y - self.rect.h;
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
        draw_rectangle(
            self.rect.x,
            self.rect.y,
            self.rect.w,
            self.rect.h,
            DARKBROWN,
        );
    }
}

impl Block {
    pub fn new(pos: Vec2, block_type: BlockType) -> Self {
        Self {
            rect: Rect::new(pos.x, pos.y, BLOCK_SIZE.x, BLOCK_SIZE.y),
            lives: 2,
            block_type,
        }
    }
    pub fn draw(&self) {
        let color = match self.block_type {
            BlockType::Regular => match self.lives {
                1 => RED,
                2 => ORANGE,
                _ => DARKBLUE,
            },
            BlockType::SpawnBallOnDeath => YELLOW,
        };
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, color);
    }
}

impl Player {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(
                screen_width() * 0.5f32 - PLAYER_SIZE.x * 0.5f32,
                screen_height() - 100f32,
                PLAYER_SIZE.x,
                PLAYER_SIZE.y,
            ),
        }
    }
    pub fn update(&mut self, _dt: f32) {
        self.rect.x = mouse_position().0;

        if self.rect.x < 0f32 {
            self.rect.x = 0f32
        }

        if self.rect.x > screen_width() - self.rect.w {
            self.rect.x = screen_width() - self.rect.w
        }
    }
    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, BLUE);
    }
}

fn resolve_collision(a: &mut Rect, vel: &mut Vec2, b: &Rect) -> bool {
    // early exit
    let intersection = match a.intersect(*b) {
        Some(intersection) => intersection,
        None => return false,
    };
    let a_center = a.point() + a.size() * 0.5f32;
    let b_center = b.point() + b.size() * 0.5f32;
    let to = b_center - a_center;
    let to_signum = to.signum();
    match intersection.w > intersection.h {
        true => {
            // bounce on y
            a.y -= to_signum.y * intersection.h;
            vel.y = -to_signum.y * vel.y.abs();
        }
        false => {
            // bounce on x
            a.x -= to_signum.x * intersection.w;
            vel.x = -to_signum.x * vel.x.abs();
        }
    }
    true
}
#[macroquad::main("breakout")]
async fn main() {
    let font = load_ttf_font("res/BrunoAceSC-Regular.ttf").await.unwrap();
    let mut game_state = GameState::Menu;
    let mut score = 0;
    let mut player_lives = 3;
    let mut player = Player::new();
    let mut blocks = Vec::new();
    let mut balls = Vec::new();
    let mut ball_state = BallState::AttachedToPlayer;
    init_blocks(&mut blocks);

    balls.push(Ball::new(
        vec2(screen_width() * 0.5f32, screen_height() * 0.5f32),
        vec2(0f32, 0f32),
        BallState::AttachedToPlayer,
    ));

    loop {
        clear_background(WHITE);

        match game_state {
            GameState::Menu => {
                draw_title_text("Breakout! Press SPACE to start", font);
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Game;
                }
            }
            GameState::Game => {
                game_text(&format!("Score: {}", score), font, &player_lives);
                player.update(get_frame_time());
                for ball in balls.iter_mut() {
                    ball.update(get_frame_time(), &player);
                }
                let mut spawn_later = vec![];
                for ball in balls.iter_mut() {
                    resolve_collision(&mut ball.rect, &mut ball.velocity, &player.rect);
                    // block collision
                    for block in blocks.iter_mut() {
                        if resolve_collision(&mut ball.rect, &mut ball.velocity, &block.rect) {
                            block.lives -= 1;
                            if block.lives == 0 {
                                score += 10;
                                // if block is special, spawn a ball
                                if block.block_type == BlockType::SpawnBallOnDeath {
                                    spawn_later.push(Ball::new(
                                        block.rect.point(),
                                        vec2(rand::gen_range(1f32, 1f32), -1.0).normalize(),
                                        BallState::Free,
                                    ));
                                }
                            }
                        }
                    }
                }
                for ball in spawn_later.into_iter() {
                    balls.push(ball);
                }

                let balls_len = balls.len();
                let was_last_ball = balls_len == 1;
                balls.retain(|ball| ball.rect.y < screen_height());
                let removed_balls = balls_len - balls.len();
                if removed_balls > 0 && was_last_ball {
                    player_lives -= removed_balls;
                    balls.push(Ball::new(
                        vec2(player.rect.x + player.rect.w * 0.5f32, player.rect.y - 10.0),
                        vec2(0f32, 0f32),
                        BallState::AttachedToPlayer,
                    ));
                    if player_lives == 0 {
                        game_state = GameState::Dead;
                    }
                }
                // Remove blocks with 0 lives
                blocks.retain(|block| block.lives > 0);
                if blocks.is_empty() {
                    game_state = GameState::LevelCompleted;
                }
                // update balls
                for ball in balls.iter_mut() {
                    ball.update(get_frame_time(), &player);
                    ball.draw();
                }
                // draw player
                player.draw();
                // draw blocks
                for block in blocks.iter() {
                    block.draw();
                }
                // if mouse is clicked make ball move up
                for ball in balls.iter_mut() {
                    if ball.ball_state == BallState::AttachedToPlayer {
                        if is_mouse_button_pressed(MouseButton::Left) {
                            ball_state = BallState::Free;
                            ball.ball_state = BallState::Free;
                            ball.velocity = vec2(rand::gen_range(1f32, 1f32), -1.0).normalize();
                        }
                    }
                }
            }
            GameState::LevelCompleted => {
                draw_title_text(&format!("You Win! Score: {}", score), font);
            }
            GameState::Dead => {
                draw_title_text(
                    &format!("You Lose! Score: {}, Press SPACE to start again", score),
                    font,
                );
                if is_key_pressed(KeyCode::Space) {
                    reset_game(
                        &mut score,
                        &mut player_lives,
                        &mut player,
                        &mut blocks,
                        &mut balls,
                    );
                    game_state = GameState::Game;
                }
            }
        }

        next_frame().await
    }
}
