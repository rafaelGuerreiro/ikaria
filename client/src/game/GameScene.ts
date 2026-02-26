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

function tileKey(x: number, y: number): string {
  return `${x},${y}`
}

type PlacedTile = {
  tag: string
  image: Phaser.GameObjects.Image
}

export class GameScene extends Phaser.Scene {
  private tiles = new Map<string, PlacedTile>()

  constructor() {
    super({ key: 'GameScene' })
  }

  preload() {
    this.load.image('grass', '/assets/tiny_town/tile_0000.png')
    this.load.image('water', '/assets/tiny_town/tile_0018.png')
  }

  create() {
    this.cameras.main.setBackgroundColor('#000000')
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

      const imageKey = TILE_KEYS[tile.tag] ?? TILE_KEYS.Grass
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

    this.updateCameraBounds()
  }

  private updateCameraBounds() {
    if (this.tiles.size === 0) return

    let minX = Infinity
    let minY = Infinity
    let maxX = -Infinity
    let maxY = -Infinity

    for (const placed of this.tiles.values()) {
      const x = placed.image.x - TILE_SIZE / 2
      const y = placed.image.y - TILE_SIZE / 2
      if (x < minX) minX = x
      if (y < minY) minY = y
      if (x + TILE_SIZE > maxX) maxX = x + TILE_SIZE
      if (y + TILE_SIZE > maxY) maxY = y + TILE_SIZE
    }

    this.cameras.main.setBounds(minX, minY, maxX - minX, maxY - minY)
    this.cameras.main.centerOn(
      minX + (maxX - minX) / 2,
      minY + (maxY - minY) / 2,
    )
  }
}
