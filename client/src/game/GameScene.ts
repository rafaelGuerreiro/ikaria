import Phaser from 'phaser';
import { PlayerMovement, tileToPixel, type Movement } from './PlayerMovement';

const TILE_SIZE = 16;
const VISIBLE_TILES = 10; // tiles visible from center in each direction
const VISIBLE_AREA = (VISIBLE_TILES * 2 + 1) * TILE_SIZE; // 21 tiles in world pixels
const TILE_KEYS: Record<string, string> = {
  Grass: 'grass',
  Water: 'water',
};

export type MapTile = {
  x: number;
  y: number;
  tag: string;
};

export type NearbyPlayer = {
  characterId: bigint;
  x: number;
  y: number;
  displayName: string;
  arrivesAtMs: number;
};

export type { Movement };

function tileKey(x: number, y: number): string {
  return `${x},${y}`;
}

type PlacedTile = {
  tag: string;
  image: Phaser.GameObjects.Image;
};

type PlacedPlayer = {
  sprite: Phaser.GameObjects.Image;
  label: Phaser.GameObjects.Text;
  tileX: number;
  tileY: number;
};

export class GameScene extends Phaser.Scene {
  private mapContainer: Phaser.GameObjects.Container | null = null;
  private tiles = new Map<string, PlacedTile>();
  private tileTagsByCoord = new Map<string, string>();
  private nearbyPlayers = new Map<bigint, PlacedPlayer>();
  private cursors: Phaser.Types.Input.Keyboard.CursorKeys | null = null;
  private movement: PlayerMovement | null = null;

  // buffer updates that arrive before textures are loaded
  private ready = false;
  private pendingMap: MapTile[] | null = null;
  private pendingPosition: { x: number; y: number } | null = null;
  private pendingMoveCallback: ((movement: Movement) => void) | null = null;
  private pendingSpeed: number | null = null;
  private pendingDisplayName: string | null = null;
  private pendingNearbyPlayers: NearbyPlayer[] | null = null;

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
    this.updateZoom();
    this.scale.on('resize', () => this.updateZoom());
    this.cursors = this.input.keyboard!.createCursorKeys();

    this.mapContainer = this.add.container(0, 0);

    const maskHalf = VISIBLE_AREA / 2;
    const maskShape = this.make.graphics();
    maskShape.fillStyle(0xffffff);
    maskShape.fillRect(-maskHalf, -maskHalf, VISIBLE_AREA, VISIBLE_AREA);
    this.mapContainer.setMask(maskShape.createGeometryMask());

    this.movement = new PlayerMovement(this, this.mapContainer);
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
    if (this.pendingDisplayName !== null) {
      this.movement.setDisplayName(this.pendingDisplayName);
      this.pendingDisplayName = null;
    }
    if (this.pendingMap) {
      this.updateMap(this.pendingMap);
      this.pendingMap = null;
    }
    if (this.pendingPosition) {
      this.updatePlayerPosition(this.pendingPosition.x, this.pendingPosition.y);
      this.pendingPosition = null;
    }
    if (this.pendingNearbyPlayers) {
      this.updateNearbyPlayers(this.pendingNearbyPlayers);
      this.pendingNearbyPlayers = null;
    }
  }

  private updateZoom() {
    const size = Math.min(this.scale.width, this.scale.height);
    if (size > 0) {
      this.cameras.main.setZoom(size / VISIBLE_AREA);
    }
  }

  update() {
    if (!this.cursors || !this.movement) return;

    const up = this.cursors.up.isDown || this.input.keyboard!.addKey(Phaser.Input.Keyboard.KeyCodes.W).isDown;
    const down = this.cursors.down.isDown || this.input.keyboard!.addKey(Phaser.Input.Keyboard.KeyCodes.S).isDown;
    const left = this.cursors.left.isDown || this.input.keyboard!.addKey(Phaser.Input.Keyboard.KeyCodes.A).isDown;
    const right = this.cursors.right.isDown || this.input.keyboard!.addKey(Phaser.Input.Keyboard.KeyCodes.D).isDown;

    let dir: Movement | null = null;
    if (up && left) dir = 'NorthWest';
    else if (up && right) dir = 'NorthEast';
    else if (down && left) dir = 'SouthWest';
    else if (down && right) dir = 'SouthEast';
    else if (up) dir = 'North';
    else if (down) dir = 'South';
    else if (left) dir = 'West';
    else if (right) dir = 'East';

    if (dir) {
      this.movement.tryMove(dir);
    } else {
      this.movement.clearPending();
    }
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

  setDisplayName(name: string) {
    if (!this.movement) {
      this.pendingDisplayName = name;
      return;
    }
    this.movement.setDisplayName(name);
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

      this.mapContainer!.add(image);
      this.tiles.set(key, { tag: tile.tag, image });
    }

    for (const [key, placed] of this.tiles) {
      if (!incomingKeys.has(key)) {
        placed.image.destroy();
        this.tiles.delete(key);
        this.tileTagsByCoord.delete(key);
      }
    }
  }

  updatePlayerPosition(x: number, y: number) {
    if (!this.ready) {
      this.pendingPosition = { x, y };
      return;
    }

    this.movement?.updateServerPosition(x, y);
  }

  updateNearbyPlayers(players: NearbyPlayer[]) {
    if (!this.ready) {
      this.pendingNearbyPlayers = players;
      return;
    }

    const seen = new Set<bigint>();

    for (const player of players) {
      seen.add(player.characterId);
      const px = tileToPixel(player.x);
      const py = tileToPixel(player.y);

      const existing = this.nearbyPlayers.get(player.characterId);
      if (existing) {
        existing.label.setText(player.displayName);

        if (existing.tileX === player.x && existing.tileY === player.y) continue;

        const dx = Math.abs(player.x - existing.tileX);
        const dy = Math.abs(player.y - existing.tileY);
        existing.tileX = player.x;
        existing.tileY = player.y;

        const remaining = player.arrivesAtMs - Date.now();

        // teleport if moved more than 1 tile, no arrival time, or already arrived
        if (dx > 1 || dy > 1 || remaining <= 0) {
          existing.sprite.setPosition(px, py);
          existing.label.setPosition(px, py - TILE_SIZE);
          continue;
        }

        this.tweens.add({
          targets: existing.sprite,
          x: px,
          y: py,
          duration: remaining,
          ease: 'Linear',
        });
        this.tweens.add({
          targets: existing.label,
          x: px,
          y: py - TILE_SIZE,
          duration: remaining,
          ease: 'Linear',
        });
        continue;
      }

      const sprite = this.add
        .image(px, py, 'player')
        .setDisplaySize(TILE_SIZE, TILE_SIZE)
        .setDepth(1);

      const label = this.add
        .text(px, py - TILE_SIZE, player.displayName, {
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

      this.mapContainer!.add(sprite);
      this.mapContainer!.add(label);
      this.nearbyPlayers.set(player.characterId, { sprite, label, tileX: player.x, tileY: player.y });
    }

    for (const [id, placed] of this.nearbyPlayers) {
      if (seen.has(id)) continue;
      placed.sprite.destroy();
      placed.label.destroy();
      this.nearbyPlayers.delete(id);
    }
  }
}
