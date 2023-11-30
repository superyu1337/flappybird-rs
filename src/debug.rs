use macroquad::prelude::*;
use crate::objects::*;

fn draw_debug(string: String, dbg_count: &mut i32) {
    draw_text(string.as_str(), 10.0, 16.0 * (*dbg_count as f32), 16.0, WHITE);
    *dbg_count += 1;
}

pub fn draw_debug_options(draw_fps: bool, draw_window: bool, draw_player: bool, draw_pipes: bool, player: &Player, pipes: &[Pipe]) {
    let mut dbg_count = 1;
    
    if draw_fps {
        draw_debug(format!("fps: {}", get_fps()), &mut dbg_count);
    }

    if draw_window {
        draw_debug(format!("window width: {}", screen_width()), &mut dbg_count);
        draw_debug(format!("window height: {}", screen_height()), &mut dbg_count);
        dbg_count += 1;
    }

    if draw_player {
        draw_debug(format!("speed: {}", player.speed()), &mut dbg_count);
        draw_debug(format!("pos_y: {:.02}", player.pos().y), &mut dbg_count);
        draw_debug(format!("rotation: {:.02}° / {:.02} rad", player.rotation(), player.rotation().to_radians()), &mut dbg_count);
        draw_debug(format!("alive: {}", player.is_alive()), &mut dbg_count);
        dbg_count += 1;
    }

    if draw_pipes {
        (0..pipes.len()).for_each(|i| {
            draw_debug(format!("pipe_{}_height: {:.02}", i+1, pipes[i].height()), &mut dbg_count);
        });

    }
}
