import Phaser from 'phaser';
import { PlayerMovement, tileToPixel, type Movement } from './PlayerMovement';

const TILE_SIZE = 16;
const VISIBLE_RADIUS = 10;

const TILE_KEYS: Record<string, string> = {
  Grass: 'grass',
  Water: 'water',
};

export type MapTile = {
  x: number;
  y: number;
  tag: string;
};

export type { Movement };

function tileKey(x: number, y: number): string {
  return `${x},${y}`;
}

type PlacedTile = {
  tag: string;
  image: Phaser.GameObjects.Image;
};

export class GameScene extends Phaser.Scene {
  private tiles = new Map<string, PlacedTile>();
  private tileTagsByCoord = new Map<string, string>();
  private cursors: Phaser.Types.Input.Keyboard.CursorKeys | null = null;
  private movement: PlayerMovement | null = null;

  // buffer updates that arrive before textures are loaded
  private ready = false;
  private pendingMap: MapTile[] | null = null;
  private pendingPosition: { x: number; y: number } | null = null;
  private pendingMoveCallback: ((movement: Movement) => void) | null = null;
  private pendingSpeed: number | null = null;

  constructor() {
    super({ key: 'GameScene' });
  }

  preload() {
    this.load.image('grass', '/assets/tiny_town/tile_0000.png');
    this.load.image('grass_alt', '/assets/tiny_town/tile_0001.png');
    this.load.image('water', '/assets/tiny_town/tile_0018.png');
    this.load.image('player', '/assets/tiny_dungeon/tile_0085.png');
  }

  create() {
    this.cameras.main.setBackgroundColor('#000000');
    this.cameras.main.setZoom(3);
    this.cursors = this.input.keyboard!.createCursorKeys();

    this.movement = new PlayerMovement(this);
    this.movement.setTileLookup((x, y) => this.tileTagsByCoord.get(tileKey(x, y)));

    this.ready = true;

    if (this.pendingMoveCallback) {
      this.movement.setMoveCallback(this.pendingMoveCallback);
      this.pendingMoveCallback = null;
    }
    if (this.pendingSpeed !== null) {
      this.movement.setSpeed(this.pendingSpeed);
      this.pendingSpeed = null;
    }
    if (this.pendingMap) {
      this.updateMap(this.pendingMap);
      this.pendingMap = null;
    }
    if (this.pendingPosition) {
      this.updatePlayerPosition(this.pendingPosition.x, this.pendingPosition.y);
      this.pendingPosition = null;
    }
  }

  update() {
    if (!this.cursors || !this.movement || this.movement.moving) return;

    const up = this.cursors.up.isDown;
    const down = this.cursors.down.isDown;
    const left = this.cursors.left.isDown;
    const right = this.cursors.right.isDown;

    let dir: Movement | null = null;
    if (up && left) dir = 'NorthWest';
    else if (up && right) dir = 'NorthEast';
    else if (down && left) dir = 'SouthWest';
    else if (down && right) dir = 'SouthEast';
    else if (up) dir = 'North';
    else if (down) dir = 'South';
    else if (left) dir = 'West';
    else if (right) dir = 'East';

    if (dir) this.movement.tryMove(dir);
  }

  setMoveCallback(callback: (movement: Movement) => void) {
    if (!this.movement) {
      this.pendingMoveCallback = callback;
      return;
    }
    this.movement.setMoveCallback(callback);
  }

  setSpeed(speed: number) {
    if (!this.movement) {
      this.pendingSpeed = speed;
      return;
    }
    this.movement.setSpeed(speed);
  }

  updateMap(incoming: MapTile[]) {
    if (!this.ready) {
      this.pendingMap = incoming;
      return;
    }

    const incomingKeys = new Set<string>();

    for (const tile of incoming) {
      const key = tileKey(tile.x, tile.y);
      incomingKeys.add(key);
      this.tileTagsByCoord.set(key, tile.tag);

      const existing = this.tiles.get(key);
      if (existing && existing.tag === tile.tag) continue;

      if (existing) {
        existing.image.destroy();
      }

      let imageKey = TILE_KEYS[tile.tag] ?? TILE_KEYS.Grass;
      if (imageKey === 'grass' && (tile.x + tile.y) % 2 === 1) {
        imageKey = 'grass_alt';
      }

      const image = this.add
        .image(tileToPixel(tile.x), tileToPixel(tile.y), imageKey)
        .setDisplaySize(TILE_SIZE, TILE_SIZE);

      this.tiles.set(key, { tag: tile.tag, image });
    }

    for (const [key, placed] of this.tiles) {
      if (!incomingKeys.has(key)) {
        placed.image.destroy();
        this.tiles.delete(key);
        this.tileTagsByCoord.delete(key);
      }
    }

    this.updateTileVisibility();
  }

  updatePlayerPosition(x: number, y: number) {
    if (!this.ready) {
      this.pendingPosition = { x, y };
      return;
    }

    this.movement?.updateServerPosition(x, y);
  }

  private updateTileVisibility() {
    const center = this.movement?.position ?? { x: 0, y: 0 };

    for (const [key, placed] of this.tiles) {
      const [tx, ty] = key.split(',').map(Number);
      const dx = Math.abs(tx - center.x);
      const dy = Math.abs(ty - center.y);
      placed.image.setVisible(dx <= VISIBLE_RADIUS && dy <= VISIBLE_RADIUS);
    }
  }
}
