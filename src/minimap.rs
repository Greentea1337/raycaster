// minimap.rs
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::player::Player;
use crate::renderer::{MAP_WIDTH, MAP_HEIGHT};

const WALL_COLOR: Color = Color::RGB(255, 255, 255); // Белый цвет для стен
const EMPTY_COLOR: Color = Color::RGB(0, 0, 0); // Черный цвет для пустот
const PLAYER_COLOR: Color = Color::RGB(255, 0, 0); // Красный цвет для игрока
const MAP_SCALE: usize = 4; // Масштаб карты

fn draw_block(canvas: &mut Canvas<Window>, x: i32, y: i32, color: Color) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.fill_rect(Rect::new(x, y, MAP_SCALE as u32, MAP_SCALE as u32))
}

pub fn draw_mini_map(canvas: &mut Canvas<Window>, player: &Player, world_map: &[[u8; MAP_WIDTH]; MAP_HEIGHT]) -> Result<(), String> {
    let mini_map_width = (MAP_WIDTH * MAP_SCALE) as u32;
    let mini_map_height = (MAP_HEIGHT * MAP_SCALE) as u32;

    // Рисуем фон для мини-карты
    canvas.set_draw_color(EMPTY_COLOR);
    canvas.fill_rect(Rect::new(0, 0, mini_map_width, mini_map_height))?;

    // Рисуем блоки карты
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let wall = world_map[y][x];
            let block_color = if wall == 1 { WALL_COLOR } else { EMPTY_COLOR };
            draw_block(canvas, (x * MAP_SCALE) as i32, (y * MAP_SCALE) as i32, block_color)?;
        }
    }

    // Рисуем игрока на мини-карте
    draw_block(
        canvas,
        (player.x as i32 * MAP_SCALE as i32) - (MAP_SCALE as i32 / 2),
        (player.y as i32 * MAP_SCALE as i32) - (MAP_SCALE as i32 / 2),
        PLAYER_COLOR,
    )?;

    Ok(())
}