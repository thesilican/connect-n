import { Rect, Vec2 } from "./math";

type FillTextOptions = {
  fontSize?: number;
  color?: string;
  justification?: "left" | "center" | "right";
  debug?: boolean;
};

export class Canvas {
  public canvas: HTMLCanvasElement;
  public ctx: CanvasRenderingContext2D;
  public dim = new Vec2(0, 0);
  public mouse = new Vec2(0, 0);

  constructor(canvas: HTMLCanvasElement) {
    this.canvas = canvas;
    const ctx = canvas.getContext("2d");
    if (ctx === null) {
      throw new Error("Cannot obtain 2d context");
    }
    this.ctx = ctx;
    this.updateDim();
    window.addEventListener("resize", () => this.updateDim());
    this.canvas.addEventListener("mousemove", (e) =>
      this.updateMouse(new Vec2(e.clientX, e.clientY))
    );
  }

  private updateDim() {
    this.dim.x = this.canvas.clientWidth * window.devicePixelRatio;
    this.dim.y = this.canvas.clientHeight * window.devicePixelRatio;
    this.canvas.width = this.dim.x;
    this.canvas.height = this.dim.y;
  }

  private updateMouse(mouse: Vec2) {
    this.mouse = mouse.scale(window.devicePixelRatio);
  }

  fillRect(rect: Rect, color = "black") {
    this.ctx.fillStyle = color;
    this.ctx.fillRect(rect.min.x, rect.min.y, rect.width, rect.height);
  }

  fillCircle(position: Vec2, radius: number, color = "black") {
    this.ctx.fillStyle = color;
    this.ctx.beginPath();
    this.ctx.arc(position.x, position.y, radius, 0, 2 * Math.PI);
    this.ctx.fill();
  }

  fillText(text: string, position: Vec2, options?: FillTextOptions) {
    const fontSize = options?.fontSize ?? "24";
    const color = options?.color ?? "black";
    const justification = options?.justification ?? "left";
    const debug = options?.debug ?? false;

    this.ctx.font = `${fontSize}px Arial`;
    const metrics = this.ctx.measureText(text);
    const height =
      metrics.actualBoundingBoxAscent + metrics.actualBoundingBoxDescent;

    let x = position.x;
    if (justification === "center") {
      x -= Math.floor(metrics.width / 2);
    } else if (justification === "right") {
      x -= metrics.width;
    }

    if (debug) {
      this.ctx.fillStyle = "red";
      this.ctx.fillRect(x, position.y, metrics.width, height);
    }
    this.ctx.fillStyle = color;
    this.ctx.fillText(text, x, position.y + height);
  }

  drawTexture(texture: Texture, position: Vec2, size?: Vec2) {
    this.ctx.drawImage(
      texture.img,
      position.x,
      position.y,
      size?.x ?? texture.dim.x,
      size?.y ?? texture.dim.y
    );
  }

  clear() {
    this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
  }
}

export class Texture {
  img: HTMLImageElement;
  dim: Vec2;
  load: Promise<void>;

  constructor(src: string, width: number, height: number) {
    this.img = new Image(width, height);
    this.img.src = src;
    this.load = new Promise((res) => {
      this.img.addEventListener("load", () => {
        res();
      });
    });
    this.dim = new Vec2(width, height);
  }
}
