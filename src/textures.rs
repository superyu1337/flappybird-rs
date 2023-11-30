use macroquad::texture::{Texture2D, load_texture};

pub struct Textures {
    player: Texture2D,
    pipe: Texture2D,
    background_day: Texture2D
}

impl Textures {
    pub async fn new() -> Self {
        Self {
            player: load_texture("sprites/redbird-downflap.png").await.unwrap(),
            pipe: load_texture("sprites/pipe-green.png").await.unwrap(),
            background_day: load_texture("sprites/background-day.png").await.unwrap(),
        }
    }

    pub fn player(&self) -> &Texture2D {
        &self.player
    } 

    pub fn pipe(&self) -> &Texture2D {
        &self.pipe
    }

    pub fn background_day(&self) -> &Texture2D {
        &self.background_day
    } 
}