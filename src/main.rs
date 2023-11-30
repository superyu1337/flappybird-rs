mod objects;
mod audio;
mod textures;
mod constants;
mod enums;
mod debug;

use audio::Audio;
use textures::Textures;
use objects::*;
use debug::*;
use enums::*;

use macroquad::prelude::*;

fn draw_background(texture: &Texture2D) {
    let width = screen_height()*0.5625;
    let iterations = (screen_width()/width) as i32 + 1;

    for i in 0..iterations {
        draw_texture_ex(texture, width*i as f32, 0.0, WHITE, DrawTextureParams {
            dest_size: Some(Vec2::new(screen_height()*0.5625, screen_height())),
            ..Default::default()
        });
    }
}

fn draw_score(score: &usize, font: &Font) {
    let text = format!("{}", score);
    let center = get_text_center(text.as_str(), Some(font), 32, 1.0, 0.0);
    draw_text_ex(
        text.as_str(),
        (screen_width()*0.5) - center.x,
        screen_height()*0.25 - center.y,
        TextParams { font: Some(font), font_size: 32, font_scale: 1.0, font_scale_aspect: 1.0, rotation: 0.0, color: WHITE }
    )
}

fn draw_menu(font: &Font) {
    let text = String::from("Press Space to start!");
    let center = get_text_center(text.as_str(), Some(font), 32, 1.0, 0.0);
    draw_text_ex(
        text.as_str(),
        (screen_width()*0.5) - center.x,
        screen_height()*0.5 - center.y,
        TextParams { font: Some(font), font_size: 32, font_scale: 1.0, font_scale_aspect: 1.0, rotation: 0.0, color: WHITE }
    )
}

fn draw_dead(score: &usize, font: &Font) {
    let text = String::from("You died!");
    let center = get_text_center(text.as_str(), Some(font), 32, 1.0, 0.0);
    draw_text_ex(
        text.as_str(),
        (screen_width()*0.5) - center.x,
        screen_height()*0.3 - center.y,
        TextParams { font: Some(font), font_size: 32, font_scale: 1.0, font_scale_aspect: 1.0, rotation: 0.0, color: WHITE }
    );

    let text = format!("Final score: {}", score);
    let center = get_text_center(text.as_str(), Some(font), 32, 1.0, 0.0);
    draw_text_ex(
        text.as_str(),
        (screen_width()*0.5) - center.x,
        screen_height()*0.3+32.0 - center.y,
        TextParams { font: Some(font), font_size: 32, font_scale: 1.0, font_scale_aspect: 1.0, rotation: 0.0, color: WHITE }
    );

    let text = String::from("Press Space to try again!");
    let center = get_text_center(text.as_str(), Some(font), 32, 1.0, 0.0);
    draw_text_ex(
        text.as_str(),
        (screen_width()*0.5) - center.x,
        screen_height()*0.6 - center.y,
        TextParams { font: Some(font), font_size: 32, font_scale: 1.0, font_scale_aspect: 1.0, rotation: 0.0, color: WHITE }
    )
}

#[macroquad::main(window_config)]
async fn main() {
    set_pc_assets_folder("assets");
    let audio = Audio::new().await;
    let textures = Textures::new().await;
    let font = load_ttf_font("font.ttf").await.unwrap();

    let mut score = 0;
    let mut gamestate = GameState::Menu;
    let mut player = Player::new();
    let mut pipes = [Pipe::default(); 6];

    setup_pipes(&mut pipes);

    loop {
        clear_background(BLACK);
        draw_background(textures.background_day());

        match gamestate {
            GameState::Menu => {
                player.draw(&textures);
                pipes.iter().for_each(|pipe| pipe.draw(&textures));
                draw_menu(&font);

                if is_key_pressed(KeyCode::Space) {
                    gamestate = GameState::Playing;
                }
            },
            GameState::Playing => {
                player.update(get_frame_time(), &audio);

                if player.check_collisions(&pipes) {
                    audio.hit();
                    audio.die();
                    gamestate = GameState::Dead;
                }
                player.draw(&textures);

                let last_pipe_x = pipes
                    .iter()
                    .max_by(|a, b| {
                        a.pos().x.partial_cmp(&b.pos().x).unwrap()
                    })
                    .unwrap()
                    .pos().x;

                pipes.iter_mut().for_each(|pipe| {
                    if player.is_alive() {
                        pipe.update(get_frame_time(), last_pipe_x);
                    }
                    pipe.draw(&textures);
                });

                player.check_score(&mut pipes, &mut score, &audio);
                draw_score(&score, &font);
            },
            GameState::Dead => {
                player.draw(&textures);
                pipes.iter().for_each(|pipe| pipe.draw(&textures));
                draw_dead(&score, &font);

                if is_key_pressed(KeyCode::Space) {
                    setup_pipes(&mut pipes);
                    player.reset();
                    score = 0;
                    gamestate = GameState::Playing;
                }
            },
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        draw_debug_options(true, false, false, false, &player, &pipes);
        next_frame().await
    }
}

fn window_config() -> Conf {
    Conf {
        window_title: "flappybird-rs".to_string(),
        window_width: 800,
        window_height: 600,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}