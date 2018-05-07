import { Universe } from "./hello_world";

const pre = document.getElementById("game-of-life-canvas");
const universe = Universe.new();

const renderLoop = () => {
  pre.textContent = universe.render();
  universe.tick();
  setTimeout(() => requestAnimationFrame(renderLoop), 500);
};

requestAnimationFrame(renderLoop);
