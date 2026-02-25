export type World = {
  id: string
  name: string
  database: string
  description: string
}

export const SPACETIME_URI = 'https://maincloud.spacetimedb.com'

export const WORLDS: World[] = [
  {
    id: 'alpha',
    name: 'Alpha',
    database: 'world-alpha-ikariadb',
    description: 'The first realm of Ikaria.',
  },
  {
    id: 'draconis',
    name: 'Draconis',
    database: 'world-draconis-ikariadb',
    description: 'The dragon realm.',
  },
]

export function tokenStorageKey(world: World): string {
  return `ikaria.auth.token.${world.id}`
}
