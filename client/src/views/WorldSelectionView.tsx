import { useState } from 'react';
import { Card, Stack } from 'react-bootstrap';
import { DbConnection } from '../module_bindings';
import { type World, SPACETIME_URI, WORLDS, tokenStorageKey } from '../worlds';

type ConnectionBuilder = ReturnType<typeof DbConnection.builder>;

type WorldSelectionViewProps = {
  onConnect: (connectionBuilder: ConnectionBuilder, world: World) => void;
};

function readSavedToken(world: World): string | undefined {
  const token = window.localStorage.getItem(tokenStorageKey(world))?.trim();
  return token ? token : undefined;
}

function forgetSavedToken(world: World): void {
  window.localStorage.removeItem(tokenStorageKey(world));
}

export function WorldSelectionView({ onConnect }: WorldSelectionViewProps) {
  const [tokenState, setTokenState] = useState(() =>
    Object.fromEntries(WORLDS.map((w) => [w.id, Boolean(readSavedToken(w))])),
  );

  const handleWorldClick = (world: World) => {
    const token = readSavedToken(world);
    const builder = DbConnection.builder()
      .withUri(SPACETIME_URI)
      .withDatabaseName(world.database)
      .withLightMode(true);

    if (token) {
      builder.withToken(token);
    }

    onConnect(builder, world);
  };

  const handleForgetToken = (event: React.MouseEvent, world: World) => {
    event.stopPropagation();
    forgetSavedToken(world);
    setTokenState((prev) => ({ ...prev, [world.id]: false }));
  };

  return (
    <>
      <h1 className="mb-1">Ikaria</h1>
      <p className="text-muted mb-3">Choose your world</p>

      <Stack gap={3}>
        {WORLDS.map((world) => (
          <Card
            key={world.id}
            className="world-card"
            role="button"
            onClick={() => handleWorldClick(world)}
          >
            <Card.Body>
              <Card.Title>{world.name}</Card.Title>
              <Card.Text className="text-muted">{world.description}</Card.Text>
              {tokenState[world.id] && (
                <small
                  className="text-secondary text-decoration-underline"
                  role="button"
                  tabIndex={0}
                  onClick={(e) => handleForgetToken(e, world)}
                  onKeyDown={(e) => {
                    if (e.key === 'Enter' || e.key === ' ') {
                      handleForgetToken(e as unknown as React.MouseEvent, world);
                    }
                  }}
                >
                  Forget saved session
                </small>
              )}
            </Card.Body>
          </Card>
        ))}
      </Stack>
    </>
  );
}
