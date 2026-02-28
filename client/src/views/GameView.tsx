import Phaser from 'phaser';
import { useCallback, useEffect, useRef } from 'react';
import { Button, Spinner, Stack } from 'react-bootstrap';
import { useReducer } from 'spacetimedb/react';
import { GameScene, type Movement, type MapTile, type NearbyPlayer } from '../game/GameScene';
import { reducers, tables } from '../module_bindings';
import { useLocalTable } from '../hooks/useLocalTable';
import './GameLayout.css';
import type { CharacterPositionV1, CharacterStatsV1, CharacterV1, MapV1 } from '../module_bindings/types';

const DEFAULT_BOTTOM_HEIGHT = 200;
const MIN_BOTTOM_HEIGHT = 100;
const MAX_BOTTOM_HEIGHT = 500;

type GameViewProps = {
  onLeaveGame: () => void;
};

export function GameView({ onLeaveGame }: GameViewProps) {
  const gameCenterWrapperRef = useRef<HTMLDivElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const bottomPanelRef = useRef<HTMLDivElement>(null);
  const gameRef = useRef<Phaser.Game | null>(null);
  const sceneRef = useRef<GameScene | null>(null);
  const isDraggingRef = useRef(false);
  const dragStartYRef = useRef(0);
  const dragStartHeightRef = useRef(0);
  const bottomHeightRef = useRef(DEFAULT_BOTTOM_HEIGHT);
  const mapRows: MapV1[] = useLocalTable(tables.vw_world_map_v1);
  const positions: CharacterPositionV1[] = useLocalTable(tables.vw_world_my_character_position_v1);
  const stats: CharacterStatsV1[] = useLocalTable(tables.vw_character_me_stats_v1);
  const characterMe: CharacterV1[] = useLocalTable(tables.vw_character_me_v1);
  const nearbyCharacters: CharacterV1[] = useLocalTable(tables.vw_nearby_characters_v1);
  const nearbyPositions: CharacterPositionV1[] = useLocalTable(tables.vw_nearby_character_positions_v1);
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

  // Keep game container square based on available space
  useEffect(() => {
    const wrapper = gameCenterWrapperRef.current;
    const container = containerRef.current;
    if (!wrapper || !container) return;

    const updateSize = () => {
      const { width, height } = wrapper.getBoundingClientRect();
      const size = Math.floor(Math.min(width, height));
      if (size <= 0) return;
      container.style.width = `${size}px`;
      container.style.height = `${size}px`;
      if (gameRef.current) {
        gameRef.current.scale.resize(size, size);
      }
    };

    updateSize();
    const observer = new ResizeObserver(updateSize);
    observer.observe(wrapper);
    return () => observer.disconnect();
  }, []);

  // Initialize Phaser
  useEffect(() => {
    if (!containerRef.current) return;

    const rect = containerRef.current.getBoundingClientRect();
    const size = Math.max(Math.floor(Math.min(rect.width, rect.height)), 100);

    const scene = new GameScene();
    sceneRef.current = scene;

    const game = new Phaser.Game({
      type: Phaser.AUTO,
      parent: containerRef.current,
      width: size,
      height: size,
      scene,
      scale: {
        mode: Phaser.Scale.NONE,
      },
    });

    gameRef.current = game;

    return () => {
      game.destroy(true);
      gameRef.current = null;
      sceneRef.current = null;
    };
  }, []);

  // Bottom panel resize drag
  useEffect(() => {
    const handleMouseMove = (e: MouseEvent) => {
      if (!isDraggingRef.current) return;
      e.preventDefault();
      const delta = dragStartYRef.current - e.clientY;
      const newHeight = Math.max(
        MIN_BOTTOM_HEIGHT,
        Math.min(MAX_BOTTOM_HEIGHT, dragStartHeightRef.current + delta),
      );
      bottomHeightRef.current = newHeight;
      if (bottomPanelRef.current) {
        bottomPanelRef.current.style.height = `${newHeight}px`;
      }
    };

    const handleMouseUp = () => {
      if (isDraggingRef.current) {
        isDraggingRef.current = false;
        document.body.style.cursor = '';
      }
    };

    window.addEventListener('mousemove', handleMouseMove);
    window.addEventListener('mouseup', handleMouseUp);
    return () => {
      window.removeEventListener('mousemove', handleMouseMove);
      window.removeEventListener('mouseup', handleMouseUp);
    };
  }, []);

  const handleResizeStart = useCallback((e: React.MouseEvent) => {
    e.preventDefault();
    isDraggingRef.current = true;
    dragStartYRef.current = e.clientY;
    dragStartHeightRef.current = bottomHeightRef.current;
    document.body.style.cursor = 'ns-resize';
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
    <div className="game-layout">
      <div className="panel-left" />

      <div className="game-middle-column">
        <div className="panel-top">
          <Button variant="outline-light" size="sm" onClick={handleLeaveGame}>
            ← Back
          </Button>
          <span className="panel-top-title">Ikaria</span>
        </div>

        <div className="game-center-wrapper" ref={gameCenterWrapperRef}>
          <div className="game-center" ref={containerRef}>
            {isLoading && (
              <Stack
                className="align-items-center justify-content-center"
                style={{ position: 'absolute', inset: 0, zIndex: 20, background: '#000' }}
              >
                <Spinner animation="border" role="status" className="mb-3" />
                <p className="text-muted">Loading map...</p>
              </Stack>
            )}
          </div>
        </div>

        <div
          className="panel-bottom"
          ref={bottomPanelRef}
          style={{ height: DEFAULT_BOTTOM_HEIGHT }}
        >
          <div className="resize-handle" onMouseDown={handleResizeStart} />
        </div>
      </div>

      <div className="panel-right" />
    </div>
  );
}
