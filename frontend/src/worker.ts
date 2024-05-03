import type { Tile } from "./model";
import { bot } from "wasm";

self.addEventListener("message", (evt) => {
  console.log("AI Request:", evt.data);
  const board: number[] = evt.data.map((x: Tile) =>
    x === "X" ? 1 : x === "O" ? 2 : 0
  );

  const int32Array = new Int32Array(board);
  const response = bot(int32Array, 19);

  self.postMessage(response);
});
