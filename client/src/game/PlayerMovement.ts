import Phaser from 'phaser';

const TILE_SIZE = 16;
const SPEED_TO_MS = 40_000;
// slightly slower than the server cooldown so held keys produce smooth continuous movement
const ANIMATION_SLOWDOWN = 1.15;

export type Movement =
  | 'North'
  | 'NorthEast'
  | 'East'
  | 'SouthEast'
  | 'South'
  | 'SouthWest'
  | 'West'
  | 'NorthWest';

const MOVEMENT_DELTA: Record<Movement, { dx: number; dy: number }> = {
  North: { dx: 0, dy: -1 },
  NorthEast: { dx: 1, dy: -1 },
  East: { dx: 1, dy: 0 },
  SouthEast: { dx: 1, dy: 1 },
  South: { dx: 0, dy: 1 },
  SouthWest: { dx: -1, dy: 1 },
  West: { dx: -1, dy: 0 },
  NorthWest: { dx: -1, dy: -1 },
};

export function tileToPixel(t: number): number {
  return t * TILE_SIZE + TILE_SIZE / 2;
}

type TileLookup = (x: number, y: number) => string | undefined;

export class PlayerMovement {
  private scene: Phaser.Scene;
  private playerSprite: Phaser.GameObjects.Image | null = null;
  private moveCallback: ((movement: Movement) => void) | null = null;
  private tileLookup: TileLookup | null = null;

  private predictedX = 0;
  private predictedY = 0;
  private serverX = 0;
  private serverY = 0;
  private isLerping = false;
  private speed = 120;
  private initialized = false;

  constructor(scene: Phaser.Scene) {
    this.scene = scene;
  }

  setMoveCallback(callback: (movement: Movement) => void) {
    this.moveCallback = callback;
  }

  setTileLookup(lookup: TileLookup) {
    this.tileLookup = lookup;
  }

  setSpeed(speed: number) {
    this.speed = speed;
  }

  get moving(): boolean {
    return this.isLerping;
  }

  get position(): { x: number; y: number } {
    return { x: this.predictedX, y: this.predictedY };
  }

  tryMove(movement: Movement) {
    if (this.isLerping || !this.moveCallback || !this.playerSprite) return;

    const delta = MOVEMENT_DELTA[movement];
    const targetX = this.predictedX + delta.dx;
    const targetY = this.predictedY + delta.dy;

    // client-side water check — don't even send the request
    if (this.tileLookup) {
      const tag = this.tileLookup(targetX, targetY);
      if (tag === 'Water') return;
    }

    this.predictedX = targetX;
    this.predictedY = targetY;

    const duration = (SPEED_TO_MS / this.speed) * ANIMATION_SLOWDOWN;
    this.isLerping = true;

    this.scene.tweens.add({
      targets: this.playerSprite,
      x: tileToPixel(targetX),
      y: tileToPixel(targetY),
      duration,
      ease: 'Linear',
      onComplete: () => {
        this.isLerping = false;
        this.reconcile();
      },
    });

    this.moveCallback(movement);
  }

  updateServerPosition(x: number, y: number) {
    this.serverX = x;
    this.serverY = y;

    const pixelX = tileToPixel(x);
    const pixelY = tileToPixel(y);

    if (!this.playerSprite) {
      this.playerSprite = this.scene.add
        .image(pixelX, pixelY, 'player')
        .setDisplaySize(TILE_SIZE, TILE_SIZE)
        .setDepth(1);

      this.scene.cameras.main.startFollow(this.playerSprite, true);

      this.predictedX = x;
      this.predictedY = y;
      this.initialized = true;
      return;
    }

    if (!this.initialized) {
      this.predictedX = x;
      this.predictedY = y;
      this.playerSprite.setPosition(pixelX, pixelY);
      this.initialized = true;
      return;
    }

    // while lerping, just store the server position — reconcile happens on complete
    if (!this.isLerping) {
      this.playerSprite.setPosition(pixelX, pixelY);
      this.reconcile();
    }
  }

  private reconcile() {
    if (this.serverX === this.predictedX && this.serverY === this.predictedY) {
      return;
    }

    this.predictedX = this.serverX;
    this.predictedY = this.serverY;

    this.playerSprite?.setPosition(tileToPixel(this.serverX), tileToPixel(this.serverY));
  }
}
