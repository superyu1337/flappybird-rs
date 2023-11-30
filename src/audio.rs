use macroquad::audio::{Sound, load_sound, play_sound};

pub struct Audio {
    sfx_die: Sound,
    sfx_hit: Sound,
    sfx_score: Sound,
    sfx_wing: Sound,
}

impl Audio {
    pub async fn new() -> Self {
        Self {
            sfx_die: load_sound("sounds/die.wav").await.unwrap(),
            sfx_hit: load_sound("sounds/hit.wav").await.unwrap(),
            sfx_score: load_sound("sounds/score.wav").await.unwrap(),
            sfx_wing: load_sound("sounds/wing.wav").await.unwrap(),
            
        }
    }

    pub fn die(&self) {
        play_sound(&self.sfx_die, macroquad::audio::PlaySoundParams { looped: false, volume: 0.5 })
    }

    pub fn hit(&self) {
        play_sound(&self.sfx_hit, macroquad::audio::PlaySoundParams { looped: false, volume: 0.1 })
    }

    pub fn score(&self) {
        play_sound(&self.sfx_score, macroquad::audio::PlaySoundParams { looped: false, volume: 0.5 })
    }

    pub fn wing(&self) {
        play_sound(&self.sfx_wing, macroquad::audio::PlaySoundParams { looped: false, volume: 0.15 })
    }
}