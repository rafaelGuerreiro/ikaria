import type { CharacterSummary } from './types'

type CharacterSelectionProps = {
  characters: CharacterSummary[]
  selectedCharacterId: bigint | null
  selectingCharacterId: bigint | null
  statusMessage: string
  isReady: boolean
  identityHex: string | null
  onSelectCharacter: (characterId: bigint, displayName: string) => void
  onOpenCharacterCreation: () => void
  onSignOut: () => void
}

export function CharacterSelection({
  characters,
  selectedCharacterId,
  selectingCharacterId,
  statusMessage,
  isReady,
  identityHex,
  onSelectCharacter,
  onOpenCharacterCreation,
  onSignOut,
}: CharacterSelectionProps) {
  return (
    <>
      <h2 className="section-title">Character Selection</h2>
      <p className="isolation-note">
        Showing only characters scoped to your connected identity in Alpha world.
      </p>
      {identityHex && (
        <p className="identity">
          Identity: <code>{identityHex}</code>
        </p>
      )}
      <p className="character-status">{statusMessage}</p>

      <div className="actions">
        <button
          type="button"
          onClick={onOpenCharacterCreation}
          disabled={!isReady || selectingCharacterId !== null}
        >
          Create new character
        </button>
        <button type="button" className="button-secondary" onClick={onSignOut}>
          Sign out
        </button>
      </div>

      {characters.length === 0 ? (
        <p className="empty-characters">No characters yet.</p>
      ) : (
        <ul className="character-list">
          {characters.map((character) => (
            <li key={character.characterId.toString()} className="character-item">
              <div>
                <strong>{character.displayName}</strong>
                <p className="character-meta">
                  Gender: {character.genderTag} · Race: {character.raceTag}
                </p>
              </div>
              <div className="character-actions">
                {selectedCharacterId === character.characterId && (
                  <span className="selected-badge">Current</span>
                )}
                <button
                  type="button"
                  onClick={() =>
                    onSelectCharacter(character.characterId, character.displayName)
                  }
                  disabled={
                    !isReady ||
                    selectingCharacterId !== null ||
                    selectedCharacterId === character.characterId
                  }
                >
                  {selectingCharacterId === character.characterId
                    ? 'Selecting...'
                    : 'Select'}
                </button>
              </div>
            </li>
          ))}
        </ul>
      )}
    </>
  )
}
