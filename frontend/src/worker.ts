import type { Tile } from "./model";

self.addEventListener("message", (evt) => {
  console.log("AI Request:", evt.data);
  const board: number[] = evt.data.map((x: Tile) =>
    x === "X" ? 1 : x === "O" ? 2 : 0
  );

  // For now we return the sum of board mod 7
  const response = board.reduce((a, v) => a + v, 0) % 7;

  // Simulate a delay
  setTimeout(() => {
    self.postMessage(response);
  }, 1000);
});
