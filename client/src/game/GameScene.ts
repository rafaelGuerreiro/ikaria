import Phaser from 'phaser'

const TILE_SIZE = 16

const TILE_KEYS: Record<string, string> = {
  Grass: 'grass',
  Water: 'water',
}

export type MapTile = {
  x: number
  y: number
  tag: string
}

export type Movement =
  | 'North'
  | 'NorthEast'
  | 'East'
  | 'SouthEast'
  | 'South'
  | 'SouthWest'
  | 'West'
  | 'NorthWest'

function tileKey(x: number, y: number): string {
  return `${x},${y}`
}

type PlacedTile = {
  tag: string
  image: Phaser.GameObjects.Image
}

export class GameScene extends Phaser.Scene {
  private tiles = new Map<string, PlacedTile>()
  private playerSprite: Phaser.GameObjects.Image | null = null
  private cursors: Phaser.Types.Input.Keyboard.CursorKeys | null = null
  private moveCallback: ((movement: Movement) => void) | null = null
  private keyLock = false

  constructor() {
    super({ key: 'GameScene' })
  }

  preload() {
    this.load.image('grass', '/assets/tiny_town/tile_0000.png')
    this.load.image('grass_alt', '/assets/tiny_town/tile_0001.png')
    this.load.image('water', '/assets/tiny_town/tile_0018.png')
    this.load.image('player', '/assets/tiny_dungeon/tile_0085.png')
  }

  create() {
    this.cameras.main.setBackgroundColor('#000000')
    this.cursors = this.input.keyboard!.createCursorKeys()
  }

  update() {
    if (!this.cursors || this.keyLock) return

    const up = this.cursors.up.isDown
    const down = this.cursors.down.isDown
    const left = this.cursors.left.isDown
    const right = this.cursors.right.isDown

    let movement: Movement | null = null
    if (up && left) movement = 'NorthWest'
    else if (up && right) movement = 'NorthEast'
    else if (down && left) movement = 'SouthWest'
    else if (down && right) movement = 'SouthEast'
    else if (up) movement = 'North'
    else if (down) movement = 'South'
    else if (left) movement = 'West'
    else if (right) movement = 'East'

    if (movement && this.moveCallback) {
      this.keyLock = true
      this.moveCallback(movement)
      this.time.delayedCall(150, () => {
        this.keyLock = false
      })
    }
  }

  setMoveCallback(callback: (movement: Movement) => void) {
    this.moveCallback = callback
  }

  updateMap(incoming: MapTile[]) {
    const incomingKeys = new Set<string>()

    for (const tile of incoming) {
      const key = tileKey(tile.x, tile.y)
      incomingKeys.add(key)

      const existing = this.tiles.get(key)

      // tile unchanged — skip
      if (existing && existing.tag === tile.tag) continue

      // tile changed — destroy old sprite
      if (existing) {
        existing.image.destroy()
      }

      let imageKey = TILE_KEYS[tile.tag] ?? TILE_KEYS.Grass
      if (imageKey === 'grass' && (tile.x + tile.y) % 2 === 1) {
        imageKey = 'grass_alt'
      }

      const image = this.add
        .image(
          tile.x * TILE_SIZE + TILE_SIZE / 2,
          tile.y * TILE_SIZE + TILE_SIZE / 2,
          imageKey,
        )
        .setDisplaySize(TILE_SIZE, TILE_SIZE)

      this.tiles.set(key, { tag: tile.tag, image })
    }

    // remove tiles no longer in the view
    for (const [key, placed] of this.tiles) {
      if (!incomingKeys.has(key)) {
        placed.image.destroy()
        this.tiles.delete(key)
      }
    }
  }

  updatePlayerPosition(x: number, y: number) {
    const pixelX = x * TILE_SIZE + TILE_SIZE / 2
    const pixelY = y * TILE_SIZE + TILE_SIZE / 2

    if (!this.playerSprite) {
      this.playerSprite = this.add
        .image(pixelX, pixelY, 'player')
        .setDisplaySize(TILE_SIZE, TILE_SIZE)
        .setDepth(1)
    } else {
      this.playerSprite.setPosition(pixelX, pixelY)
    }

    this.cameras.main.startFollow(this.playerSprite, true)
  }
}
