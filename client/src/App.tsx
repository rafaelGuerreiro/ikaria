import { useState } from 'react'
import { Card, Container } from 'react-bootstrap'
import { SpacetimeDBProvider } from 'spacetimedb/react'
import { DbConnection } from './module_bindings'
import type { World } from './worlds'
import { CharacterCreationView } from './views/CharacterCreationView'
import { CharacterListView } from './views/CharacterListView'
import { WorldSelectionView } from './views/WorldSelectionView'
import 'bootstrap/dist/css/bootstrap.min.css'
import './App.css'

type AppView = 'world-selection' | 'character-list' | 'character-creation'
type ConnectionBuilder = ReturnType<typeof DbConnection.builder>

function App() {
  const [currentView, setCurrentView] = useState<AppView>('world-selection')
  const [connectionBuilder, setConnectionBuilder] =
    useState<ConnectionBuilder | null>(null)
  const [world, setWorld] = useState<World | null>(null)

  const handleConnect = (builder: ConnectionBuilder, selectedWorld: World) => {
    setConnectionBuilder(builder)
    setWorld(selectedWorld)
    setCurrentView('character-list')
  }

  const handleLeaveWorld = () => {
    setConnectionBuilder(null)
    setWorld(null)
    setCurrentView('world-selection')
  }

  const handleCreateCharacter = () => {
    setCurrentView('character-creation')
  }

  const handleBack = () => {
    setCurrentView('character-list')
  }

  const handleCharacterCreated = (_displayName: string) => {
    setCurrentView('character-list')
  }

  return (
    <Container className="app">
      <Card className="w-100 p-4">
        <Card.Body>
          {currentView === 'world-selection' || !connectionBuilder || !world ? (
            <WorldSelectionView onConnect={handleConnect} />
          ) : (
            <SpacetimeDBProvider connectionBuilder={connectionBuilder}>
              {currentView === 'character-list' ? (
                <CharacterListView
                  world={world}
                  onCreateCharacter={handleCreateCharacter}
                  onLeaveWorld={handleLeaveWorld}
                />
              ) : (
                <CharacterCreationView
                  onBack={handleBack}
                  onCharacterCreated={handleCharacterCreated}
                />
              )}
            </SpacetimeDBProvider>
          )}
        </Card.Body>
      </Card>
    </Container>
  )
}

export default App
