use crate::ball::ball::*;
use macroquad::input::*;
use macroquad::prelude::*;

mod ball;

const PLAYER_SIZE: Vec2 = Vec2::from_array([150f32, 20f32]);

const BLOCK_SIZE: Vec2 = Vec2::from_array([75f32, 20f32]);
const BALL_SIZE: f32 = 20f32;
const POWER_UP_SIZE: Vec2 = Vec2::from_array([20f32, 20f32]);
const POWER_UP_SPEED: f32 = 5f32;
pub struct Player {
    rect: Rect,
}

struct Powerup {
    rect: Rect,
    velocity: Vec2,
    powerup_type: PowerupType,
}

pub fn draw_title_text(text: &str, font: Font, offset: f32) {
    let dims = measure_text(text, Some(font), 30u16, 1.0);
    draw_text_ex(
        text,
        screen_width() * 0.5f32 - dims.width * 0.5,
        screen_height() * 0.5f32 - dims.height * 0.5 + offset,
        TextParams {
            font,
            font_size: 30u16,
            color: BLACK,
            ..Default::default()
        },
    );
}

fn game_text(score: &str, font: Font, player_lives: &usize, level: &usize) {
    let level_text = format!("Level: {}", level);
    let level_text_dim = measure_text(&level_text, Some(font), 30u16, 1.0);
    draw_text_ex(
        &level_text,
        level_text_dim.width - 100f32,
        40.0,
        TextParams {
            font,
            font_size: 30u16,
            color: BLACK,
            ..Default::default()
        },
    );
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
    level: &mut usize,
) {
    *level = 0;
    *score = 0;
    *player_lives = 3;
    *player = Player::new();
    blocks.clear();
    balls.clear();
    init_blocks(blocks, level);
    init_balls(balls, &player);
}

fn init_blocks(blocks: &mut Vec<Block>, level: &mut usize) {
    let width = 8;
    let padding = 5f32;
    let total_block_size = BLOCK_SIZE + vec2(padding, padding);
    let board_start = vec2(
        (screen_width() - (BLOCK_SIZE.x * width as f32)) * 0.5,
        100f32,
    );
    // instead of creating an array, filling random, a function that creates the level according to the
    let level_one = vec![
        2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2,
    ];
    let level_two = vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1];
    let level_three = vec![
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ];
    let levels = vec![&level_one, &level_two, &level_three];
    // TODO: This produces an off by 1 error when level is increased past the length of levels
    let level_data = levels[*level];

    for i in 0..level_data.len() {
        let block_x = (i % width) as f32 * total_block_size.x;
        let block_y = (i / width) as f32 * total_block_size.y;

        let block_type = match level_data[i] {
            0 => BlockType::Regular,
            1 => BlockType::SpawnBallOnDeath,
            2 => BlockType::DropLife,
            _ => BlockType::Regular,
        };
        blocks.push(Block::new(board_start + vec2(block_x, block_y), block_type));
    }
}

fn init_balls(balls: &mut Vec<Ball>, player: &Player) {
    balls.push(crate::Ball::new(
        player.rect.point() + vec2(0f32, -BALL_SIZE),
        vec2(0f32, 0f32),
        BallState::AttachedToPlayer,
    ));
}

pub enum GameState {
    Menu,
    Game,
    LevelCompleted,
    GameCompleted,
    Dead,
}
#[derive(PartialEq, Debug)]
enum BlockType {
    Regular,
    SpawnBallOnDeath,
    DropLife,
}
#[derive(PartialEq)]
enum PowerupType {
    NewLife,
}

struct Block {
    rect: Rect,
    lives: i32,
    block_type: BlockType,
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
            BlockType::DropLife => BLACK,
        };
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, color);
    }
}

impl Powerup {
    pub fn new(pos: Vec2, powerup_type: PowerupType, velocity: Vec2) -> Self {
        Self {
            rect: Rect::new(pos.x, pos.y, POWER_UP_SIZE.x, POWER_UP_SIZE.y),
            velocity: velocity,
            powerup_type: powerup_type,
        }
    }
    // TODO: fix as this will be gross

    // add lots of y to Powerup as a temporary fix
    pub fn add_height(&mut self) {
        self.rect.y += 1000f32;
    }
    pub fn update(&mut self, dt: f32, player: &Player) {
        self.rect.y += self.velocity.y * dt * POWER_UP_SPEED;
    }

    pub fn draw(&self) {
        draw_circle(self.rect.x, self.rect.y, self.rect.w * 0.5, BLUE)
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
    let mut powerups = Vec::new();
    let mut level = 0;

    init_blocks(&mut blocks, &mut level);

    balls.push(Ball::new(
        vec2(screen_width() * 0.5f32, screen_height() * 0.5f32),
        vec2(0f32, 0f32),
        BallState::AttachedToPlayer,
    ));

    loop {
        clear_background(WHITE);
        match game_state {
            GameState::Menu => {
                draw_title_text("Breakout! Press SPACE to start", font, 0f32);
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Game;
                }
            }
            GameState::Game => {
                game_text(
                    &format!("Score: {}", score),
                    font,
                    &player_lives,
                    //TODO: Fix: seems gross
                    &(&level + 1),
                );
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
                                //TODO: abstract this into it's own function
                                // if block is special, spawn a ball
                                if block.block_type == BlockType::SpawnBallOnDeath {
                                    spawn_later.push(Ball::new(
                                        block.rect.point(),
                                        vec2(rand::gen_range(1f32, 1f32), -1.0).normalize(),
                                        BallState::Free,
                                    ));
                                }
                                if block.block_type == BlockType::DropLife {
                                    // create a powerup drop
                                    powerups.push(Powerup::new(
                                        vec2(
                                            block.rect.point().x + (block.rect.w * 0.5f32),
                                            block.rect.point().y,
                                        ),
                                        PowerupType::NewLife,
                                        vec2(0f32, 50f32),
                                    ));
                                }
                            }
                        }
                    }
                }

                for ball in spawn_later.into_iter() {
                    balls.push(ball);
                }
                for item in powerups.iter_mut() {
                    Powerup::draw(&item);
                    item.update(get_frame_time(), &player);
                    if resolve_collision(&mut item.rect, &mut item.velocity, &player.rect) {
                        match item.powerup_type {
                            PowerupType::NewLife => {
                                player_lives += 1;
                            }
                        }
                        item.add_height();
                    }
                }
                // removes when they are off screen
                powerups.retain(|powerup| powerup.rect.y < screen_height());
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

                blocks.retain(|block| block.lives > 0);
                if blocks.is_empty() {
                    level += 1;
                    // TODO: fix magic number
                    if level > 3 {
                        game_state = GameState::GameCompleted;
                    }
                    init_blocks(&mut blocks, &mut level);
                    // reset balls
                    balls = Vec::new();
                    init_balls(&mut balls, &player);
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
                            ball.ball_state = BallState::Free;
                            ball.velocity = vec2(rand::gen_range(1f32, 1f32), -1.0).normalize();
                        }
                    }
                }
            }
            GameState::GameCompleted => {
                draw_title_text(&format!("You Win! Score: {}", score), font, 0f32);
            }
            GameState::LevelCompleted => {
                draw_title_text(&format!("You Win! Score: {}", score), font, 0f32);
            }
            GameState::Dead => {
                draw_title_text(&format!("You Lose! Score: {}", score), font, 0f32);
                draw_title_text("Press SPACE to start again", font, 50f32);
                if is_key_pressed(KeyCode::Space) {
                    reset_game(
                        &mut score,
                        &mut player_lives,
                        &mut player,
                        &mut blocks,
                        &mut balls,
                        &mut level,
                    );
                    game_state = GameState::Game;
                }
            }
        }

        next_frame().await
    }
}
