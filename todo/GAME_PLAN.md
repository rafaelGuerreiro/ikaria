# Ikaria pseudo-Tibia planning

## Problem and current state
- Goal: deliver a pseudo-Tibia style game loop with small, visible, playable tasks.
- Current repo already has client state flow (`WorldSelection -> CharacterSelect -> CharacterCreation -> Game`) with world selection before sign-in.
- Token-based sign-in already connects directly to the selected world module using a single session connection.
- Backend is split into world-specific modules (`world-alpha-ikariadb`, `world-draconis-ikariadb`) over shared `ikariadb-core`.
- Client is React + Phaser + SpacetimeDB (TypeScript), using `subscribeToAllTables()` with local table reads.
- Game view uses Tibia-style panel layout with side panels, top bar, resizable bottom panel, and square center viewport.

## Confirmed product decisions
- Milestone 1 focus: **proper character creation workflow**.
- Pre-milestone requirement: **select world server before initial sign-in** to avoid dual-module auth.
- Connection/auth model: **single Spacetime module connection per session** (the selected world).
- Character visibility model: **after sign-in, show only characters that belong to the selected world**.
- World model: **distinct world servers ("Worlds") with different feature sets and maps over time**.
- Post-milestone-3 priority: **character stats foundation** (`capacity`, `hp`, `mana`, and skills: `melee`, `distance`, `magic`, `shield`).
- Stats scope includes **progression and regeneration**, delivered in small playable slices.
- Character creation fields: **name + gender + race**.
- Character races: **Human and Elf**.
- Character slots: **unlimited**.
- Character names: **globally unique**.
- Name validation: **3-20 chars, letters and spaces only**.
- On successful create: **auto-select and enter game immediately**.
- First playable world: **fixed handcrafted map**.
- Movement: **WASD**, **server-authoritative from day 1**.
- Milestone 1 must show **other players** moving.
- Milestone 2 priority: **chat bubbles above players**, visible only to nearby players using the same radius as gameplay visibility.
- Chat mode toggle: **Enter enables chat mode; Enter sends and disables chat mode**.
- Chat multiline: **Shift+Enter or Alt+Enter inserts newline instead of sending**.
- Chat message processing: **always trim message before send**.
- Chat message limit: **1024 characters maximum**.
- Milestone 3 priority: **proper inventory** with floor items, hand slots, an 8-slot bag, and stackable item combining.
- Milestone 3 hand model: **two generic hand slots (left/right)**.
- Milestone 3 stack rules: **item-specific stack caps** (example: coins 1000, apples 10).
- Milestone 3 pickup rule for stackables: **all-or-nothing** when no full fit is possible.
- Milestone 5 priority: **stairs traversal between floors**.
- Stair up control: **left mouse click on stair tile goes up**.
- Stair down control: **walking into stair hole goes down**.
- Milestone 6 priority: **mob AI** with two animal types (rats and trolls).
- Mob spawning: **fixed spawn points seeded with the map**.
- Mob idle behavior: **wander randomly until a player enters aggro radius**.
- Mob chase behavior: **once locked on a player, keep chasing until death** (Tibia-style).
- Mob movement: **server-side scheduled tick** (e.g. every 500ms).
- Milestone 7 priority: **melee combat** between players and mobs.
- Combat model: **melee auto-attack** (click target, character attacks automatically when adjacent).
- Mob death: **respawn after a cooldown timer** at original spawn point.
- Combat XP: **basic XP from kills feeding into M4 stats system**.

## Delivery strategy (small playable slices)
Build vertical slices where each slice can be tested in-game immediately.

### Milestone 0: world split foundation (before gameplay milestones)
**Status:** ✅ Complete

#### Backend track
- ✅ `m0-split-backend-modules` **COMPLETED**
   - Split backend into distinct world servers/modules that can diverge in features and maps.
   - Created `server/bins/world-alpha-ikariadb` as parallel backend module.
   - Created `server/bins/world-draconis-ikariadb` as parallel backend module.
   - Extracted shared backend code into `server/sdks/ikariadb-core` to eliminate duplication.
   - Both world modules now delegate to shared core.
   - Playable result: each world can run independently.
- ✅ `m0-character-service-schema` **COMPLETED**
   - Define world-scoped character domain so characters are owned by and listed within a single selected world.
   - Playable result: character list is isolated per world.
- ✅ `m0-world-service-schema` **COMPLETED**
   - Core world schema tables exist (`map_v1`, `town_temple_v1`, `character_position_v1`, `item_definition_v1`) and are isolated per world module.
   - Gameplay reducers/services exist: `create_character_v1`, `select_character_v1`, `move_character_v1`, `seed_initial_map`.
   - Playable result: world data and simulation loop are implemented.
- ✅ `m0-world-registry-contract` **COMPLETED**
   - World registry covers module routing (`database`) and world label (`name`, `description`).
   - Client reads world list from `client/src/worlds.ts` config.
   - Playable result: world routing works for all current needs.

#### Client track
- ✅ `m0-client-world-selection` **COMPLETED**
   - Manual world selection screen before initial sign-in.
   - Playable result: player explicitly chooses a world first.
- ✅ `m0-client-single-connection-flow` **COMPLETED**
   - Single connection flow: connect/authenticate only against selected world module.
   - Playable result: no second authentication is required to enter gameplay.

#### Shared validation
- ✅ `m0-world-split-playtest` **COMPLETED**
   - Full loop validated: choose world → sign in → see world-scoped characters → enter matching world server.
   - Playable result: pre-M1 architecture is proven.

### Milestone 1: character creation + multiplayer walking
**Status:** ✅ Complete

#### Backend track
- ✅ `m1-create-character-schema` **COMPLETED**
   - Schema supports name+gender+race creation contract with `CharacterV1` and `CharacterStatsV1` tables.
   - Playable result: backend contract supports proper create payload.
- ✅ `m1-create-character-reducer` **COMPLETED**
   - Reducer/service with name validation (3-20 chars, letters/spaces, no consecutive separators) and global name uniqueness via `#[unique]` canonical name.
   - Auto-selects character and initializes stats on creation.
   - Playable result: reducer callable and creates persisted characters.
- ✅ `m1-seed-fixed-map` **COMPLETED**
   - Deterministic handcrafted map seeded at init: grass area surrounded by water margins, with spawn/temple points.
   - Idempotent seeding via `seed_initial_map()` triggered by `SystemInit` event.
   - Playable result: world data exists to render and spawn in.
- ✅ `m1-character-spawn-session` **COMPLETED**
   - `spawn_character()` assigns world position on character selection (finds existing or creates at default spawn).
   - Position stored in `online_character_position_v1` table.
   - Playable result: entering game has a valid in-world spawn.
- ✅ `m1-move-reducer` **COMPLETED**
   - Server-authoritative movement reducer with cooldown, bounds, occupancy, and walkability checks.
   - 8-direction movement with diagonal Pythagorean cooldown scaling.
   - Playable result: position updates are authoritative on server.

#### Client track
- ✅ `m1-create-character-client-flow` **COMPLETED**
   - CharacterCreationView calls `createCharacterV1` reducer with name/gender/race.
   - Error handling and auto-enter game on success.
   - Playable result: user can create character and transition to game.
- ✅ `m1-game-map-render` **COMPLETED**
   - Phaser renders tile map from server `vw_world_map_v1` data (grass/water tiles).
   - Tibia-style panel layout: side panels (200px), top bar (40px), resizable bottom panel, square center viewport showing 10 tiles in each direction.
   - Playable result: player sees a real map.
- ✅ `m1-sync-player-entities` **COMPLETED**
   - Local player + nearby players rendered from `vw_nearby_characters_v1` and `vw_nearby_character_positions_v1` table data.
   - Smooth movement tweening with timestamp-based interpolation.
   - Playable result: multiplayer visibility in shared map.
- ✅ `m1-wasd-network-input` **COMPLETED**
   - WASD + arrow keys send `moveCharacterV1` reducer commands (8-direction).
   - Client-side prediction with server reconciliation.
   - Playable result: walking works end-to-end through server.

#### Shared validation
- ✅ `m1-playtest-multiplayer-loop` **COMPLETED**
   - Full loop validated: create → auto-enter → walk → see others.
   - Playable result: first pseudo-Tibia multiplayer walking loop complete.

### Milestone 2: local chat bubbles
**Status:** Not Started

#### Backend track
- `m2-chat-reducer-nearby`
   - Add say/chat reducer and delivery scoped by visible radius, receiving already-trimmed bounded messages.
   - Playable result: valid chat messages are distributed to nearby players only.

#### Client track
- `m2-chat-input-mode`
   - Implement chat input mode state: Enter turns chat on; Enter sends and turns chat off.
   - Playable result: players can reliably switch between movement and text input.
- `m2-chat-message-rules`
   - Implement text rules: Shift+Enter/Alt+Enter adds newline, trim on send, and 1024-char max.
   - Playable result: chat input behaves predictably and bounded.
- `m2-chat-bubble-client`
   - Render overhead chat text per player with duration based on message length.
   - Playable result: nearby players see temporary overhead text that clears automatically.

#### Shared validation
- `m2-chat-playtest-loop`
   - Validate full loop: enter chat mode -> type/multiline -> send -> exit chat mode -> nearby visibility only.
   - Playable result: complete chat loop works as intended in multiplayer.

### Milestone 3: proper inventory foundations
**Status:** Not Started

#### Backend track
- `m3-floor-item-instances`
   - Add server-side floor item instances and deterministic placement rules for visible item drops.
   - Playable result: items can exist on tiles and be synced to clients.
- `m3-pickup-to-bag`
   - Implement pickup reducer flow to move floor items into character inventory with all-or-nothing fit checks.
   - Playable result: player can pick items from floor into inventory.
- `m3-bag-eight-slots`
   - Enforce exactly 8 bag slots and block pickup when no slot/stack space is available.
   - Playable result: inventory capacity is clear and testable.
- `m3-item-stack-limits`
   - Add item-definition stack limits per item type (example: coins 1000, apples 10).
   - Playable result: stack behavior is consistent with item category.
- `m3-stackable-combine`
   - Implement stack merge rules using per-item limits, with all-or-nothing pickup when inventory cannot fully fit the stack.
   - Playable result: stackable items combine correctly and overflow behavior is deterministic.

#### Client track
- `m3-render-floor-items`
   - Render floor items in the game view so players can see loot on ground tiles.
   - Playable result: nearby floor items are visible in-world.
- `m3-hand-slots-equip`
   - Add hand equip flow so carried items can be equipped into hand slots.
   - Playable result: player can hold items in hands from inventory.

#### Shared validation
- `m3-inventory-playtest-loop`
   - Validate full loop: see floor items -> pick up -> manage 8-slot bag -> equip hands -> verify stack combining.
   - Playable result: first complete inventory gameplay loop is usable.

### Milestone 4: character stats foundations (after milestone 3)
**Status:** Not Started

#### Backend track
- `m4-stat-model-contract`
   - Define canonical stat contract (capacity, hp, mana, and skill entries for melee/distance/magic/shield) with world-scoped ownership.
   - Playable result: stats are modeled consistently across server/client contracts.
- `m4-base-stats-initialization`
   - Initialize base stats and skill rows for newly created characters.
   - Playable result: every new character starts with valid stats and skills.
- `m4-hp-mana-regen`
   - Implement deterministic hp/mana regeneration over time on the server.
   - Playable result: hp/mana values change naturally without manual commands.
- `m4-skill-progression-rules`
   - Implement deterministic progression rules for melee/distance/magic/shield.
   - Playable result: skills can increase through server-side progression logic.
- `m4-progression-trigger-actions`
   - Add minimal action hooks/reducers that can trigger skill progression before full combat is implemented.
   - Playable result: skill progression is testable in gameplay loop now.

#### Client track
- `m4-stats-sync-client`
   - Sync stat/skill updates to the client from authoritative server state.
   - Playable result: client always reflects current hp/mana/capacity/skills.
- `m4-stats-ui-panel`
   - Show hp, mana, capacity, and four skills in a simple visible UI panel.
   - Playable result: players can inspect stat changes in real time.

#### Shared validation
- `m4-stats-playtest-loop`
   - Validate loop: create/load character -> observe regen -> trigger progression -> verify UI updates from server.
   - Playable result: end-to-end stat foundation is playable and visible.

### Milestone 5: stair traversal between floors
**Status:** Not Started

#### Backend track
- `m5-stair-topology-model`
   - Define stair topology model (stair-up tile + stair-hole tile) and deterministic links between source/target floor positions.
   - Playable result: stairs have explicit floor-transition mapping.
- `m5-seed-stair-points`
   - Seed first stair links on map data so transitions are testable in-game.
   - Playable result: world has usable up/down stair points.
- `m5-floor-transition-guards`
   - Enforce transition guards (valid linked target tile, occupancy/walkability checks, deterministic failure behavior).
   - Playable result: stair transitions are safe and consistent.

#### Client track
- `m5-click-stair-up`
   - Implement left-click interaction on stair tile that moves character up one floor via authoritative server transition.
   - Playable result: clicking stair reliably goes up.
- `m5-walk-hole-down`
   - Implement automatic down transition when walking into stair hole tile.
   - Playable result: walking into hole reliably goes down.
- `m5-zlevel-sync-client`
   - Sync floor/z-level transitions to client rendering and visibility state.
   - Playable result: player and nearby entities render correctly after floor changes.

#### Shared validation
- `m5-stair-playtest-loop`
   - Validate loop: click stair up -> move around upper floor -> walk into hole -> return down with correct state sync.
   - Playable result: bi-directional stair traversal is fully playable.

### Milestone 6: mob AI (rats and trolls)
**Status:** Not Started

#### Backend track
- `m6-mob-definition-schema`
   - Define mob type table (`MobDefinitionV1`) with species (Rat, Troll), base stats (hp, speed, aggro radius, damage range), and spawn cooldown.
   - Playable result: mob types are data-driven and extensible.
- `m6-mob-spawn-points`
   - Define spawn point table (`MobSpawnV1`) with fixed positions seeded alongside the map in `seed_initial_map`.
   - Each spawn point references a mob definition, a position, and a respawn timer.
   - Playable result: world has predefined mob spawn locations.
- `m6-mob-instance-lifecycle`
   - Define live mob instance table (`MobInstanceV1`) with current hp, position, target, and state (Idle/Chasing).
   - Spawn mob instances from spawn points at init and after respawn cooldown.
   - Playable result: mobs exist as live entities in the world.
- `m6-mob-tick-scheduler`
   - Implement server-side scheduled tick (e.g. every 500ms) that drives all mob behavior.
   - Tick iterates live mob instances and executes AI state per mob.
   - Playable result: mobs act autonomously on a regular cadence.
- `m6-mob-idle-wander`
   - Implement idle behavior: mob picks a random walkable adjacent tile and moves to it.
   - Respect movement cooldown and walkability/occupancy rules (reuse existing tile checks).
   - Playable result: mobs wander visibly when no player is nearby.
- `m6-mob-aggro-chase`
   - Implement aggro detection: when a player enters the mob's aggro radius, lock on as target.
   - Implement chase behavior: mob pathfinds toward target player each tick (simple greedy step toward target).
   - Once locked, mob keeps chasing until it or the target dies (Tibia-style persistent aggro).
   - Playable result: mobs chase players and don't give up.

#### Client track
- `m6-render-mobs`
   - Render mob instances from server table data as sprites with species-specific visuals and name labels.
   - Smooth movement tweening using the same timestamp interpolation as player entities.
   - Playable result: mobs are visible and animate smoothly in the game view.
- `m6-mob-sync-updates`
   - React to mob position/state changes from server, including spawn and despawn events.
   - Playable result: mobs appear, move, and disappear in sync with server state.

#### Shared validation
- `m6-mob-playtest-loop`
   - Validate loop: enter world → see mobs wandering → approach mob → mob locks on and chases → mob follows persistently.
   - Playable result: mob AI is visible and responsive in multiplayer.

### Milestone 7: melee combat
**Status:** Not Started

#### Backend track
- `m7-combat-target-system`
   - Add target selection reducer: player clicks a mob to set it as combat target.
   - Store active combat target per character in server state.
   - Playable result: player can designate an attack target.
- `m7-melee-auto-attack`
   - Implement melee auto-attack loop: when character has a target and is adjacent, deal damage on a combat tick interval.
   - Damage calculated from character stats (melee skill, base damage) with randomized range.
   - Playable result: standing next to a targeted mob deals damage automatically.
- `m7-mob-melee-attack`
   - Implement mob attack behavior: when mob is adjacent to its chase target, deal damage based on mob definition stats.
   - Mobs attack on their scheduled tick when in melee range.
   - Playable result: mobs fight back when close to a player.
- `m7-damage-and-death`
   - Apply damage to hp for both players and mobs. When mob hp reaches 0, remove mob instance.
   - When player hp reaches 0, handle death (e.g. respawn at temple with penalty — keep simple for now).
   - Playable result: combat has consequences for both sides.
- `m7-mob-respawn`
   - After mob death, start respawn cooldown timer on the spawn point.
   - When cooldown expires, spawn a fresh mob instance at the original spawn position.
   - Playable result: mobs come back after being killed.
- `m7-combat-xp`
   - Award XP on mob kill based on mob definition (rats give less, trolls give more).
   - Feed XP into M4 stats system (melee skill progression, level/experience).
   - Playable result: killing mobs progresses character stats.

#### Client track
- `m7-target-selection-ui`
   - Implement click-to-target on mob sprites. Show visual indicator on targeted mob (highlight or health bar).
   - Playable result: player can see which mob is targeted.
- `m7-health-bars`
   - Render health bars above mobs and the local player showing current hp relative to max.
   - Update in real-time as damage is applied.
   - Playable result: combat damage is visually clear.
- `m7-combat-feedback`
   - Show damage numbers or hit indicators when attacks land (on both player and mobs).
   - Show death animation or visual feedback when a mob is killed.
   - Playable result: combat feels responsive and readable.
- `m7-death-respawn-client`
   - Handle player death: show death state, transition to respawn at temple.
   - Playable result: dying and respawning works smoothly on client.

#### Shared validation
- `m7-combat-playtest-loop`
   - Validate loop: target mob → auto-attack when adjacent → take damage from mob → kill mob → gain XP → mob respawns → die to mob → respawn at temple.
   - Playable result: full melee combat loop is playable end-to-end.

## Notes
- Keep each slice merged only when it is playable and observable in-game.
