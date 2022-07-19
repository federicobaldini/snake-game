import init, { World } from "snake-game";

init().then(() => {
  const gameWorld = World.new();
  console.log(gameWorld.width());
});
