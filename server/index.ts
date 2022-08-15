import init, { Direction, InitOutput, World } from "snake-game";
import { random } from "./utils/random";

const drawWorld = (
  context: CanvasRenderingContext2D,
  gameWorld: World,
  cellSize: number
): void => {
  context.beginPath();

  for (let x = 0; x < gameWorld.width() + 1; x++) {
    context.moveTo(cellSize * x, 0);
    context.lineTo(cellSize * x, gameWorld.width() * cellSize);
  }

  for (let y = 0; y < gameWorld.width() + 1; y++) {
    context.moveTo(0, cellSize * y);
    context.lineTo(gameWorld.width() * cellSize, cellSize * y);
  }

  context.stroke();
};

const drawReward = (
  context: CanvasRenderingContext2D,
  gameWorld: World,
  cellSize: number
) => {
  const column = gameWorld.reward_cell() % gameWorld.width();
  const row = Math.floor(gameWorld.reward_cell() / gameWorld.width());

  context.beginPath();
  context.fillStyle = "#FF0000";
  context.fillRect(column * cellSize, row * cellSize, cellSize, cellSize);
  context.stroke();
};

const drawSnake = (
  context: CanvasRenderingContext2D,
  gameWorld: World,
  cellSize: number,
  wasm: InitOutput
): void => {
  const snakeCells = new Uint32Array(
    wasm.memory.buffer,
    gameWorld.snake_cells(),
    gameWorld.snake_length()
  );

  // or .slice().reverse() instead of .filter()
  snakeCells
    .filter(
      (snakeCellIndex, index) =>
        !(index > 0 && snakeCellIndex === snakeCells[0])
    )
    .forEach((snakeCellIndex, index) => {
      const column = snakeCellIndex % gameWorld.width();
      const row = Math.floor(snakeCellIndex / gameWorld.width());

      context.fillStyle = !index ? "#7979db" : "#000000";

      context.beginPath();
      context.fillRect(column * cellSize, row * cellSize, cellSize, cellSize);
      context.stroke();
    });
};

const drawGameStatus = (gameWorld: World): void => {
  const gameStatus: HTMLLabelElement = document.getElementById(
    "game-status"
  ) as HTMLLabelElement;

  gameStatus.textContent = gameWorld.status_text();
};

const drawGamePoints = (gameWorld: World): void => {
  const gameStatus: HTMLLabelElement = document.getElementById(
    "game-points"
  ) as HTMLLabelElement;

  gameStatus.textContent = String(gameWorld.points());
};

const createGameWorld = (
  context: CanvasRenderingContext2D,
  gameWorld: World,
  cellSize: number,
  wasm: InitOutput
): void => {
  drawWorld(context, gameWorld, cellSize);
  drawSnake(context, gameWorld, cellSize, wasm);
  drawReward(context, gameWorld, cellSize);
  drawGameStatus(gameWorld);
  drawGamePoints(gameWorld);
};

const updateGameWorld = (
  canvas: HTMLCanvasElement,
  context: CanvasRenderingContext2D,
  cellSize: number,
  gameWorld: World,
  wasm: InitOutput,
  updateTimeout: { timeout: NodeJS.Timeout }
): void => {
  const frame_per_second: number = 10;

  updateTimeout.timeout = setTimeout(() => {
    context.clearRect(0, 0, canvas.width, canvas.height);
    gameWorld.move_snake();
    createGameWorld(context, gameWorld, cellSize, wasm);
    if (gameWorld.status() === 2) {
      requestAnimationFrame(() =>
        updateGameWorld(
          canvas,
          context,
          cellSize,
          gameWorld,
          wasm,
          updateTimeout
        )
      );
    } else {
      clearTimeout(updateTimeout.timeout);
    }
  }, 1000 / frame_per_second);
};

init().then((wasm: InitOutput) => {
  const CELL_SIZE: number = 20;
  const WORLD_WIDTH: number = 16;

  const snake_spawn_index: number = random(WORLD_WIDTH * WORLD_WIDTH);
  const gameWorld: World = World.new(WORLD_WIDTH, snake_spawn_index);
  const gameWorldWidth: number = gameWorld.width();

  document.addEventListener("keydown", (event: KeyboardEvent) => {
    switch (event.code) {
      case "ArrowUp":
      case "KeyW":
        gameWorld.change_snake_direction(Direction.Up);
        break;
      case "ArrowRight":
      case "KeyD":
        gameWorld.change_snake_direction(Direction.Right);
        break;
      case "ArrowDown":
      case "KeyS":
        gameWorld.change_snake_direction(Direction.Down);
        break;
      case "ArrowLeft":
      case "KeyA":
        gameWorld.change_snake_direction(Direction.Left);
        break;
    }
  });

  const canvas: HTMLCanvasElement = document.getElementById(
    "snake-game-canvas"
  ) as HTMLCanvasElement;

  if (canvas) {
    const canvasContext: CanvasRenderingContext2D = canvas.getContext("2d");
    canvas.width = gameWorldWidth * CELL_SIZE;
    canvas.height = gameWorldWidth * CELL_SIZE;

    createGameWorld(canvasContext, gameWorld, CELL_SIZE, wasm);

    const gameControlButton: HTMLButtonElement = document.getElementById(
      "play-button"
    ) as HTMLButtonElement;
    const updateTimeout: { timeout: NodeJS.Timeout } = {
      timeout: undefined,
    };

    gameControlButton.addEventListener("click", (_) => {
      if (gameWorld.status() === undefined) {
        gameWorld.start_game();
        updateGameWorld(
          canvas,
          canvasContext,
          CELL_SIZE,
          gameWorld,
          wasm,
          updateTimeout
        );
      } else if (gameWorld.status() !== 2) {
        gameWorld.start_game();
        clearTimeout(updateTimeout.timeout);
        updateGameWorld(
          canvas,
          canvasContext,
          CELL_SIZE,
          gameWorld,
          wasm,
          updateTimeout
        );
        gameControlButton.textContent = "PAUSE!";
      } else {
        gameWorld.pause_game();
        clearTimeout(updateTimeout.timeout);
        gameControlButton.textContent = "PLAY!";
      }
    });
  }
});
