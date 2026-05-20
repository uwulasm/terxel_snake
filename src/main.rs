use std::{io::Write, time::Duration};

use crossterm::{cursor::{DisableBlinking, EnableBlinking, Hide, Show}, event::{self, Event, KeyCode, KeyEvent}, execute, terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode}};
use terxel::{Canvas, Color};

#[derive(PartialEq, Eq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

struct Snake {
    x: f32,
    y: f32,
    heading: Direction,
    body: Vec<(f32, f32)>,
}

struct Food {
    x: f32,
    y: f32,
}

impl Food {
    fn new(width: usize, height: usize) -> Food {
        Food {
            x: rand::random_range(0..width) as f32,
            y: rand::random_range(0..height) as f32,
        }
    }

    fn update(&mut self, snake: &mut Snake, width: usize, height: usize) {
        if self.x == snake.x && self.y == snake.y {
            snake.grow();

            self.x = rand::random_range(0..width) as f32;
            self.y = rand::random_range(0..height) as f32;
        }
    }

    fn draw(&self, canvas: &mut Canvas) {
        canvas.set_pixel(self.x as usize, self.y as usize, Color::rgb(255, 0, 0));
    }
}

impl Snake {
    fn new() -> Snake {
        Snake {
            x: 0.0,
            y: 0.0,
            heading: Direction::Right,
            body: Vec::new(),
        }
    }

    fn grow(&mut self) {
        if let Some(last) = self.body.last() {
            self.body.push(*last);
        } else {
            self.body.push((self.x, self.y));
        }
    }

    fn update(&mut self, key: Option<&KeyEvent>, width: usize, height: usize) {
        if let Some(ke) = key {
            if ke.code == KeyCode::Right && self.heading != Direction::Left {
                self.heading = Direction::Right;
            }
            if ke.code == KeyCode::Up && self.heading != Direction::Down {
                self.heading = Direction::Up;
            }
            if ke.code == KeyCode::Down && self.heading != Direction::Up {
                self.heading = Direction::Down;
            }
            if ke.code == KeyCode::Left && self.heading != Direction::Right {
                self.heading = Direction::Left;
            }
        }

        if !self.body.is_empty() {
            if self.body.len() > 1 {
                for i in (1..self.body.len()).rev() {
                    self.body[i] = self.body[i - 1];
                }
            }

            self.body[0] = (self.x, self.y);
        }

        match self.heading {
            Direction::Right => self.x += 1.0,
            Direction::Up => self.y -= 1.0,
            Direction::Down => self.y += 1.0,
            Direction::Left => self.x -= 1.0,
        }

        for (x, y) in &self.body {
            if self.x == *x && self.y == *y {
                self.body.clear();
                break;
            }
        }

        if self.x as usize >= width {
            self.x -= width as f32;
        }
        if self.y as usize >= height {
            self.y -= height as f32;
        }
        if self.x < 0.0 {
            self.x += width as f32;
        }
        if self.y < 0.0 {
            self.y += height as f32;
        }
    }

    fn draw(&self, canvas: &mut Canvas) {
        canvas.set_pixel(self.x as usize, self.y as usize, Color::rgb(0, 255, 255));
        for (x, y) in &self.body {
            canvas.set_pixel(*x as usize, *y as usize, Color::rgb(0, 255, 255));
        }
    }
}

fn main() {
    let mut canvas = Canvas::with_scale(27, 11, 4);
    execute!(std::io::stdout(), EnterAlternateScreen, Hide).unwrap();
    enable_raw_mode().unwrap();

    let mut snake = Snake::new();
    let mut food = Food::new(canvas.width(), canvas.height());
    loop {
        let mut key = None;
        if event::poll(Duration::from_secs(0)).unwrap() {
            let event = event::read().unwrap();
            if let Event::Key(ke) = event {
                key = Some(ke);
                if ke.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
        terxel::set_cursor_position(std::io::stdout(), 0, 0).unwrap();
        canvas.clear();
        snake.update(key.as_ref(), canvas.width(), canvas.height());
        food.update(&mut snake, canvas.width(), canvas.height());
        snake.draw(&mut canvas);
        food.draw(&mut canvas);
        println!("{}", canvas.render().replace("\n", "\r\n"));
        std::io::stdout().flush().unwrap();
        std::thread::sleep(Duration::from_secs_f32(1.0/12.0));
    }
    disable_raw_mode().unwrap();
    execute!(std::io::stdout(), Show, LeaveAlternateScreen).unwrap();
}
