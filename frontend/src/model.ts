import { Vec2 } from "./math";

export type Tile = "X" | "O" | null;

export class ConnectN {
  dim: Vec2;
  winLength: number;
  board: Tile[];
  columnHeights: number[];
  winCell: Vec2 | null;
  winDirection: Vec2 | null;
  toMove: Tile;
  isTie: boolean;

  get gameOver() {
    return this.winCell !== null || this.isTie;
  }

  constructor(width: number, height: number, winLength: number) {
    this.dim = new Vec2(width, height);
    this.winLength = winLength;
    this.board = [];
    for (let i = 0; i < width * height; i++) {
      this.board.push(null);
    }
    this.columnHeights = [];
    for (let i = 0; i < width; i++) {
      this.columnHeights.push(0);
    }
    this.winCell = null;
    this.winDirection = null;
    this.toMove = "X";
    this.isTie = false;
  }

  get(x: number, y: number) {
    if (x < 0 || x >= this.dim.x || y < 0 || y >= this.dim.y) {
      throw new Error("index out of bounds");
    }
    return this.board[y * this.dim.x + x];
  }

  set(x: number, y: number, tile: Tile) {
    if (x < 0 || x >= this.dim.x || y < 0 || y >= this.dim.y) {
      throw new Error("index out of bounds");
    }
    this.board[y * this.dim.x + x] = tile;
  }

  play(column: number) {
    if (column < 0 || column >= this.dim.x) {
      throw new Error("index out of bounds");
    }
    if (this.columnHeights[column] === this.dim.y) {
      throw new Error("column is filled");
    }
    this.set(column, this.columnHeights[column], this.toMove);
    this.columnHeights[column]++;
    if (this.toMove === "X") {
      this.toMove = "O";
    } else {
      this.toMove = "X";
    }
    this.checkWin();
  }

  checkWin() {
    const directions = [
      new Vec2(1, 0),
      new Vec2(1, 1),
      new Vec2(0, 1),
      new Vec2(-1, 1),
    ];
    for (let x = 0; x < this.dim.x; x++) {
      for (let y = 0; y < this.dim.y; y++) {
        const tile = this.get(x, y);
        if (tile === null) {
          continue;
        }
        for (const dir of directions) {
          let pos = new Vec2(x, y);
          let success = true;
          for (let k = 0; k < this.winLength; k++) {
            if (
              pos.x < 0 ||
              pos.x >= this.dim.x ||
              pos.y < 0 ||
              pos.y >= this.dim.y
            ) {
              success = false;
              break;
            }
            if (tile !== this.get(pos.x, pos.y)) {
              success = false;
              break;
            }
            pos = pos.add(dir);
          }
          if (success) {
            this.winCell = new Vec2(x, y);
            this.winDirection = dir;
            return;
          }
        }
      }
    }

    // Check for ties
    let found = false;
    for (let i = 0; i < this.dim.x; i++) {
      for (let j = 0; j < this.dim.y; j++) {
        if (this.get(i, j) === null) {
          found = true;
        }
      }
    }
    if (!found) {
      this.isTie = true;
    }
  }
}
