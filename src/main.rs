// main.rs
mod renderer;
mod player;
mod minimap;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use std::thread;
use player::Player;
use renderer::{render_frame_sdl, is_wall_at, SCREEN_WIDTH, SCREEN_HEIGHT};

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

	let window = video_subsystem
		.window("Raycasting renderer", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
		.position_centered()
		.build()
		.map_err(|e| e.to_string())?;

    let mut canvas = window
		.into_canvas()
		.accelerated() // Этот флаг включает аппаратное ускорение
		.build()
		.map_err(|e| e.to_string())?;
    let mut events = sdl_context.event_pump()?;

    let mut player = Player::new(15.0, 15.0, -1.5708);
    let mut last_frame = sdl_context.timer()?.ticks();

    'running: loop {
        let current_frame = sdl_context.timer()?.ticks();
        let delta_time = (current_frame - last_frame) as f32 / 1000.0;
        last_frame = current_frame;

        // Event handling
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }

        // Check wall is a closure that captures the is_wall_at() from the renderer.rs module
        let check_wall = |x, y| is_wall_at(x, y);

        // Handle keys takes the new check_wall closure as the third argument
        player.handle_keys(&events, delta_time, &check_wall);

        // Update screen
        render_frame_sdl(&mut canvas, &player)?;

        // Frame rate control
        thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}