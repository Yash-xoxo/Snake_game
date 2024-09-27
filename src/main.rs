extern crate pancurses;

use pancurses::{initscr, endwin, Input, noecho, timeout};
use std::collections::VecDeque;
use std::time::Duration;
use std::thread;

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Snake {
    body: VecDeque<(i32, i32)>,
    direction: Direction,
}

impl Snake {
    fn new(x: i32, y: i32) -> Snake {
        let mut body = VecDeque::new();
        body.push_back((x, y));
        Snake {
            body,
            direction: Direction::Right,
        }
    }

    fn move_forward(&mut self, grow: bool) {
        let head = *self.body.front().expect("Snake has no body");
        let new_head = match self.direction {
            Direction::Up => (head.0 - 1, head.1),
            Direction::Down => (head.0 + 1, head.1),
            Direction::Left => (head.0, head.1 - 1),
            Direction::Right => (head.0, head.1 + 1),
        };
        self.body.push_front(new_head);
        if !grow {
            self.body.pop_back();
        }
    }

    fn change_direction(&mut self, new_direction: Direction) {
        self.direction = new_direction;
    }

    fn head_position(&self) -> (i32, i32) {
        *self.body.front().expect("Snake has no body")
    }

    fn is_colliding(&self, pos: (i32, i32)) -> bool {
        self.body.contains(&pos)
    }
}

fn main() {
    let window = initscr();
    window.keypad(true);
    noecho();
    timeout(100);

    let (max_y, max_x) = window.get_max_yx();

    let mut snake = Snake::new(max_y / 2, max_x / 2);
    let mut food = (max_y / 4, max_x / 4);
    let mut score = 0;

    let mut grow = false;

    loop {
        window.clear();

        // Draw Snake
        for &(y, x) in snake.body.iter() {
            window.mvaddch(y, x, 'O');
        }

        // Draw Food
        window.mvaddch(food.0, food.1, 'X');

        // Check user input
        match window.getch() {
            Some(Input::KeyUp) if snake.direction != Direction::Down => snake.change_direction(Direction::Up),
            Some(Input::KeyDown) if snake.direction != Direction::Up => snake.change_direction(Direction::Down),
            Some(Input::KeyLeft) if snake.direction != Direction::Right => snake.change_direction(Direction::Left),
            Some(Input::KeyRight) if snake.direction != Direction::Left => snake.change_direction(Direction::Right),
            Some(Input::Character('q')) => break,
            _ => {}
        }

        // Move Snake
        snake.move_forward(grow);
        grow = false;

        // Check for collisions
        let head_pos = snake.head_position();
        if head_pos == food {
            food = ((head_pos.0 + 3) % max_y, (head_pos.1 + 5) % max_x);
            grow = true;
            score += 1;
        }

        if head_pos.0 <= 0 || head_pos.0 >= max_y - 1 || head_pos.1 <= 0 || head_pos.1 >= max_x - 1 || snake.is_colliding(head_pos) {
            break;  // Game over on collision
        }

        // Update screen
        window.mvprintw(0, 0, &format!("Score: {}", score));
        window.refresh();

        thread::sleep(Duration::from_millis(100));
    }

    endwin();
    println!("Game Over! Final score: {}", score);
}

