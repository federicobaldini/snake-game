use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[derive(PartialEq)]
enum Direction {
  Up,
  Right,
  Down,
  Left,
}

struct SnakeCell(usize);

struct Snake {
  body: Vec<SnakeCell>,
  direction: Direction,
}

impl Snake {
  fn new(spawn_index: usize) -> Snake {
    Snake {
      body: vec![SnakeCell(spawn_index)],
      direction: Direction::Left,
    }
  }
}

#[wasm_bindgen]
pub struct World {
  width: usize,
  size: usize,
  snake: Snake,
}

#[wasm_bindgen]
impl World {
  pub fn new(width: usize, snake_index: usize) -> World {
    World {
      width,
      size: width * width,
      snake: Snake::new(snake_index),
    }
  }

  pub fn width(&self) -> usize {
    self.width
  }

  pub fn snake_head_index(&self) -> usize {
    self.snake.body[0].0
  }

  pub fn move_snake(&mut self) {
    let row = self.snake_head_index() / self.width;

    match &self.snake.direction {
      Direction::Right => {
        let next_column = (self.snake_head_index() + 1) % self.width;
        self.snake.body[0].0 = (row * self.width) + next_column;
      }
      Direction::Left => {
        let next_column = (self.snake_head_index() - 1) % self.width;
        self.snake.body[0].0 = (row * self.width) + next_column;
      }
      _ => (),
    }
  }
}

// wasm-pack build --target web
