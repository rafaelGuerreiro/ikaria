import Phaser from 'phaser';

const TILE_SIZE = 16;
const SPEED_TO_MS = 40_000;
// slightly slower than the server cooldown so held keys produce smooth continuous movement
const ANIMATION_SLOWDOWN = 0.99;

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
  private mapContainer: Phaser.GameObjects.Container;
  private playerSprite: Phaser.GameObjects.Image | null = null;
  private playerLabel: Phaser.GameObjects.Text | null = null;
  private displayName: string | null = null;
  private moveCallback: ((movement: Movement) => void) | null = null;
  private tileLookup: TileLookup | null = null;

  private predictedX = 0;
  private predictedY = 0;
  private serverX = 0;
  private serverY = 0;
  private isLerping = false;
  private pendingMovement: Movement | null = null;
  private speed = 120;
  private initialized = false;

  constructor(scene: Phaser.Scene, mapContainer: Phaser.GameObjects.Container) {
    this.scene = scene;
    this.mapContainer = mapContainer;
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

  setDisplayName(name: string) {
    this.displayName = name;
    if (this.playerLabel) {
      this.playerLabel.setText(name);
    }
  }

  get moving(): boolean {
    return this.isLerping;
  }

  get position(): { x: number; y: number } {
    return { x: this.predictedX, y: this.predictedY };
  }

  clearPending() {
    this.pendingMovement = null;
  }

  tryMove(movement: Movement) {
    if (!this.moveCallback || !this.playerSprite) return;

    if (this.isLerping) {
      this.pendingMovement = movement;
      return;
    }

    const delta = MOVEMENT_DELTA[movement];
    const targetX = this.predictedX + delta.dx;
    const targetY = this.predictedY + delta.dy;

    if (this.tileLookup) {
      const tag = this.tileLookup(targetX, targetY);
      if (tag === 'Water') return;
    }

    this.predictedX = targetX;
    this.predictedY = targetY;

    const isDiagonal = delta.dx !== 0 && delta.dy !== 0;
    const duration = (SPEED_TO_MS / this.speed) * ANIMATION_SLOWDOWN * (isDiagonal ? Math.SQRT2 : 1);
    this.isLerping = true;

    this.scene.tweens.add({
      targets: this.mapContainer,
      x: -tileToPixel(targetX),
      y: -tileToPixel(targetY),
      duration,
      ease: 'Linear',
      onComplete: () => {
        this.isLerping = false;
        this.reconcile();
        if (this.pendingMovement) {
          const next = this.pendingMovement;
          this.pendingMovement = null;
          this.tryMove(next);
        }
      },
    });

    this.moveCallback(movement);
  }

  updateServerPosition(x: number, y: number) {
    this.serverX = x;
    this.serverY = y;

    if (!this.playerSprite) {
      this.playerSprite = this.scene.add
        .image(0, 0, 'player')
        .setDisplaySize(TILE_SIZE, TILE_SIZE)
        .setDepth(1);

      this.playerLabel = this.scene.add
        .text(0, -TILE_SIZE, this.displayName ?? '', {
          fontSize: '7px',
          fontFamily: 'Roboto, sans-serif',
          fontStyle: '900',
          color: '#ffffff',
          stroke: '#000000',
          strokeThickness: 2,
          align: 'center',
          resolution: 4,
        })
        .setOrigin(0.5, 1)
        .setDepth(2);

      this.scene.cameras.main.startFollow(this.playerSprite, true);

      this.predictedX = x;
      this.predictedY = y;
      this.mapContainer.setPosition(-tileToPixel(x), -tileToPixel(y));
      this.initialized = true;
      return;
    }

    if (!this.initialized) {
      this.predictedX = x;
      this.predictedY = y;
      this.mapContainer.setPosition(-tileToPixel(x), -tileToPixel(y));
      this.initialized = true;
      return;
    }

    // while lerping, just store the server position — reconcile happens on complete
    if (!this.isLerping) {
      this.reconcile();
    }
  }

  private reconcile() {
    if (this.serverX === this.predictedX && this.serverY === this.predictedY) {
      return;
    }

    this.predictedX = this.serverX;
    this.predictedY = this.serverY;

    this.mapContainer.setPosition(-tileToPixel(this.serverX), -tileToPixel(this.serverY));
  }
}
