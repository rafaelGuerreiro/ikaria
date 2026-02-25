export type CharacterGenderTag = 'Male' | 'Female'

export type CharacterSummary = {
  characterId: bigint
  displayName: string
  genderTag: string
  raceTag: string
}

export type CharacterView = 'selection' | 'creation'
