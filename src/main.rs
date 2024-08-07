use minifb::{Key, Window, WindowOptions};
use std::time::Instant;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;
const GRAVITY: f32 = 0.25;
const BASE_JUMP_POWER: f32 = -7.0;
const MAX_HOLD_TIME: f32 = 1.1;
const MAX_JUMP_MULTIPLIER: f32 = 1.8;
const GROUND_LEVEL: f32 = HEIGHT as f32 - 20.0;
const WHITE: u32 = 0xFFFFFF;
const YELLOW: u32 = 0xFFFF00;

struct Platform {
    x: f32,
    y: f32,
    width: usize,
    height: usize,
    color: u32,
}

struct GameObject {
    x: f32,
    y: f32,
    width: usize,
    height: usize,
    color: u32,
    velocity_y: f32,
    on_ground: bool,
    jump_start_time: Option<Instant>,
}

impl GameObject {
    fn new(x: f32, y: f32, width: usize, height: usize, color: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            color,
            velocity_y: 0.0,
            on_ground: false,
            jump_start_time: None,
        }
    }

    fn update(&mut self, window: &Window, platforms: &[Platform]) {
        if window.is_key_down(Key::Space) && self.on_ground {
            if self.jump_start_time.is_none() {
                self.jump_start_time = Some(Instant::now());
            }

            if let Some(start_time) = self.jump_start_time {
                let elapsed = start_time.elapsed().as_secs_f32();
                if elapsed >= MAX_HOLD_TIME {
                    self.color = YELLOW;
                }
            }
        } else if !window.is_key_down(Key::Space) && self.jump_start_time.is_some() {
            if let Some(start_time) = self.jump_start_time {
                let elapsed = start_time.elapsed().as_secs_f32();
                let hold_time = elapsed.min(MAX_HOLD_TIME);
                let jump_power = BASE_JUMP_POWER
                    * (1.0 + (MAX_JUMP_MULTIPLIER - 1.0) * (hold_time / MAX_HOLD_TIME));
                self.velocity_y = jump_power;
            }
            self.jump_start_time = None;
            self.color = WHITE;
            self.on_ground = false;
        }

        if !self.on_ground {
            self.velocity_y += GRAVITY;
        }

        let mut dx = 0.0;
        let dy = self.velocity_y;

        if window.is_key_down(Key::Left) || window.is_key_down(Key::A) {
            dx -= 2.0;
        }
        if window.is_key_down(Key::Right) || window.is_key_down(Key::D) {
            dx += 2.0;
        }

        self.x += dx;

        if self.x < 0.0 {
            self.x = 0.0;
        } else if self.x + self.width as f32 > WIDTH as f32 {
            self.x = WIDTH as f32 - self.width as f32;
        }

        for platform in platforms {
            if self.x < platform.x + platform.width as f32
                && self.x + self.width as f32 > platform.x
                && self.y < platform.y + platform.height as f32
                && self.y + self.height as f32 > platform.y
            {
                if dx > 0.0 {
                    self.x = platform.x - self.width as f32;
                } else if dx < 0.0 {
                    self.x = platform.x + platform.width as f32;
                }
            }
        }

        self.y += dy;
        self.on_ground = false;

        if self.y < 0.0 {
            self.y = 0.0;
            self.velocity_y = 0.0;
        } else if self.y + self.height as f32 > HEIGHT as f32 {
            self.y = HEIGHT as f32 - self.height as f32;
            self.velocity_y = 0.0;
            self.on_ground = true;
        }

        for platform in platforms {
            if self.x < platform.x + platform.width as f32
                && self.x + self.width as f32 > platform.x
                && self.y < platform.y + platform.height as f32
                && self.y + self.height as f32 > platform.y
            {
                if dy > 0.0 {
                    self.y = platform.y - self.height as f32;
                    self.velocity_y = 0.0;
                    self.on_ground = true;
                } else if dy < 0.0 {
                    self.y = platform.y + platform.height as f32;
                    self.velocity_y = 0.0;
                }
            }
        }
    }

    fn draw(&self, buffer: &mut [u32]) {
        for y in self.y as usize..(self.y as usize + self.height) {
            for x in self.x as usize..(self.x as usize + self.width) {
                if x < WIDTH && y < HEIGHT {
                    buffer[y * WIDTH + x] = self.color;
                }
            }
        }
    }
}

impl Platform {
    fn new(x: f32, y: f32, width: usize, height: usize, color: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            color,
        }
    }

    fn draw(&self, buffer: &mut [u32]) {
        for y in self.y as usize..(self.y as usize + self.height) {
            for x in self.x as usize..(self.x as usize + self.width) {
                if x < WIDTH && y < HEIGHT {
                    buffer[y * WIDTH + x] = self.color;
                }
            }
        }
    }
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new("LeapEngine", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    let mut player = GameObject::new(100.0, 100.0, 50, 50, WHITE);
    let platforms = vec![
        Platform::new(200.0, 600.0, 100, 20, 0x0000FF),
        Platform::new(300.0, 500.0, 100, 20, 0x0000FF),
        Platform::new(500.0, 400.0, 100, 20, 0x0000FF),
        Platform::new(700.0, 300.0, 100, 20, 0x0000FF),
        Platform::new(900.0, 200.0, 100, 20, 0x0000FF),
        Platform::new(1100.0, 150.0, 100, 20, 0x00FF00),
    ];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        player.update(&window, &platforms);

        for i in buffer.iter_mut() {
            *i = 0;
        }

        player.draw(&mut buffer);
        for platform in &platforms {
            platform.draw(&mut buffer);
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
