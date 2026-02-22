# Ikaria pseudo-Tibia planning

## Problem and current state
- Goal: deliver a pseudo-Tibia style game loop with small, visible, playable tasks.
- Current repo already has client state flow (`SignIn -> CharacterSelect -> Game`) and token-based sign-in wiring.
- Current backend is still a single server module with account/world gameplay data combined.
- Character creation UI exists but backend reducer is missing.
- Game view is still placeholder UI; no tile map rendering, no movement loop, and no player sync.
- Backend currently defines tables/events but lacks gameplay reducers for create/select/spawn/move/chat.

## Confirmed product decisions
- Milestone 1 focus: **proper character creation workflow**.
- Pre-milestone requirement: **select world server before initial sign-in** to avoid dual-module auth.
- Connection/auth model: **single Spacetime module connection per session** (the selected world).
- Character visibility model: **after sign-in, show only characters that belong to the selected world**.
- World model: **distinct world servers ("Worlds") with different feature sets and maps over time**.
- Post-milestone-3 priority: **character stats foundation** (`capacity`, `hp`, `mana`, and skills: `melee`, `distance`, `magic`, `shield`).
- Stats scope includes **progression and regeneration**, delivered in small playable slices.
- Character creation fields: **name + gender only**.
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

## Delivery strategy (small playable slices)
Build vertical slices where each slice can be tested in-game immediately.

### Milestone 0: world split foundation (before gameplay milestones)
- `m0-split-backend-modules`
   - Split backend into distinct world servers/modules that can diverge in features and maps.
   - Playable result: each world can run independently.
- `m0-character-service-schema`
   - Define world-scoped character domain so characters are owned by and listed within a single selected world.
   - Playable result: character list is isolated per world.
- `m0-world-service-schema`
   - Keep world simulation data per world server (map, positions, movement state, floor items, chat visibility context).
   - Playable result: world state evolves independently per world.
- `m0-world-registry-contract`
   - Define "World" metadata contract (world id/name/features/map identity) so client can choose a world pre-auth.
   - Playable result: client can render world list before signing in.
- `m0-client-world-selection`
   - Add manual world selection screen before initial sign-in.
   - Playable result: player explicitly chooses a world first.
- `m0-client-dual-connection-flow`
   - Implement single connection flow: connect/authenticate only against selected world module.
   - Playable result: no second authentication is required to enter gameplay.
- `m0-world-split-playtest`
   - Validate loop: choose world -> sign in -> see world-scoped characters -> enter matching world server.
   - Playable result: pre-M1 architecture is proven before gameplay milestones.

### Milestone 1: character creation + multiplayer walking
- `m1-create-character-schema`
   - Add schema support required for name+gender creation contract.
   - Playable result: backend contract supports proper create payload.
- `m1-create-character-reducer`
   - Implement reducer/service with validation and global name uniqueness.
   - Playable result: reducer callable and creates persisted characters.
- `m1-create-character-client-flow`
   - Wire CharacterSelect form to reducer, show errors, auto-enter game on success.
   - Playable result: user can create character and transition to game.
- `m1-seed-fixed-map`
   - Seed deterministic handcrafted map + spawn/temple points at init.
   - Playable result: world data exists to render and spawn in.
- `m1-character-spawn-session`
   - Ensure created/selected character receives world position for session start.
   - Playable result: entering game has a valid in-world spawn.
- `m1-move-reducer`
   - Add server-authoritative movement reducer with walkability/bounds checks.
   - Playable result: position updates are authoritative on server.
- `m1-game-map-render`
   - Render simple tile map in Bevy game state.
   - Playable result: player sees a real map instead of placeholder.
- `m1-sync-player-entities`
   - Render local player + other players from synchronized table data.
   - Playable result: multiplayer visibility in shared map.
- `m1-wasd-network-input`
   - Send WASD as movement reducer commands and reflect server state updates.
   - Playable result: walking works end-to-end through server.
- `m1-playtest-multiplayer-loop`
    - Validate full loop: create -> auto-enter -> walk -> see others.
    - Playable result: first pseudo-Tibia multiplayer walking loop complete.

### Milestone 2: local chat bubbles
- `m2-chat-input-mode`
    - Implement chat input mode state: Enter turns chat on; Enter sends and turns chat off.
    - Playable result: players can reliably switch between movement and text input.
- `m2-chat-message-rules`
    - Implement text rules: Shift+Enter/Alt+Enter adds newline, trim on send, and 1024-char max.
    - Playable result: chat input behaves predictably and bounded.
- `m2-chat-reducer-nearby`
    - Add say/chat reducer and delivery scoped by visible radius, receiving already-trimmed bounded messages.
    - Playable result: valid chat messages are distributed to nearby players only.
- `m2-chat-bubble-client`
    - Render overhead chat text per player with duration based on message length.
    - Playable result: nearby players see temporary overhead text that clears automatically.
- `m2-chat-playtest-loop`
    - Validate full loop: enter chat mode -> type/multiline -> send -> exit chat mode -> nearby visibility only.
    - Playable result: complete chat loop works as intended in multiplayer.

### Milestone 3: proper inventory foundations
- `m3-floor-item-instances`
    - Add server-side floor item instances and deterministic placement rules for visible item drops.
    - Playable result: items can exist on tiles and be synced to clients.
- `m3-render-floor-items`
    - Render floor items in the game view so players can see loot on ground tiles.
    - Playable result: nearby floor items are visible in-world.
- `m3-pickup-to-bag`
    - Implement pickup reducer flow to move floor items into character inventory with all-or-nothing fit checks.
    - Playable result: player can pick items from floor into inventory.
- `m3-bag-eight-slots`
    - Enforce exactly 8 bag slots and block pickup when no slot/stack space is available.
    - Playable result: inventory capacity is clear and testable.
- `m3-hand-slots-equip`
    - Add hand equip flow so carried items can be equipped into hand slots.
    - Playable result: player can hold items in hands from inventory.
- `m3-item-stack-limits`
    - Add item-definition stack limits per item type (example: coins 1000, apples 10).
    - Playable result: stack behavior is consistent with item category.
- `m3-stackable-combine`
    - Implement stack merge rules using per-item limits, with all-or-nothing pickup when inventory cannot fully fit the stack.
    - Playable result: stackable items combine correctly and overflow behavior is deterministic.
- `m3-inventory-playtest-loop`
    - Validate full loop: see floor items -> pick up -> manage 8-slot bag -> equip hands -> verify stack combining.
    - Playable result: first complete inventory gameplay loop is usable.

### Milestone 4: character stats foundations (after milestone 3)
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
- `m4-stats-sync-client`
   - Sync stat/skill updates to the client from authoritative server state.
   - Playable result: client always reflects current hp/mana/capacity/skills.
- `m4-stats-ui-panel`
   - Show hp, mana, capacity, and four skills in a simple visible UI panel.
   - Playable result: players can inspect stat changes in real time.
- `m4-stats-playtest-loop`
   - Validate loop: create/load character -> observe regen -> trigger progression -> verify UI updates from server.
   - Playable result: end-to-end stat foundation is playable and visible.

### Milestone 5: stair traversal between floors
- `m5-stair-topology-model`
   - Define stair topology model (stair-up tile + stair-hole tile) and deterministic links between source/target floor positions.
   - Playable result: stairs have explicit floor-transition mapping.
- `m5-seed-stair-points`
   - Seed first stair links on map data so transitions are testable in-game.
   - Playable result: world has usable up/down stair points.
- `m5-click-stair-up`
   - Implement left-click interaction on stair tile that moves character up one floor via authoritative server transition.
   - Playable result: clicking stair reliably goes up.
- `m5-walk-hole-down`
   - Implement automatic down transition when walking into stair hole tile.
   - Playable result: walking into hole reliably goes down.
- `m5-floor-transition-guards`
   - Enforce transition guards (valid linked target tile, occupancy/walkability checks, deterministic failure behavior).
   - Playable result: stair transitions are safe and consistent.
- `m5-zlevel-sync-client`
   - Sync floor/z-level transitions to client rendering and visibility state.
   - Playable result: player and nearby entities render correctly after floor changes.
- `m5-stair-playtest-loop`
   - Validate loop: click stair up -> move around upper floor -> walk into hole -> return down with correct state sync.
   - Playable result: bi-directional stair traversal is fully playable.

## Notes
- Keep each slice merged only when it is playable and observable in-game.
