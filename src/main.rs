use std::{io::Write, time::Duration};

use crossterm::{cursor::{DisableBlinking, EnableBlinking, Hide, Show}, event::{self, Event, KeyCode, KeyEvent}, execute, terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode}};
use terxel::{Canvas, Color};

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

impl Snake {
    fn new() -> Snake {
        Snake {
            x: 0.0,
            y: 0.0,
            heading: Direction::Right,
            body: Vec::new(),
        }
    }

    fn update(&mut self, key: Option<&KeyEvent>, width: usize, height: usize) {
        if let Some(ke) = key {
            if ke.code == KeyCode::Right {
                self.heading = Direction::Right;
            }
            if ke.code == KeyCode::Up {
                self.heading = Direction::Up;
            }
            if ke.code == KeyCode::Down {
                self.heading = Direction::Down;
            }
            if ke.code == KeyCode::Left {
                self.heading = Direction::Left;
            }
            if ke.code == KeyCode::Char(' ') {
                self.body.push((self.x, self.y));
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
    let mut canvas = Canvas::new(20, 20);
    execute!(std::io::stdout(), EnterAlternateScreen, Hide).unwrap();
    enable_raw_mode().unwrap();

    let mut snake = Snake::new();
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
        snake.draw(&mut canvas);
        println!("{}", canvas.render().replace("\n", "\r\n"));
        std::io::stdout().flush().unwrap();
        std::thread::sleep(Duration::from_secs_f32(1.0/24.0));
    }
    disable_raw_mode().unwrap();
    execute!(std::io::stdout(), Show, LeaveAlternateScreen).unwrap();
}
