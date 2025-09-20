pub mod base;

use crossterm::event::{poll, read, Event, KeyCode};

use crate::base::{clear_field, flush, Cell, Vector2i};
use std::{collections::VecDeque, io::stdout, thread, time::Duration};

fn main() -> std::io::Result<()> {
    println!("Hello World!");

    // Field
    const WIDTH: usize = 40;
    const HEIGHT: usize = 10;

    let mut stdout = stdout();
    let mut matrix = [Cell::Empty; WIDTH * HEIGHT];

    // Snake
    let mut snake = VecDeque::from(vec![5, 5 + WIDTH, 5 + WIDTH * 2]);

    // Apples
    let mut apple = Vector2i::new(25, 5);

    // I/O
    let mut event_queue = VecDeque::new();

    let mut direction = Vector2i::DOWN;

    loop {
        // I/O
        while poll(Duration::from_millis(0))? {
            let event = read()?;
            event_queue.push_back(event);
        }

        while let Some(event) = event_queue.pop_front() {
            match event {
                Event::Key(key_event) => {
                    match key_event.code {
                        KeyCode::Char('q') => {
                            println!("Выход...");
                            return Ok(());
                        }
                        KeyCode::Down => direction = Vector2i::DOWN,
                        KeyCode::Up => direction = Vector2i::UP,
                        KeyCode::Right => direction = Vector2i::RIGHT,
                        KeyCode::Left => direction = Vector2i::LEFT,
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        // Main Cycle
        clear_field(&mut matrix);
        move_snake(&mut snake, &direction, &mut apple, WIDTH, HEIGHT);
        render_snake(&mut matrix, &snake);
        render_apple(&mut matrix, WIDTH, &apple);
        flush(&matrix, WIDTH, &mut stdout);
        thread::sleep(Duration::from_millis(500));
    };
}

fn handle_apple_collision(snake: &mut VecDeque<usize>, apple: &mut Vector2i, width: usize, height: usize) {
    snake.push_back(apple.to_usize(&width));
    *apple = pick_random_pos(snake, width, height);
}

fn pick_random_pos(snake: &mut VecDeque<usize>, width: usize, height: usize) -> Vector2i {
    let new = Vector2i::new(rand::random_range(0..width as i32), rand::random_range(0..height as i32));
    
    if snake.contains(&new.to_usize(&width)) {
        return pick_random_pos(snake, width, height);
    }
 
    return new;
}

fn move_snake(snake: &mut VecDeque<usize>, target_dir: &Vector2i, apple: &mut Vector2i, width: usize, height: usize) {
    let last = Vector2i::from_usize(&snake.back().unwrap(), &width);
    let prelast = Vector2i::from_usize(&snake[snake.len()-2], &width);

    let direction = if last + target_dir == prelast { last - prelast } else { target_dir.to_owned() };

    let next = last + direction;

    if *apple == next
    {
        handle_apple_collision(snake, apple, width, height);
    }
    else {
        snake.pop_front();
        snake.push_back(next.to_usize(&width));
    }
}

fn render_apple(matrix: &mut [Cell], width: usize, apple: &Vector2i) {
    matrix[apple.to_usize(&width)] = Cell::Apple;
}

fn render_snake(matrix: &mut [Cell], snake: &VecDeque<usize>) {
    for part in snake {
        matrix[*part] = Cell::Snake;
    }
}
