use macroquad::prelude::*;
use crate::{constants::*, audio::Audio, textures::Textures};

pub struct Player {
    pos: Vec2,
    speed: f32,
    is_alive: bool,
}

impl Player {
    pub fn new() -> Self {
        Self {
            pos: Vec2::new(screen_width() * 0.20, screen_height() * 0.4),
            speed: 0.0,
            is_alive: true,
        }
    }

    pub fn rotation(&self) -> f32 {
        let speed = (450.0, -350.0);
        let angle = (45., -45.0);
        angle.0 + (self.speed - speed.0) * (angle.1 - angle.0) / (speed.1 - speed.0)
    }

    pub fn draw(&self, textures: &Textures) {

        let rotation = self.rotation();

        draw_texture_ex(textures.player(), self.pos.x, self.pos.y, WHITE, DrawTextureParams {
            dest_size: Some(PLAYER_SIZE),
            rotation: rotation.to_radians(),
            ..Default::default()
        });
    }

    pub fn update(&mut self, dt: f32, audio: &Audio) {
        if !self.is_alive {
            return;
        }

        if is_key_pressed(KeyCode::Space) {
            self.speed -= 750.0;
            audio.wing()
        }

        self.speed += 7.5;
        
        self.speed = self.speed.min(450.0);
        self.speed = self.speed.max(-350.0);

        self.pos.y += self.speed * dt;
    }

    pub fn check_collisions(&mut self, pipes: &[Pipe]) -> bool {
        if self.pos.y < 0.0 || self.pos.y > screen_height() - 50.0 {
            self.is_alive = false;
            return true
        }

        let own_rect = Rect::new(
            self.pos.x,
            self.pos.y,
            PLAYER_SIZE.x, 
            PLAYER_SIZE.y
        );

        for pipe in pipes.iter() {
            let rect1 = Rect::new(
                pipe.pos.x, 
                0.0, 
                PIPE_WIDTH, 
                pipe.height-(PIPE_VERT_GAP*0.5),
            );
    
            let rect2 = Rect::new(
                pipe.pos.x, 
                pipe.height+(PIPE_VERT_GAP*0.5), 
                PIPE_WIDTH, 
                screen_height(), 
            );

            if own_rect.intersect(rect1).is_some() || own_rect.intersect(rect2).is_some() {
                self.is_alive = false;
                return true;
            }
        }

        false
    }

    pub fn check_score(&self, pipes: &mut [Pipe], score: &mut usize, audio: &Audio) {
        for pipe in pipes {
            if pipe.pos.x < self.pos.x && !pipe.scored {
                *score += 1;
                audio.score();
                pipe.set_scored()
            }
        }
    }

    pub fn reset(&mut self) {
        self.pos = Vec2::new(screen_width() * 0.20, screen_height() * 0.25);
        self.speed = 0.0;
        self.is_alive = true;
    }

    pub fn is_alive(&self) -> bool {
        self.is_alive
    }

    pub fn speed(&self) -> f32 {
        self.speed
    }

    pub fn pos(&self) -> Vec2 {
        self.pos
    }
}

#[derive(Default, Clone, Copy)]
pub struct Pipe {
    pos: Vec2,
    height: f32,
    scored: bool
}

impl Pipe {
    pub fn draw(&self, textures: &Textures) {
        // Top half
        draw_texture_ex(textures.pipe(), self.pos.x, -(PIPE_WIDTH*6.15)+self.height-(PIPE_VERT_GAP*0.5), WHITE, DrawTextureParams {
            dest_size: Some(Vec2::new(PIPE_WIDTH, PIPE_WIDTH*6.15)),
            flip_y: true,
            ..Default::default()
        });


        // Bottom half
        draw_texture_ex(textures.pipe(), self.pos.x, self.height+(PIPE_VERT_GAP*0.5), WHITE, DrawTextureParams {
            dest_size: Some(Vec2::new(PIPE_WIDTH, PIPE_WIDTH*6.15)),
            flip_y: false,
            ..Default::default()
        });
    }

    pub fn update(&mut self, dt: f32, last: f32) {

        if self.pos.x < 0.0 - PIPE_WIDTH {
            self.pos.x = last + PIPE_HORI_GAP;
            self.scored = false;
            self.height = rand::gen_range(PIPE_VERT_GAP, screen_height()-(PIPE_VERT_GAP));
        }

        self.pos.x -= 150.0 * dt;
    }

    pub fn pos(&self) -> Vec2 {
        self.pos
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn set_scored(&mut self) {
        self.scored = true;
    }
}

pub fn setup_pipes(pipes: &mut [Pipe]) {
    (0..pipes.len()).for_each(|i| {
        pipes[i].pos.x = (PIPE_HORI_GAP * (i as f32)) + screen_width() * 0.7;
        pipes[i].height = rand::gen_range(PIPE_VERT_GAP, screen_height()-(PIPE_VERT_GAP));
        pipes[i].scored = false;
    });
}