use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen(module = "/server/utils/date.ts")]
extern "C" {
  fn now() -> usize;
}

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction {
  Up,
  Right,
  Down,
  Left,
}

#[derive(Clone, Copy)]
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
  next_cell: Option<SnakeCell>,
  reward_cell: usize,
}

#[wasm_bindgen]
impl World {
  pub fn new(width: usize, snake_index: usize) -> World {
    World {
      width,
      size: width * width,
      snake: Snake::new(snake_index, 3),
      next_cell: None,
      reward_cell: now() % (width * width),
    }
  }

  pub fn width(&self) -> usize {
    self.width
  }

  pub fn reward_cell(&self) -> usize {
    self.reward_cell
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
    let initial_snake_body = self.snake.body.clone();
    let snake_body_length = self.snake.body.len();

    match self.next_cell {
      Some(cell) => {
        // Move head cell
        self.set_snake_head_index(cell);
        self.next_cell = None;
      }
      None => {
        self.set_snake_head_index(self.generate_next_snake_cell(&self.snake.direction));
      }
    }

    // Move body cells
    for i in 1..snake_body_length {
      self.snake.body[i] = SnakeCell(initial_snake_body[i - 1].0)
    }
  }

  fn generate_next_snake_cell(&self, direction: &Direction) -> SnakeCell {
    let actual_row = self.snake_head_index() / self.width;

    return match direction {
      Direction::Right => {
        let treshhold = (actual_row + 1) * self.width;
        if self.snake_head_index() + 1 == treshhold {
          SnakeCell(treshhold - self.width)
        } else {
          SnakeCell(self.snake_head_index() + 1)
        }
      }
      Direction::Left => {
        let treshhold = actual_row * self.width;
        if self.snake_head_index() == treshhold {
          SnakeCell(treshhold + (self.width - 1))
        } else {
          SnakeCell(self.snake_head_index() - 1)
        }
      }
      Direction::Up => {
        let treshhold = self.snake_head_index() - (actual_row * self.width);
        if self.snake_head_index() == treshhold {
          SnakeCell((self.size - self.width) + treshhold)
        } else {
          SnakeCell(self.snake_head_index() - self.width)
        }
      }
      Direction::Down => {
        let treshhold = self.snake_head_index() + ((self.width - actual_row) * self.width);
        if self.snake_head_index() + self.width == treshhold {
          SnakeCell(treshhold - ((actual_row + 1) * self.width))
        } else {
          SnakeCell(self.snake_head_index() + self.width)
        }
      }
    };
  }

  pub fn change_snake_direction(&mut self, direction: Direction) {
    let next_cell = self.generate_next_snake_cell(&direction);

    if self.snake.body[1].0 == next_cell.0 {
      return;
    }

    self.next_cell = Some(next_cell);
    self.snake.direction = direction;
  }
}

// wasm-pack build --target web
