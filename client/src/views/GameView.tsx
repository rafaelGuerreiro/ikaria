import Phaser from 'phaser';
import { useEffect, useRef } from 'react';
import { Button, Spinner, Stack } from 'react-bootstrap';
import { useReducer, useTable } from 'spacetimedb/react';
import { GameScene, type Movement, type MapTile, type NearbyPlayer } from '../game/GameScene';
import { reducers, tables } from '../module_bindings';

type GameViewProps = {
  onLeaveGame: () => void;
};

export function GameView({ onLeaveGame }: GameViewProps) {
  const containerRef = useRef<HTMLDivElement>(null);
  const gameRef = useRef<Phaser.Game | null>(null);
  const sceneRef = useRef<GameScene | null>(null);
  const [mapRows] = useTable(tables.vw_world_map_v1);
  const [positions] = useTable(tables.vw_world_my_character_position_v1);
  const [stats] = useTable(tables.vw_character_me_stats_v1);
  const [characterMe] = useTable(tables.vw_character_me_v1);
  const [nearbyCharacters] = useTable(tables.vw_nearby_characters_v1);
  const [nearbyPositions] = useTable(tables.vw_nearby_character_positions_v1);
  const runMoveCharacter = useReducer(reducers.moveCharacterV1);
  const runUnselectCharacter = useReducer(reducers.unselectCharacterV1);

  const handleLeaveGame = async () => {
    try {
      await runUnselectCharacter();
    } catch {
      // best-effort: still navigate back even if unselect fails
    }
    onLeaveGame();
  };

  useEffect(() => {
    if (!containerRef.current) return;

    const scene = new GameScene();
    sceneRef.current = scene;

    const game = new Phaser.Game({
      type: Phaser.AUTO,
      parent: containerRef.current,
      width: window.innerWidth,
      height: window.innerHeight,
      pixelArt: true,
      scene,
      scale: {
        mode: Phaser.Scale.RESIZE,
        autoCenter: Phaser.Scale.CENTER_BOTH,
      },
    });

    gameRef.current = game;

    return () => {
      game.destroy(true);
      gameRef.current = null;
      sceneRef.current = null;
    };
  }, []);

  useEffect(() => {
    const scene = sceneRef.current;
    if (!scene) return;

    const handleMove = (movement: Movement) => {
      runMoveCharacter({ movement: { tag: movement } });
    };

    scene.setMoveCallback(handleMove);
  }, [runMoveCharacter]);

  useEffect(() => {
    if (!sceneRef.current || mapRows.length === 0) return;

    const tiles: MapTile[] = mapRows.flatMap((row) => {
      const result: MapTile[] = [];
      for (let x = row.x1; x <= row.x2; x++) {
        for (let y = row.y1; y <= row.y2; y++) {
          result.push({ x, y, tag: row.tile.tag });
        }
      }
      return result;
    });

    sceneRef.current.updateMap(tiles);
  }, [mapRows]);

  useEffect(() => {
    if (!sceneRef.current || positions.length === 0) return;

    const pos = positions[0];
    sceneRef.current.updatePlayerPosition(pos.x, pos.y);
  }, [positions]);

  useEffect(() => {
    if (!sceneRef.current || stats.length === 0) return;

    sceneRef.current.setSpeed(stats[0].speed);
  }, [stats]);

  useEffect(() => {
    if (!sceneRef.current || characterMe.length === 0) return;

    sceneRef.current.setDisplayName(characterMe[0].displayName);
  }, [characterMe]);

  useEffect(() => {
    if (!sceneRef.current) return;

    const myCharacterId = positions.length > 0 ? positions[0].characterId : null;
    const namesByCharacterId = new Map(
      nearbyCharacters.map((c) => [c.characterId, c.displayName]),
    );

    const players: NearbyPlayer[] = [];
    for (const pos of nearbyPositions) {
      if (pos.characterId === myCharacterId) continue;
      const displayName = namesByCharacterId.get(pos.characterId) ?? '???';
      players.push({ characterId: pos.characterId, x: pos.x, y: pos.y, displayName, arrivesAtMs: Number(pos.arrivesAt.toMillis()) });
    }

    sceneRef.current.updateNearbyPlayers(players);
  }, [nearbyCharacters, nearbyPositions, positions]);

  const isLoading = mapRows.length === 0;

  return (
    <div
      style={{
        width: '100vw',
        height: '100vh',
        overflow: 'hidden',
        background: '#000',
        position: 'relative',
      }}
    >
      {isLoading && (
        <Stack
          className="align-items-center justify-content-center"
          style={{ position: 'absolute', inset: 0, zIndex: 20 }}
        >
          <Spinner animation="border" role="status" className="mb-3" />
          <p className="text-muted">Loading map...</p>
          <Button variant="outline-secondary" size="sm" onClick={handleLeaveGame}>
            Back to characters
          </Button>
        </Stack>
      )}
      <div style={{ position: 'absolute', top: 8, left: 8, zIndex: 10 }}>
        <Button variant="outline-light" size="sm" onClick={handleLeaveGame}>
          Back to characters
        </Button>
      </div>
      <div ref={containerRef} style={{ width: '100%', height: '100%' }} />
    </div>
  );
}
