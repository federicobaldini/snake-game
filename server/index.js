import init, { World } from "snake-game";

init().then(() => {
  const gameWorld = World.new();
  const canvas = document.getElementById("snake-game-canvas");
  if (canvas) {
    const canvasContext = canvas.getContext("2d");
  }
});
