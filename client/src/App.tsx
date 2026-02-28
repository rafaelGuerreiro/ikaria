import { useState } from 'react';
import { Card, Container } from 'react-bootstrap';
import { SpacetimeDBProvider } from 'spacetimedb/react';
import { DbConnection } from './module_bindings';
import { SubscriptionProvider } from './hooks/SubscriptionProvider';
import type { World } from './worlds';
import { CharacterCreationView } from './views/CharacterCreationView';
import { CharacterListView } from './views/CharacterListView';
import { GameView } from './views/GameView';
import { WorldSelectionView } from './views/WorldSelectionView';
import './App.css';

type AppView = 'world-selection' | 'character-list' | 'character-creation' | 'game';
type ConnectionBuilder = ReturnType<typeof DbConnection.builder>;

function App() {
  const [currentView, setCurrentView] = useState<AppView>('world-selection');
  const [connectionBuilder, setConnectionBuilder] = useState<ConnectionBuilder | null>(null);
  const [world, setWorld] = useState<World | null>(null);

  const handleConnect = (builder: ConnectionBuilder, selectedWorld: World) => {
    setConnectionBuilder(builder);
    setWorld(selectedWorld);
    setCurrentView('character-list');
  };

  const handleLeaveWorld = () => {
    setConnectionBuilder(null);
    setWorld(null);
    setCurrentView('world-selection');
  };

  const handleCreateCharacter = () => {
    setCurrentView('character-creation');
  };

  const handleBack = () => {
    setCurrentView('character-list');
  };

  const handleCharacterCreated = (_displayName: string) => {
    setCurrentView('character-list');
  };

  const handleEnterGame = () => {
    setCurrentView('game');
  };

  const handleLeaveGame = () => {
    setCurrentView('character-list');
  };

  if (currentView === 'game' && connectionBuilder && world) {
    return (
      <SpacetimeDBProvider connectionBuilder={connectionBuilder}>
        <SubscriptionProvider>
          <GameView onLeaveGame={handleLeaveGame} />
        </SubscriptionProvider>
      </SpacetimeDBProvider>
    );
  }

  return (
    <Container className="app">
      <Card className="w-100 p-4">
        <Card.Body>
          {currentView === 'world-selection' || !connectionBuilder || !world ? (
            <WorldSelectionView onConnect={handleConnect} />
          ) : (
            <SpacetimeDBProvider connectionBuilder={connectionBuilder}>
              <SubscriptionProvider>
                {currentView === 'character-list' ? (
                  <CharacterListView
                    world={world}
                    onCreateCharacter={handleCreateCharacter}
                    onLeaveWorld={handleLeaveWorld}
                    onEnterGame={handleEnterGame}
                  />
                ) : (
                  <CharacterCreationView
                    onBack={handleBack}
                    onCharacterCreated={handleCharacterCreated}
                  />
                )}
              </SubscriptionProvider>
            </SpacetimeDBProvider>
          )}
        </Card.Body>
      </Card>
    </Container>
  );
}

export default App;
