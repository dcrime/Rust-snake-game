use std::collections::VecDeque;

use minifb::{Key, KeyRepeat, WindowOptions};
use rand::random;

const WIDTH: usize = 200;
const HEIGHT: usize = 200;
const SCALE: usize = 4;
const FPS: usize = 30;

#[derive(Copy, Clone)]
struct Pos {
    x: i32,
    y: i32,
}

impl Default for Pos {
    fn default() -> Self {
        Pos::new((random::<f64>() * (WIDTH as f64)) as i32, (random::<f64>() * (HEIGHT as f64)) as i32)
    }
}

impl Pos {
    fn new(x: i32, y: i32) -> Self { Self { x, y } }
}

struct Direction {
    up: u32,
    down: u32,
    left: u32,
    right: u32,
    current: u32,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::new(random::<u32>() % 3)
    }
}

impl Direction {
    fn new(
        current: u32,
    ) -> Self {
        Self { up: 0, down: 2, left: 1, right: 3, current }
    }
}

struct Snake {
    pos: Pos,
    dir: Direction,
}

impl Default for Snake {
    fn default() -> Self {
        Snake::new(Pos::default())
    }
}

impl Snake {
    fn new(pos: Pos) -> Self { Self { pos, dir: Direction::default() } }
    fn wall(&mut self) {
        if self.pos.x < 0 { self.pos.x = WIDTH as i32 }
        if self.pos.x > WIDTH as i32 { self.pos.x = 0 }
        if self.pos.y < 0 { self.pos.y = HEIGHT as i32 - 1 }
        if self.pos.y > HEIGHT as i32 - 1 { self.pos.y = 0 }
    }
    fn r#move(&mut self) -> Pos {
        if self.dir.current == self.dir.up { self.pos.y -= 1; };
        if self.dir.current == self.dir.down { self.pos.y += 1; };
        if self.dir.current == self.dir.left { self.pos.x -= 1; };
        if self.dir.current == self.dir.right { self.pos.x += 1; };
        self.wall();
        return self.pos;
    }
    fn change_dir(&mut self, dir: u32) {
        if (self.dir.current + 2) % 4 != dir {
            self.dir.current = dir;
        }
    }
    fn collides(&mut self, pos: Pos) -> bool {
        return self.pos.x == pos.x && self.pos.y == pos.y;
    }
}

struct Apple {
    pos: Pos,
}

impl Default for Apple {
    fn default() -> Self {
        let apple = Apple::new(Pos::default());
        apple
    }
}

impl Apple {
    fn new(pos: Pos) -> Self { Self { pos } }
    fn respawn(&mut self) {
        self.pos = Pos::default()
    }
}

struct Tail {
    pos: Pos,
}

impl Tail {
    fn new(pos: Pos) -> Self { Self { pos } }
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut snake = Snake::default();
    let mut apple = Apple::default();
    let mut tail: VecDeque<Tail> = VecDeque::with_capacity(0);

    let dir = Direction::default();
    let mut pos_buffer: Vec<Pos> = Vec::with_capacity(0);

    fn draw(x: usize, y: usize, buffer: &mut Vec<u32>, color: u32) -> i32 {
        if x > WIDTH || y > HEIGHT { return 2; };
        let pos = (WIDTH * y) + x;
        let i = buffer.get_mut(pos);
        match i {
            None => {}
            Some(pixel) => { *pixel = color; }
        }
        return 1;
    }

    fn reset(snake: &mut Snake, apple: &mut Apple, tail: &mut VecDeque<Tail>, buffer: &mut Vec<u32>) {
        snake.pos = Pos::default();
        apple.pos = Pos::default();
        tail.clear();
        buffer.fill(0);
    }

    let mut window = minifb::Window::new(
        "My snek",
        WIDTH * SCALE,
        HEIGHT * SCALE,
        WindowOptions::default(),
    ).unwrap_or_else(|e| { panic!("{}", e); });//.expect("Some error happened while creating window");

    window.limit_update_rate(Some(std::time::Duration::from_millis((60 / FPS) as u64 * 10)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_pressed(Key::W, KeyRepeat::No) { snake.change_dir(dir.up) } else if window.is_key_pressed(Key::A, KeyRepeat::No) { snake.change_dir(dir.left) } else if window.is_key_pressed(Key::S, KeyRepeat::No) { snake.change_dir(dir.down) } else if window.is_key_pressed(Key::D, KeyRepeat::No) { snake.change_dir(dir.right) }

        for pos in pos_buffer.iter_mut() {
            draw(pos.x as usize, pos.y as usize, &mut buffer, 0);
        }
        pos_buffer.clear();

        pos_buffer.push(snake.r#move());
        pos_buffer.push(apple.pos);

        draw(apple.pos.x as usize, apple.pos.y as usize, &mut buffer, 0xFF0000);
        draw(snake.pos.x as usize, snake.pos.y as usize, &mut buffer, 0x00FF00);

        if tail.iter().any(|t| snake.collides(t.pos)) {
            reset(&mut snake, &mut apple, &mut tail, &mut buffer)
        }
        if tail.len() > 0 {
            for t in tail.iter_mut() {
                draw(t.pos.x as usize, t.pos.y as usize, &mut buffer, 0xAAAAAA);
            }
            match tail.pop_back() {
                Some(t) => { pos_buffer.push(t.pos) }
                _ => {}
            };
            tail.push_front(Tail::new(snake.pos));
        }
        if snake.collides(apple.pos) {
            apple.respawn();
            tail.push_back(Tail::new(snake.pos))
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}