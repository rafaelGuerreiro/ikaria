import React, { useState } from 'react'
import { Alert, Button, ButtonGroup, Form, Stack, ToggleButton } from 'react-bootstrap'
import { useReducer } from 'spacetimedb/react'
import { reducers } from '../module_bindings'
import type { CharacterGenderTag, CharacterRaceTag } from './types'

type CharacterCreationViewProps = {
  onBack: () => void
  onCharacterCreated: (displayName: string) => void
}

export function CharacterCreationView({
  onBack,
  onCharacterCreated,
}: CharacterCreationViewProps) {
  const runCreateCharacter = useReducer(reducers.createCharacterV1)

  const [displayName, setDisplayName] = useState('')
  const [gender, setGender] = useState<CharacterGenderTag | null>(null)
  const [race, setRace] = useState<CharacterRaceTag | null>(null)
  const [validated, setValidated] = useState(false)
  const [creatingCharacter, setCreatingCharacter] = useState(false)
  const [statusMessage, setStatusMessage] = useState<string | null>(null)

  const handleSubmit = async (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault()

    const form = event.currentTarget
    if (!form.checkValidity() || !gender || !race) {
      setValidated(true)
      return
    }

    const trimmedName = displayName.trim()
    if (!trimmedName) {
      setValidated(true)
      return
    }

    setCreatingCharacter(true)
    setStatusMessage('Creating character...')

    try {
      await runCreateCharacter({
        displayName: trimmedName,
        gender: { tag: gender },
        race: { tag: race },
      })

      onCharacterCreated(trimmedName)
    } catch (error) {
      const reason = error instanceof Error ? error.message : String(error)
      setStatusMessage(`Failed to create character: ${reason}`)
    } finally {
      setCreatingCharacter(false)
    }
  }

  return (
    <>
      <h2 className="mb-3">Character Creation</h2>

      {statusMessage && (
        <Alert variant={statusMessage.startsWith('Failed') ? 'danger' : 'info'}>
          {statusMessage}
        </Alert>
      )}

      <Stack direction="horizontal" className="mb-3">
        <Button variant="secondary" onClick={onBack}>
          Back to characters
        </Button>
      </Stack>

      <Form noValidate validated={validated} onSubmit={handleSubmit}>
        <Form.Group className="mb-3" controlId="characterName">
          <Form.Label>Name</Form.Label>
          <Form.Control
            type="text"
            placeholder="Character name"
            value={displayName}
            onChange={(e) => setDisplayName(e.target.value)}
            maxLength={20}
            disabled={creatingCharacter}
            required
          />
          <Form.Control.Feedback type="invalid">
            Please enter a character name.
          </Form.Control.Feedback>
        </Form.Group>

        <Form.Group className="mb-3">
          <Form.Label>Gender</Form.Label>
          <div>
            <ButtonGroup>
              <ToggleButton
                id="gender-male"
                type="radio"
                variant={gender === 'Male' ? 'primary' : 'outline-primary'}
                name="gender"
                value="Male"
                checked={gender === 'Male'}
                onChange={() => setGender('Male')}
                disabled={creatingCharacter}
              >
                Male
              </ToggleButton>
              <ToggleButton
                id="gender-female"
                type="radio"
                variant={gender === 'Female' ? 'primary' : 'outline-primary'}
                name="gender"
                value="Female"
                checked={gender === 'Female'}
                onChange={() => setGender('Female')}
                disabled={creatingCharacter}
              >
                Female
              </ToggleButton>
            </ButtonGroup>
            {validated && !gender && (
              <div className="text-danger small mt-1">Please select a gender.</div>
            )}
          </div>
        </Form.Group>

        <Form.Group className="mb-3">
          <Form.Label>Race</Form.Label>
          <div>
            <ButtonGroup>
              <ToggleButton
                id="race-human"
                type="radio"
                variant={race === 'Human' ? 'primary' : 'outline-primary'}
                name="race"
                value="Human"
                checked={race === 'Human'}
                onChange={() => setRace('Human')}
                disabled={creatingCharacter}
              >
                Human
              </ToggleButton>
              <ToggleButton
                id="race-elf"
                type="radio"
                variant={race === 'Elf' ? 'primary' : 'outline-primary'}
                name="race"
                value="Elf"
                checked={race === 'Elf'}
                onChange={() => setRace('Elf')}
                disabled={creatingCharacter}
              >
                Elf
              </ToggleButton>
            </ButtonGroup>
            {validated && !race && (
              <div className="text-danger small mt-1">Please select a race.</div>
            )}
          </div>
        </Form.Group>

        <Button type="submit" disabled={creatingCharacter}>
          {creatingCharacter ? 'Creating...' : 'Create character'}
        </Button>
      </Form>
    </>
  )
}
