import { useMemo } from 'react'
import { Button, Spinner, Stack } from 'react-bootstrap'
import { useTable } from 'spacetimedb/react'
import { tables } from '../module_bindings'

const TILE_SIZE = 16

const TILE_IMAGES: Record<string, string> = {
  Grass: '/assets/tiny_town/tile_0000.png',
  Water: '/assets/tiny_town/tile_0018.png',
}

type GameViewProps = {
  onLeaveGame: () => void
}

export function GameView({ onLeaveGame }: GameViewProps) {
  const [mapRows] = useTable(tables.vw_world_map_v1)

  const { grid, width, height } = useMemo(() => {
    if (mapRows.length === 0) {
      return { grid: [], width: 0, height: 0 }
    }

    let maxX = 0
    let maxY = 0
    for (const row of mapRows) {
      if (row.x > maxX) maxX = row.x
      if (row.y > maxY) maxY = row.y
    }

    const w = maxX + 1
    const h = maxY + 1

    const tiles: (string | null)[][] = Array.from({ length: h }, () =>
      Array.from<string | null>({ length: w }).fill(null),
    )

    for (const row of mapRows) {
      tiles[row.y][row.x] = row.tile.tag
    }

    return { grid: tiles, width: w, height: h }
  }, [mapRows])

  if (mapRows.length === 0) {
    return (
      <Stack
        className="align-items-center justify-content-center"
        style={{ minHeight: '100vh' }}
      >
        <Spinner animation="border" role="status" className="mb-3" />
        <p className="text-muted">Loading map...</p>
        <Button variant="outline-secondary" size="sm" onClick={onLeaveGame}>
          Back to characters
        </Button>
      </Stack>
    )
  }

  return (
    <div style={{ width: '100vw', height: '100vh', overflow: 'hidden', background: '#000', position: 'relative' }}>
      <div style={{ position: 'absolute', top: 8, left: 8, zIndex: 10 }}>
        <Button variant="outline-light" size="sm" onClick={onLeaveGame}>
          Back to characters
        </Button>
      </div>

      <div
        style={{
          width: '100%',
          height: '100%',
          overflow: 'auto',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
        }}
      >
        <div
          style={{
            display: 'grid',
            gridTemplateColumns: `repeat(${width}, ${TILE_SIZE}px)`,
            gridTemplateRows: `repeat(${height}, ${TILE_SIZE}px)`,
            width: width * TILE_SIZE,
            height: height * TILE_SIZE,
          }}
        >
          {grid.flatMap((row, y) =>
            row.map((tileTag, x) => {
              const src = TILE_IMAGES[tileTag ?? 'Grass'] ?? TILE_IMAGES.Grass
              return (
                <img
                  key={`${x}-${y}`}
                  src={src}
                  alt={tileTag ?? 'unknown'}
                  width={TILE_SIZE}
                  height={TILE_SIZE}
                  style={{ display: 'block', imageRendering: 'pixelated' }}
                />
              )
            }),
          )}
        </div>
      </div>
    </div>
  )
}
