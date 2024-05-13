// player.rs
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

const PLAYER_SIZE: f32 = 0.5; // Adjust the player size as needed

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub direction: f32,
}

impl Player {
    pub fn new(x: f32, y: f32, direction: f32) -> Self {
        Player { x, y, direction }
    }

    pub fn move_forward(&mut self, distance: f32, check_wall: &dyn Fn(f32, f32) -> bool) {
        self.move_in_direction(distance, check_wall);
    }

    pub fn move_backward(&mut self, distance: f32, check_wall: &dyn Fn(f32, f32) -> bool) {
        self.move_in_direction(-distance, check_wall);
    }

    pub fn move_in_direction(&mut self, distance: f32, check_wall: &dyn Fn(f32, f32) -> bool) {
        // Calculate tentative new positions
        let new_x = self.x + self.direction.cos() * distance;
        let new_y = self.y + self.direction.sin() * distance;

        // Check potential collision if moving to new_x
        if !self.check_collision(new_x, self.y, check_wall) {
            self.x = new_x;
        }

        // Check potential collision if moving to new_y
        if !self.check_collision(self.x, new_y, check_wall) {
            self.y = new_y;
        }
    }

     // Check collision with walls considering the player's size
     fn check_collision(&self, x: f32, y: f32, check_wall: &dyn Fn(f32, f32) -> bool) -> bool {
        let left = x - PLAYER_SIZE / 2.0;
        let right = x + PLAYER_SIZE / 2.0;
        let top = y - PLAYER_SIZE / 2.0;
        let bottom = y + PLAYER_SIZE / 2.0;

        let top_left = check_wall(left, top);
        let top_right = check_wall(right, top);
        let bottom_left = check_wall(left, bottom);
        let bottom_right = check_wall(right, bottom);

        top_left || top_right || bottom_left || bottom_right
    }

    pub fn turn_left(&mut self, angle: f32) {
        self.direction -= angle;
    }

    pub fn turn_right(&mut self, angle: f32) {
        self.direction += angle;
    }

    pub fn handle_keys(&mut self, events: &EventPump, delta_time: f32, check_wall: &dyn Fn(f32, f32) -> bool) {
        let keys = events.keyboard_state().pressed_scancodes().filter_map(Keycode::from_scancode).collect::<Vec<_>>();

        if keys.contains(&Keycode::W) {
            self.move_forward(5.0 * delta_time, check_wall);
        }
        if keys.contains(&Keycode::S) {
            self.move_backward(5.0 * delta_time, check_wall);
        }
        if keys.contains(&Keycode::A) {
            self.turn_left(2.0 * delta_time);
        }
        if keys.contains(&Keycode::D) {
            self.turn_right(2.0 * delta_time);
        }
    }
}