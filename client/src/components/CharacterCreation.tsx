import type { CharacterGenderTag } from './types'

type CharacterCreationProps = {
  displayName: string
  gender: CharacterGenderTag
  statusMessage: string
  creatingCharacter: boolean
  disabled: boolean
  onDisplayNameChange: (value: string) => void
  onGenderChange: (gender: CharacterGenderTag) => void
  onCreateCharacter: () => void
  onBack: () => void
}

export function CharacterCreation({
  displayName,
  gender,
  statusMessage,
  creatingCharacter,
  disabled,
  onDisplayNameChange,
  onGenderChange,
  onCreateCharacter,
  onBack,
}: CharacterCreationProps) {
  return (
    <>
      <h2 className="section-title">Character Creation</h2>
      <p className="character-status">{statusMessage}</p>
      <div className="actions">
        <button type="button" className="button-secondary" onClick={onBack}>
          Back to selection
        </button>
      </div>
      <div className="create-form">
        <label className="field">
          Name
          <input
            type="text"
            value={displayName}
            onChange={(event) => onDisplayNameChange(event.target.value)}
            placeholder="Character name"
            maxLength={20}
            disabled={disabled}
          />
        </label>

        <fieldset className="field">
          <legend>Gender</legend>
          <label className="radio-option">
            <input
              type="radio"
              name="gender"
              checked={gender === 'Male'}
              onChange={() => onGenderChange('Male')}
              disabled={disabled}
            />
            Male
          </label>
          <label className="radio-option">
            <input
              type="radio"
              name="gender"
              checked={gender === 'Female'}
              onChange={() => onGenderChange('Female')}
              disabled={disabled}
            />
            Female
          </label>
        </fieldset>

        <div className="actions">
          <button type="button" onClick={onCreateCharacter} disabled={disabled}>
            {creatingCharacter ? 'Creating...' : 'Create character'}
          </button>
        </div>
      </div>
    </>
  )
}
