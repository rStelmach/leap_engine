use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;
const GRAVITY: f32 = 0.2;
const JUMP_POWER: f32 = -10.0;
const GROUND_LEVEL: f32 = 600.0;

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
        }
    }

    fn update(&mut self, window: &Window, platforms: &[Platform]) {
        if !self.on_ground {
            self.velocity_y += GRAVITY;
        }

        if window.is_key_down(Key::Left) || window.is_key_down(Key::A) {
            self.x -= 2.0;
        }
        if window.is_key_down(Key::Right) || window.is_key_down(Key::D) {
            self.x += 2.0;
        }

        if (window.is_key_down(Key::Space) || window.is_key_down(Key::W)) && self.on_ground {
            self.velocity_y = JUMP_POWER;
            self.on_ground = false;
        }

        self.y += self.velocity_y;
        self.on_ground = false;

        if self.y + self.height as f32 >= GROUND_LEVEL {
            self.y = GROUND_LEVEL - self.height as f32;
            self.velocity_y = 0.0;
            self.on_ground = true;
        }

        for platform in platforms {
            if self.x < platform.x + platform.width as f32
                && self.x + self.width as f32 > platform.x
                && self.y + self.height as f32 > platform.y
                && self.y + self.height as f32 <= platform.y + platform.height as f32
                && self.velocity_y > 0.0
            {
                self.y = platform.y - self.height as f32;
                self.velocity_y = 0.0;
                self.on_ground = true;
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

    let mut player = GameObject::new(100.0, 100.0, 50, 50, 0xFFFFFF);
    let platforms = vec![
        Platform::new(300.0, 400.0, 100, 20, 0x00FF00),
        Platform::new(500.0, 300.0, 100, 20, 0x00FF00),
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
