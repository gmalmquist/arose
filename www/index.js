import * as wasm from "arose";

const arose = wasm.setup();

console.log("arose:", arose, Object.keys(arose));

const mainLoop = () => {
  arose.update();
};

setInterval(mainLoop, 15);

window.addEventListener('keydown', event => {
  arose.handle_key_down(event.key);
  event.preventDefault();
});

window.addEventListener("mousemove", event => {
  arose.handle_mouse_move(event.offsetX, event.offsetY);
  event.preventDefault();
});

window.addEventListener("mouseup", event => {
  arose.handle_mouse_up(event.offsetX, event.offsetY);
  event.preventDefault();
});

window.addEventListener("mousedown", event => {
  arose.handle_mouse_down(event.offsetX, event.offsetY);
  event.preventDefault();
});
