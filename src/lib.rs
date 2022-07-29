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
      direction: Direction::Up,
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

  fn cell_to_index(&self, row: usize, column: usize) -> usize {
    (row * self.width) + column
  }

  fn index_to_cell(&self, index: usize) -> (usize, usize) {
    (index / self.width, index % self.width)
  }

  fn set_snake_head_index(&mut self, index: usize) {
    self.snake.body[0].0 = index
  }

  pub fn move_snake(&mut self) {
    let (actual_row, actual_column) = self.index_to_cell(self.snake_head_index());
    let (next_row, next_column) = match &self.snake.direction {
      Direction::Right => (actual_row, (actual_column + 1) % self.width),
      Direction::Left => (actual_row, (actual_column - 1) % self.width),
      Direction::Up => ((actual_row - 1) % self.width, actual_column),
      Direction::Down => ((actual_row + 1) % self.width, actual_column),
      _ => (actual_row, actual_column),
    };
    self.set_snake_head_index(self.cell_to_index(next_row, next_column));
  }
}

// wasm-pack build --target web
