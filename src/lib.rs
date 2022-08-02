use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction {
  Up,
  Right,
  Down,
  Left,
}

pub struct SnakeCell(usize);

struct Snake {
  body: Vec<SnakeCell>,
  direction: Direction,
}

impl Snake {
  fn new(spawn_index: usize, size: usize) -> Snake {
    let mut body = vec![];

    for i in 0..size {
      body.push(SnakeCell(spawn_index - i));
    }

    Snake {
      body,
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
      snake: Snake::new(snake_index, 3),
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

  pub fn snake_length(&self) -> usize {
    self.snake.body.len()
  }

  // -> &Vec<SnakeCell> cannot return a reference (&self.snake.body) to JavaScript.
  // *const is a raw pointer and borrowing rules doesn't apply to it.
  pub fn snake_cells(&self) -> *const SnakeCell {
    self.snake.body.as_ptr()
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

  pub fn change_snake_direction(&mut self, direction: Direction) {
    self.snake.direction = direction;
  }
}

// wasm-pack build --target web
