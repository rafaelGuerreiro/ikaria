import { useEffect, useMemo, useState } from 'react'
import { useReducer, useSpacetimeDB, useTable } from 'spacetimedb/react'
import { reducers, tables } from '../module_bindings'
import { CharacterCreation } from './CharacterCreation'
import { CharacterSelection } from './CharacterSelection'
import type { CharacterGenderTag, CharacterSummary, CharacterView } from './types'

type CharacterFlowProps = {
  onPersistToken: (token: string) => void
  onSignOut: (message: string) => void
}

export function CharacterFlow({ onPersistToken, onSignOut }: CharacterFlowProps) {
  const { connectionError, getConnection, identity, isActive, token } = useSpacetimeDB()
  const [characterRows, charactersReady] = useTable(tables.vw_character_all_mine_v1)
  const [selectedRows, selectedReady] = useTable(tables.vw_character_me_v1)
  const runCreateCharacter = useReducer(reducers.createCharacterV1)
  const runSelectCharacter = useReducer(reducers.selectCharacterV1)

  const [view, setView] = useState<CharacterView>('selection')
  const [statusMessage, setStatusMessage] = useState('Connecting to Alpha world...')
  const [createDisplayName, setCreateDisplayName] = useState('')
  const [createGender, setCreateGender] = useState<CharacterGenderTag>('Male')
  const [creatingCharacter, setCreatingCharacter] = useState(false)
  const [selectingCharacterId, setSelectingCharacterId] = useState<bigint | null>(
    null,
  )

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
          left.characterId < right.characterId
            ? -1
            : left.characterId > right.characterId
              ? 1
              : 0,
        ),
    [characterRows],
  )

  const selectedCharacterId = selectedRows[0]?.characterId ?? null
  const identityHex = identity?.toHexString() ?? null
  const isReady = isActive && charactersReady && selectedReady && !connectionError

  useEffect(() => {
    if (token) {
      onPersistToken(token)
    }
  }, [token, onPersistToken])

  useEffect(() => {
    if (connectionError) {
      setStatusMessage(`Connection error: ${connectionError.message}`)
      return
    }

    if (!isActive) {
      setStatusMessage('Connecting to Alpha world...')
      return
    }

    if (!charactersReady || !selectedReady) {
      setStatusMessage('Synchronizing character tables...')
      return
    }

    setStatusMessage('Character tables synchronized.')
  }, [charactersReady, connectionError, isActive, selectedReady])

  const openCharacterCreation = () => {
    setView('creation')
    setStatusMessage('Fill in the form to create a character.')
  }

  const backToSelection = () => {
    setView('selection')
  }

  const handleSignOut = () => {
    getConnection()?.disconnect()
    onSignOut('Signed out from Alpha world.')
  }

  const createCharacter = async () => {
    if (!isReady) {
      setStatusMessage('Connection is not ready yet.')
      return
    }

    const displayName = createDisplayName.trim()
    if (!displayName) {
      setStatusMessage('Character name is required.')
      return
    }

    setCreatingCharacter(true)
    setStatusMessage('Creating character...')

    try {
      await runCreateCharacter({
        displayName,
        gender: { tag: createGender },
        race: { tag: 'Human' },
      })

      setCreateDisplayName('')
      setView('selection')
      setStatusMessage(`Character '${displayName}' created.`)
    } catch (error) {
      const reason = error instanceof Error ? error.message : String(error)
      setStatusMessage(`Character creation failed: ${reason}`)
    } finally {
      setCreatingCharacter(false)
    }
  }

  const selectCharacter = async (characterId: bigint, displayName: string) => {
    if (!isReady) {
      setStatusMessage('Connection is not ready yet.')
      return
    }

    setSelectingCharacterId(characterId)
    setStatusMessage(`Selecting '${displayName}'...`)

    try {
      await runSelectCharacter({ characterId })
      setStatusMessage(`Selected '${displayName}'.`)
    } catch (error) {
      const reason = error instanceof Error ? error.message : String(error)
      setStatusMessage(`Character selection failed: ${reason}`)
    } finally {
      setSelectingCharacterId(null)
    }
  }

  if (view === 'creation') {
    return (
      <section className="section">
        <CharacterCreation
          displayName={createDisplayName}
          gender={createGender}
          statusMessage={statusMessage}
          creatingCharacter={creatingCharacter}
          disabled={creatingCharacter || selectingCharacterId !== null}
          onDisplayNameChange={setCreateDisplayName}
          onGenderChange={setCreateGender}
          onCreateCharacter={createCharacter}
          onBack={backToSelection}
        />
      </section>
    )
  }

  return (
    <section className="section">
      <CharacterSelection
        characters={characters}
        selectedCharacterId={selectedCharacterId}
        selectingCharacterId={selectingCharacterId}
        statusMessage={statusMessage}
        isReady={isReady}
        identityHex={identityHex}
        onSelectCharacter={selectCharacter}
        onOpenCharacterCreation={openCharacterCreation}
        onSignOut={handleSignOut}
      />
    </section>
  )
}
