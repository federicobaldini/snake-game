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

  fn set_snake_head_index(&mut self, cell: SnakeCell) {
    self.snake.body[0] = cell
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
    self.set_snake_head_index(self.generate_next_snake_cell());
  }

  fn generate_next_snake_cell(&self) -> SnakeCell {
    let actual_row = self.snake_head_index() / self.width;

    return match &self.snake.direction {
      Direction::Right => {
        SnakeCell((actual_row * self.width) + (self.snake_head_index() + 1) % self.width)
      }
      Direction::Left => {
        SnakeCell((actual_row * self.width) + (self.snake_head_index() - 1) % self.width)
      }
      Direction::Up => SnakeCell((self.snake_head_index() - self.width) % self.size),
      Direction::Down => SnakeCell((self.snake_head_index() + self.width) % self.size),
    };
  }

  pub fn change_snake_direction(&mut self, direction: Direction) {
    self.snake.direction = direction;
  }
}

// wasm-pack build --target web
