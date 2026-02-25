import { useState } from 'react'
import { SpacetimeDBProvider } from 'spacetimedb/react'
import { CharacterFlow } from './components/CharacterFlow'
import { SignIn } from './components/SignIn'
import { DbConnection } from './module_bindings'
import './App.css'

const SPACETIME_URI = 'https://maincloud.spacetimedb.com'
const ALPHA_WORLD_DATABASE = 'world-alpha-ikariadb'
const TOKEN_STORAGE_KEY = 'ikaria.auth.token'

type AppView = 'sign-in' | 'character-flow'
type AlphaConnectionBuilder = ReturnType<typeof DbConnection.builder>

function readSavedToken(): string | undefined {
  const token = window.localStorage.getItem(TOKEN_STORAGE_KEY)?.trim()
  return token ? token : undefined
}

function App() {
  const [appView, setAppView] = useState<AppView>('sign-in')
  const [connectionBuilder, setConnectionBuilder] =
    useState<AlphaConnectionBuilder | null>(null)
  const [signInMessage, setSignInMessage] = useState(
    'Ready to sign in to Alpha world.',
  )
  const [hasSavedToken, setHasSavedToken] = useState<boolean>(
    () => Boolean(readSavedToken()),
  )

  const startSignIn = () => {
    const token = readSavedToken()
    const builder = DbConnection.builder()
      .withUri(SPACETIME_URI)
      .withDatabaseName(ALPHA_WORLD_DATABASE)
      .withLightMode(true)

    if (token) {
      builder.withToken(token)
      setSignInMessage('Connecting with saved token...')
    } else {
      setSignInMessage('Connecting...')
    }

    setConnectionBuilder(builder)
    setAppView('character-flow')
  }

  const persistToken = (token: string) => {
    if (!token) {
      return
    }

    window.localStorage.setItem(TOKEN_STORAGE_KEY, token)
    setHasSavedToken(true)
  }

  const returnToSignIn = (message: string) => {
    setConnectionBuilder(null)
    setAppView('sign-in')
    setSignInMessage(message)
  }

  const forgetSavedToken = () => {
    window.localStorage.removeItem(TOKEN_STORAGE_KEY)
    setHasSavedToken(false)
    setSignInMessage('Saved token removed.')
  }

  return (
    <main className="app">
      <section className="card">
        {appView === 'sign-in' || !connectionBuilder ? (
          <SignIn
            message={signInMessage}
            hasSavedToken={hasSavedToken}
            worldName="Alpha"
            moduleName={ALPHA_WORLD_DATABASE}
            serverUri={SPACETIME_URI}
            onSignIn={startSignIn}
            onForgetSavedToken={forgetSavedToken}
          />
        ) : (
          <SpacetimeDBProvider connectionBuilder={connectionBuilder}>
            <CharacterFlow
              onPersistToken={persistToken}
              onSignOut={returnToSignIn}
            />
          </SpacetimeDBProvider>
        )}
      </section>
    </main>
  )
}

export default App
