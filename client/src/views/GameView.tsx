import Phaser from 'phaser';
import { useEffect, useRef } from 'react';
import { Button, Spinner, Stack } from 'react-bootstrap';
import { useReducer, useTable } from 'spacetimedb/react';
import { GameScene, type Movement, type MapTile } from '../game/GameScene';
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
  const runMoveCharacter = useReducer(reducers.moveCharacterV1);

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

    const tiles: MapTile[] = mapRows.map((row) => ({
      x: row.x,
      y: row.y,
      tag: row.tile.tag,
    }));

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
          <Button variant="outline-secondary" size="sm" onClick={onLeaveGame}>
            Back to characters
          </Button>
        </Stack>
      )}
      <div style={{ position: 'absolute', top: 8, left: 8, zIndex: 10 }}>
        <Button variant="outline-light" size="sm" onClick={onLeaveGame}>
          Back to characters
        </Button>
      </div>
      <div ref={containerRef} style={{ width: '100%', height: '100%' }} />
    </div>
  );
}
