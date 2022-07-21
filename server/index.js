import init, { World } from "snake-game";

const drawWorld = (context, worldWidth, cellSize) => {
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

const drawSnake = (context, worldWidth, cellSize, snakeHeadIndex) => {
  const column = snakeHeadIndex % worldWidth;
  const row = Math.floor(snakeHeadIndex / worldWidth);

  context.beginPath();
  context.fillRect(column * cellSize, row * cellSize, cellSize, cellSize);
  context.stroke();
};

init().then(() => {
  const CELL_SIZE = 20;

  const gameWorld = World.new(8);
  const gameWorldWidth = gameWorld.width();

  const canvas = document.getElementById("snake-game-canvas");
  if (canvas) {
    const canvasContext = canvas.getContext("2d");
    canvas.width = gameWorldWidth * CELL_SIZE;
    canvas.height = gameWorldWidth * CELL_SIZE;
    drawWorld(canvasContext, gameWorldWidth, CELL_SIZE);
    drawSnake(
      canvasContext,
      gameWorldWidth,
      CELL_SIZE,
      gameWorld.snake_head_index()
    );
    setInterval(() => {
      canvasContext.clearRect(0, 0, canvas.width, canvas.height);
      drawWorld(canvasContext, gameWorldWidth, CELL_SIZE);
      drawSnake(
        canvasContext,
        gameWorldWidth,
        CELL_SIZE,
        gameWorld.snake_head_index()
      );
      gameWorld.move_snake_right();
    }, 500);
  }
});
