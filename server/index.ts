import init, { World } from "snake-game";

const drawWorld = (
  context: CanvasRenderingContext2D,
  worldWidth: number,
  cellSize: number
): void => {
  context.beginPath();

  for (let x = 0; x < worldWidth + 1; x++) {
    context.moveTo(cellSize * x, 0);
    context.lineTo(cellSize * x, worldWidth * cellSize);
  }

  for (let y = 0; y < worldWidth + 1; y++) {
    context.moveTo(0, cellSize * y);
    context.lineTo(worldWidth * cellSize, cellSize * y);
  }

  context.stroke();
};

const drawSnake = (
  context: CanvasRenderingContext2D,
  worldWidth: number,
  cellSize: number,
  snakeHeadIndex: number
): void => {
  const column = snakeHeadIndex % worldWidth;
  const row = Math.floor(snakeHeadIndex / worldWidth);

  context.beginPath();
  context.fillRect(column * cellSize, row * cellSize, cellSize, cellSize);
  context.stroke();
};

const createGameWorld = (
  context: CanvasRenderingContext2D,
  worldWidth: number,
  cellSize: number,
  snakeHeadIndex: number
): void => {
  drawWorld(context, worldWidth, cellSize);
  drawSnake(context, worldWidth, cellSize, snakeHeadIndex);
};

const updateGameWorld = (
  canvas: HTMLCanvasElement,
  context: CanvasRenderingContext2D,
  worldWidth: number,
  cellSize: number,
  gameWorld: World
): void => {
  const frame_per_second: number = 5;
  setTimeout(() => {
    context.clearRect(0, 0, canvas.width, canvas.height);
    gameWorld.move_snake();
    createGameWorld(
      context,
      worldWidth,
      cellSize,
      gameWorld.snake_head_index()
    );
    requestAnimationFrame(() =>
      updateGameWorld(canvas, context, worldWidth, cellSize, gameWorld)
    );
  }, 1000 / frame_per_second);
};

init().then(() => {
  const CELL_SIZE: number = 20;
  const WORLD_WIDTH: number = 8;

  const snake_spawn_index: number = Date.now() % (WORLD_WIDTH * WORLD_WIDTH);
  const gameWorld: World = World.new(WORLD_WIDTH, snake_spawn_index);
  const gameWorldWidth: number = gameWorld.width();

  const canvas: HTMLCanvasElement = document.getElementById(
    "snake-game-canvas"
  ) as HTMLCanvasElement;
  if (canvas) {
    const canvasContext: CanvasRenderingContext2D = canvas.getContext("2d");
    canvas.width = gameWorldWidth * CELL_SIZE;
    canvas.height = gameWorldWidth * CELL_SIZE;

    createGameWorld(
      canvasContext,
      gameWorldWidth,
      CELL_SIZE,
      gameWorld.snake_head_index()
    );

    updateGameWorld(
      canvas,
      canvasContext,
      gameWorldWidth,
      CELL_SIZE,
      gameWorld
    );
  }
});
