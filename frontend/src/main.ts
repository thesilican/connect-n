import "sanitize.css";
import "./style.css";
import { Canvas, Texture } from "./canvas";
import { Vec2, clamp } from "./math";
import cursorPng from "./assets/cursor.png";
import { ConnectN } from "./model";
import AiWorker from "./worker?worker";

const cursor = new Texture(cursorPng, 256, 256);
await cursor.load;

const canvasElement = document.getElementById("canvas") as HTMLCanvasElement;
const canvas = new Canvas(canvasElement);
const aiPlaysAs = "O";
const ai = new AiWorker();

const connectN = new ConnectN(7, 6, 4);
let fallingPiece: Vec2 | null = null;
let fallingPieceVelocity = 0;
let aiThinking = false;

canvasElement.addEventListener("click", () => handleClick());
ai.addEventListener("message", (evt) => {
  console.log("AI Response:", evt.data);
  handleAiResponse(evt.data);
});

function handleClick() {
  const { mouseColumn } = calculateSizing();
  if (
    connectN.gameOver ||
    fallingPiece !== null ||
    connectN.columnHeights[mouseColumn] === connectN.dim.y
  ) {
    return;
  }
  fallingPiece = new Vec2(mouseColumn, connectN.dim.y);
  fallingPieceVelocity = 0.1;
}

function handleAiResponse(column: number) {
  aiThinking = false;
  if (
    connectN.gameOver ||
    fallingPiece !== null ||
    connectN.columnHeights[column] === connectN.dim.y
  ) {
    return;
  }
  fallingPiece = new Vec2(column, connectN.dim.y);
  fallingPieceVelocity = 0.1;
}

function calculateSizing() {
  const cellSize = Math.floor(
    0.75 *
      Math.min(canvas.dim.x / connectN.dim.x, canvas.dim.y / connectN.dim.y)
  );
  const leftOffset = Math.floor((canvas.dim.x - cellSize * connectN.dim.x) / 2);
  const topOffset = Math.floor((canvas.dim.y - cellSize * connectN.dim.y) / 2);
  const mouseColumn = clamp(
    Math.floor((canvas.mouse.x - leftOffset) / cellSize),
    0,
    connectN.dim.x - 1
  );

  return { cellSize, leftOffset, topOffset, mouseColumn };
}

function tick() {
  canvas.clear();

  // Falling piece physics
  if (fallingPiece !== null) {
    fallingPiece.y -= fallingPieceVelocity;
    fallingPieceVelocity += 0.1;
    if (fallingPiece.y <= connectN.columnHeights[fallingPiece.x]) {
      connectN.play(fallingPiece.x);
      fallingPiece = null;
    }
  }

  // Trigger AI
  if (
    !connectN.gameOver &&
    !aiThinking &&
    !fallingPiece &&
    connectN.toMove == aiPlaysAs
  ) {
    ai.postMessage(connectN.board);
    aiThinking = true;
  }

  const { cellSize, leftOffset, topOffset, mouseColumn } = calculateSizing();

  // Draw mouse piece
  if (!connectN.gameOver && fallingPiece == null) {
    // Draw floating piece
    const color = connectN.toMove === "X" ? "rgb(255, 0, 0)" : "rgb(0, 0, 255)";
    const position = new Vec2(
      leftOffset + mouseColumn * cellSize + cellSize / 2,
      topOffset - cellSize / 2
    );
    canvas.fillCircle(position, cellSize * 0.4, color);

    // Draw preview piece
    // if (connectN.columnHeights[mouseColumn] < connectN.dim.y) {
    //   const ghostColor =
    //     connectN.toMove === "X"
    //       ? "rgba(255, 0, 0, 0.5)"
    //       : "rgba(0, 0, 255, 0.5)";
    //   const ghostPosition = new Vec2(
    //     leftOffset + mouseColumn * cellSize + cellSize / 2,
    //     topOffset +
    //       (connectN.dim.y - connectN.columnHeights[mouseColumn] - 1) *
    //         cellSize +
    //       cellSize / 2
    //   );
    //   canvas.fillCircle(ghostPosition, cellSize * 0.4, ghostColor);
    // }
  }

  // Draw falling piece
  if (fallingPiece !== null) {
    const color = connectN.toMove === "X" ? "rgb(255, 0, 0)" : "rgb(0, 0, 255)";
    const position = new Vec2(
      leftOffset + fallingPiece.x * cellSize + cellSize / 2,
      topOffset +
        (connectN.dim.y - fallingPiece.y - 1) * cellSize +
        cellSize / 2
    );
    canvas.fillCircle(position, cellSize * 0.4, color);
  }

  // Draw circles
  for (let i = 0; i < connectN.dim.x; i++) {
    for (let j = 0; j < connectN.dim.y; j++) {
      const tile = connectN.get(i, j);
      if (tile === null) {
        continue;
      }
      const color = tile === "X" ? "rgb(255, 0, 0)" : "rgb(0, 0, 255)";
      const position = new Vec2(
        leftOffset + i * cellSize + cellSize / 2,
        topOffset + (connectN.dim.y - j - 1) * cellSize + cellSize / 2
      );
      canvas.fillCircle(position, cellSize * 0.4, color);
    }
  }

  // Draw game board
  for (let i = 0; i < connectN.dim.x; i++) {
    for (let j = 0; j < connectN.dim.y; j++) {
      const x = leftOffset + i * cellSize;
      const y = topOffset + j * cellSize;
      canvas.ctx.save();
      canvas.ctx.fillStyle = "rgb(25, 25, 25)";
      canvas.ctx.beginPath();
      canvas.ctx.rect(x, y, cellSize, cellSize);
      canvas.ctx.arc(
        x + cellSize / 2,
        y + cellSize / 2,
        cellSize * 0.4,
        0,
        2 * Math.PI,
        true
      );
      canvas.ctx.clip();
      canvas.ctx.fillRect(x, y, cellSize, cellSize);
      canvas.ctx.restore();
    }
  }

  // Draw win line
  if (connectN.winCell !== null && connectN.winDirection !== null) {
    canvas.ctx.lineWidth = cellSize / 10;
    canvas.ctx.lineCap = "round";
    canvas.ctx.strokeStyle = "#fff";
    canvas.ctx.beginPath();
    canvas.ctx.moveTo(
      leftOffset + connectN.winCell.x * cellSize + cellSize / 2,
      topOffset +
        (connectN.dim.y - connectN.winCell.y - 1) * cellSize +
        cellSize / 2
    );
    const endPoint = connectN.winCell.add(
      connectN.winDirection.scale(connectN.winLength - 1)
    );
    canvas.ctx.lineTo(
      leftOffset + endPoint.x * cellSize + cellSize / 2,
      topOffset + (connectN.dim.y - endPoint.y - 1) * cellSize + cellSize / 2
    );
    canvas.ctx.stroke();
  }

  // Draw status text
  let message: string;
  if (connectN.winCell !== null) {
    const winner = connectN.get(connectN.winCell.x, connectN.winCell.y);
    message = winner === "X" ? "Red wins!" : "Blue wins!";
  } else if (connectN.isTie) {
    message = "It's a tie!";
  } else if (aiThinking) {
    message = "Thinking...";
  } else {
    message = `Connect ${connectN.winLength} (${connectN.dim.x}\u00D7${connectN.dim.y})`;
  }
  const position = new Vec2(
    Math.floor(leftOffset + (connectN.dim.x * cellSize) / 2),
    Math.floor(topOffset + connectN.dim.y * cellSize + topOffset * 0.1)
  );
  canvas.fillText(message, position, {
    justification: "center",
    fontSize: Math.floor(topOffset * 0.6),
  });

  // Draw mouse cursor
  canvas.drawTexture(
    cursor,
    canvas.mouse,
    new Vec2(32 * window.devicePixelRatio, 32 * window.devicePixelRatio)
  );

  requestAnimationFrame(tick);
}
requestAnimationFrame(tick);
