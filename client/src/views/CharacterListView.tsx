import { useEffect, useMemo, useState } from 'react';
import { Alert, Badge, Button, ListGroup, Spinner, Stack } from 'react-bootstrap';
import { useReducer, useSpacetimeDB } from 'spacetimedb/react';
import { reducers, tables } from '../module_bindings';
import { useLocalTable } from '../hooks/useLocalTable';
import { useSubscriptionReady } from '../hooks/useSubscriptionReady';
import { type World, tokenStorageKey } from '../worlds';
import type { CharacterSummary } from './types';
import type { CharacterV1 } from '../module_bindings/types';

type CharacterListViewProps = {
  world: World;
  onCreateCharacter: () => void;
  onLeaveWorld: () => void;
  onEnterGame: () => void;
};

export function CharacterListView({
  world,
  onCreateCharacter,
  onLeaveWorld,
  onEnterGame,
}: CharacterListViewProps) {
  const { getConnection, token } = useSpacetimeDB();
  const isReady = useSubscriptionReady();
  const characterRows: CharacterV1[] = useLocalTable(tables.vw_character_all_mine_v1);
  const selectedRows: CharacterV1[] = useLocalTable(tables.vw_character_me_v1);
  const runSelectCharacter = useReducer(reducers.selectCharacterV1);

  const [statusMessage, setStatusMessage] = useState(`Welcome to ${world.name}.`);
  const [selectingCharacterId, setSelectingCharacterId] = useState<bigint | null>(null);

  const characters = useMemo<CharacterSummary[]>(
    () =>
      [...characterRows]
        .map((character) => ({
          characterId: character.characterId,
          displayName: character.displayName,
          genderTag: character.gender.tag,
          raceTag: character.race.tag,
        }))
        .sort((left, right) =>
          left.characterId < right.characterId ? -1 : left.characterId > right.characterId ? 1 : 0,
        ),
    [characterRows],
  );

  const selectedCharacterId = selectedRows[0]?.characterId ?? null;

  useEffect(() => {
    if (selectedCharacterId !== null) {
      onEnterGame();
    }
  }, [selectedCharacterId, onEnterGame]);

  useEffect(() => {
    if (token) {
      window.localStorage.setItem(tokenStorageKey(world), token);
    }
  }, [token, world]);

  const selectCharacter = async (characterId: bigint, displayName: string) => {
    setSelectingCharacterId(characterId);
    setStatusMessage(`Selecting '${displayName}'...`);

    try {
      await runSelectCharacter({ characterId });
      setStatusMessage(`Selected '${displayName}'.`);
    } catch (error) {
      const reason = error instanceof Error ? error.message : String(error);
      setStatusMessage(`Selection failed: ${reason}`);
    } finally {
      setSelectingCharacterId(null);
    }
  };

  const handleLeaveWorld = () => {
    getConnection()?.disconnect();
    onLeaveWorld();
  };

  return (
    <>
      <h2 className="mb-1">Your Characters</h2>
      <p className="text-muted mb-2">Your characters in {world.name}</p>

      <Alert variant="info" className="mb-3">
        {statusMessage}
      </Alert>

      <Stack direction="horizontal" gap={2} className="mb-3">
        <Button onClick={onCreateCharacter} disabled={!isReady || selectingCharacterId !== null}>
          Create new character
        </Button>
        <Button variant="secondary" onClick={handleLeaveWorld}>
          Leave world
        </Button>
      </Stack>

      {!isReady ? (
        <Stack direction="horizontal" gap={2} className="text-muted">
          <Spinner animation="border" size="sm" />
          <span>Loading characters...</span>
        </Stack>
      ) : characters.length === 0 ? (
        <p className="text-muted">No characters yet.</p>
      ) : (
        <ListGroup>
          {characters.map((character) => (
            <ListGroup.Item
              key={character.characterId.toString()}
              className="d-flex justify-content-between align-items-center"
            >
              <div>
                <strong>{character.displayName}</strong>
                <br />
                <small className="text-muted">
                  Gender: {character.genderTag} · Race: {character.raceTag}
                </small>
              </div>
              <Stack direction="horizontal" gap={2}>
                {selectedCharacterId === character.characterId ? (
                  <>
                    <Badge bg="success">Current</Badge>
                    <Button size="sm" variant="success" onClick={onEnterGame}>
                      Play
                    </Button>
                  </>
                ) : (
                  <Button
                    size="sm"
                    onClick={() => selectCharacter(character.characterId, character.displayName)}
                    disabled={selectingCharacterId !== null}
                  >
                    {selectingCharacterId === character.characterId ? 'Selecting...' : 'Select'}
                  </Button>
                )}
              </Stack>
            </ListGroup.Item>
          ))}
        </ListGroup>
      )}
    </>
  );
}
