export type CharacterGenderTag = 'Male' | 'Female'
export type CharacterRaceTag = 'Human' | 'Elf'

export type CharacterSummary = {
  characterId: bigint
  displayName: string
  genderTag: string
  raceTag: string
}
