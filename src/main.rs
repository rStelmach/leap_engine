use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

struct GameObject {
    x: f32,
    y: f32,
    width: usize,
    height: usize,
    color: u32,
}

impl GameObject {
    fn new(x: f32, y: f32, width: usize, height: usize, color: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            color,
        }
    }

    fn update(&mut self, window: &Window) {
        if window.is_key_down(Key::Left) {
            self.x -= 2.0;
        }
        if window.is_key_down(Key::Right) {
            self.x += 2.0;
        }
        if window.is_key_down(Key::Up) {
            self.y -= 2.0;
        }
        if window.is_key_down(Key::Down) {
            self.y += 2.0;
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

    while window.is_open() && !window.is_key_down(Key::Escape) {
        player.update(&window);

        for i in buffer.iter_mut() {
            *i = 0;
        }

        // Draw game objects
        player.draw(&mut buffer);

        // Draw the frame
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
