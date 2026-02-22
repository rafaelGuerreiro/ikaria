# Ikaria Client Game Structure

## Goal
Plan the first playable client loop for a Tibia-inspired 2D game: sign in, select/create character, and walk in a grassland map.
- UI presentation mode is light mode.

## Core world rules
- The game is 2D and tile-based.
- The player character remains centered on screen.
- Target visible area is `11` tiles in each direction from the player (`23x23` including center tile).
- Start with a simple square grassland map using Kenney spritesheet assets.

## App flow (Bevy states)
`SignIn -> CharacterSelect -> Game`

### State transition rules
- Represent top-level flow as `AppState` and switch views via `NextState<AppState>`.
- Keep each view in its own plugin with co-located `OnEnter` setup, `Update` systems gated by `run_if(in_state(...))`, and `OnExit` cleanup.
- Add `SubStates` only when a view needs internal phases (example: `GameLoading` -> `GamePlaying`).
- Connect against the development module (`ikariadb-dev`) for client development.

### Connection and tick policy
- Use `frame_tick()` as the connection advancement strategy.
- Do not process server ticks on the initial screen before connection; start ticking from post-connection views.

### 1) SignIn view
**Purpose**
- Establish player identity before showing characters.

**Responsibilities**
- On enter, check for a saved token file.
- If a token exists, attempt token-based sign-in automatically.
- If token auth fails or file is missing, keep player in sign-in UI flow.

**Outputs**
- Authenticated session/token resource.
- Transition to `CharacterSelect` on successful sign-in.

### 2) CharacterSelect view
**Purpose**
- Let authenticated players choose how to enter the world.

**Responsibilities**
- Show existing characters for selection.
- Provide character creation with only:
  - `name`
  - `gender`

**Outputs**
- Selected or newly created character stored as active character resource.
- Transition to `Game` when a character is confirmed.

### 3) Game view
**Purpose**
- Render the map and allow player movement.

**Responsibilities**
- Spawn/initialize local player from active character.
- Load grassland tile visuals from Kenney spritesheet.
- Keep camera centered on local player.
- Process movement input from keyboard and map clicks.

## View interaction contracts
- `SignIn` owns auth token discovery/validation and writes session data used by later views.
- `CharacterSelect` requires an authenticated session and writes active character identity.
- `Game` requires both authenticated session and active character to initialize world/player state.
- State exits should despawn view-specific UI/entities so each screen is isolated.
- Prefer Bevy state-scoped cleanup (`DespawnOnExit`/`DespawnOnEnter` or equivalent state-scoped entity setup); use marker-component cleanup systems when teardown needs custom logic.

## Cross-view data and communication
- Keep long-lived cross-view data in resources (`SessionResource`, `SelectedCharacterResource`, local player reference).
- Keep screen-local entities/components owned by the corresponding state/view plugin.
- Use events for cross-feature communication (auth success, character confirmed, move target requested) to reduce plugin coupling.
- Subscribe to all tables after connection and let view/state logic determine what data is visible to the player.

## Movement model
- **WASD**: move using `W/A/S/D` keys (Bevy `KeyCode::KeyW/KeyA/KeyS/KeyD`).
- **Click-to-move**: clicking a map tile sets the next movement target position.
- If keyboard movement starts while a click target is active, keyboard input takes priority and clears/replaces the click target.

## Runtime ordering (Game view)
- Keep ordering explicit with update sets:
  1. input collection (keyboard/mouse)
  2. simulation (targeting/path/movement resolution)
  3. presentation (transform/camera/UI refresh)
- This keeps player movement and camera behavior deterministic as features grow.

## Implementation milestones
1. Define client app states/resources (`SignIn`, `CharacterSelect`, `Game`).
2. Implement sign-in token file lookup and token-auth attempt on `SignIn` enter.
3. Implement character selection + minimal creation form (name, gender).
4. Build the initial square grassland map with Kenney spritesheet assets.
5. Implement centered camera + player movement (WASD and click-to-move).
6. Organize modules as `app_state`, `events`, `resources`, and `features/{sign_in,character_select,game}` with one plugin per feature.
