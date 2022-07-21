use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

struct SnakeCell(usize);

struct Snake {
  body: Vec<SnakeCell>,
}

impl Snake {
  fn new(spawn_index: usize) -> Snake {
    Snake {
      body: vec![SnakeCell(spawn_index)],
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
  pub fn new(width: usize) -> World {
    World {
      width,
      size: width * width,
      snake: Snake::new(10),
    }
  }

  pub fn width(&self) -> usize {
    self.width
  }

  pub fn snake_head_index(&self) -> usize {
    self.snake.body[0].0
  }

  pub fn move_snake_right(&mut self) {
    if self.snake_head_index() == (self.size - 1) {
      self.snake.body[0].0 = 0;
    } else {
      self.snake.body[0].0 = self.snake_head_index() + 1;
    }
  }
}

// wasm-pack build --target web
