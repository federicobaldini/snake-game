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

init().then(() => {
  const CELL_SIZE = 20;

  const gameWorld = World.new();
  const gameWorldWidth = gameWorld.width();

  const canvas = document.getElementById("snake-game-canvas");
  if (canvas) {
    const canvasContext = canvas.getContext("2d");
    canvas.width = gameWorldWidth * CELL_SIZE;
    canvas.height = gameWorldWidth * CELL_SIZE;
    drawWorld(canvasContext, gameWorldWidth, CELL_SIZE);
  }
});
