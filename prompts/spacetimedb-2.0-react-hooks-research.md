# SpacetimeDB 2.0 React Hooks Research

## Package & Installation

```bash
pnpm install spacetimedb
```

The single `spacetimedb` package (v2.0.1) provides subpath exports:
- `spacetimedb` - Core SDK (connection, types, BSATN encoding)
- `spacetimedb/react` - React hooks and provider
- `spacetimedb/server` - Server-side module definitions
- `spacetimedb/vue`, `spacetimedb/svelte`, `spacetimedb/angular`, `spacetimedb/tanstack` - Other framework bindings

Peer dependency: `react ^18.0.0 || ^19.0.0`

> Note: The old `@clockworklabs/spacetimedb-sdk` package is deprecated. Use `spacetimedb` directly.

---

## 1. Architecture Overview

SpacetimeDB 2.0 TypeScript SDK works as follows:

1. **Code generation**: Run `spacetime generate` against your deployed module. This produces a `module_bindings/` folder containing:
   - `DbConnection` class (typed to your module's schema)
   - `tables` export (query builder for each table/view)
   - `reducers` export (accessor map for each reducer)
   - Type definitions for all server-side types (enums, structs)

2. **Connection**: Client connects via WebSocket to SpacetimeDB. The SDK maintains a local **client cache** that mirrors subscribed rows.

3. **Subscriptions**: Client registers SQL-like subscription queries. The server sends the initial matching rows, then pushes real-time deltas.

4. **React layer**: `SpacetimeDBProvider` manages the WebSocket lifecycle. Hooks (`useTable`, `useReducer`, `useSpacetimeDB`) provide reactive access to subscribed data.

---

## 2. React Exports from `spacetimedb/react`

```typescript
export { SpacetimeDBProvider } from './SpacetimeDBProvider';
export { useSpacetimeDB } from './useSpacetimeDB';
export { useTable } from './useTable';
export { useReducer } from './useReducer';
```

---

## 3. SpacetimeDBProvider

Wraps the component tree to manage the WebSocket connection lifecycle.

```tsx
import { SpacetimeDBProvider } from 'spacetimedb/react';
import { DbConnection } from './module_bindings';

const connectionBuilder = DbConnection.builder()
  .withUri('wss://maincloud.spacetimedb.com')
  .withDatabaseName('my-database')
  .withToken(localStorage.getItem('auth_token') || undefined)
  .onConnect((conn, identity, token) => {
    localStorage.setItem('auth_token', token);
    console.log('Connected as', identity.toHexString());
  })
  .onDisconnect((ctx, error) => {
    console.log('Disconnected', error);
  })
  .onConnectError((ctx, error) => {
    console.error('Connection error', error);
  });

function App() {
  return (
    <SpacetimeDBProvider connectionBuilder={connectionBuilder}>
      <MyApp />
    </SpacetimeDBProvider>
  );
}
```

### Key details:
- Pass the **builder** (do NOT call `.build()`) -- the provider calls it internally
- Uses `ConnectionManager` internally with a retain/release pattern
- Compatible with React StrictMode (only one WebSocket created despite double-mount)
- Uses `useSyncExternalStore` for state synchronization
- Connection state includes: `isActive`, `identity`, `token`, `connectionId`, `connectionError`

---

## 4. useSpacetimeDB Hook

Returns the current connection state. Re-renders when connection state changes.

```typescript
import { useSpacetimeDB } from 'spacetimedb/react';

function MyComponent() {
  const {
    isActive,           // boolean - whether WebSocket is connected
    identity,           // Identity | undefined - the connected user's identity
    token,              // string | undefined - private auth token
    connectionId,       // ConnectionId - unique connection identifier
    connectionError,    // Error | undefined - last connection error
    getConnection,      // () => DbConnection | null - get raw connection
  } = useSpacetimeDB();

  if (!isActive) return <div>Connecting...</div>;
  if (connectionError) return <div>Error: {connectionError.message}</div>;

  return <div>Connected as {identity?.toHexString()}</div>;
}
```

### ConnectionState type:

```typescript
type ConnectionState = {
  isActive: boolean;
  identity?: Identity;
  token?: string;
  connectionId: ConnectionId;
  connectionError?: Error;
  getConnection(): DbConnectionImpl<any> | null;
};
```

---

## 5. useTable Hook

The primary hook for subscribing to table data. Automatically:
1. Creates a subscription query via `subscriptionBuilder().subscribe(querySql)`
2. Registers `onInsert`, `onDelete`, `onUpdate` callbacks on the table
3. Uses `useSyncExternalStore` for efficient React updates
4. Unsubscribes on cleanup

### Signature:

```typescript
function useTable<TableDef>(
  query: Query<TableDef>,
  callbacks?: UseTableCallbacks<RowType<TableDef>>
): [readonly RowType<TableDef>[], boolean]
```

Returns a tuple:
- `[0]` - `readonly Row[]` - the current matching rows
- `[1]` - `boolean` - whether the subscription has been applied (initial data loaded)

### Basic usage:

```tsx
import { useTable } from 'spacetimedb/react';
import { tables } from './module_bindings';

function PlayerList() {
  // Subscribe to all rows in a table
  const [players, isReady] = useTable(tables.player);

  if (!isReady) return <div>Loading...</div>;
  return (
    <ul>
      {players.map(p => <li key={p.playerId.toString()}>{p.name}</li>)}
    </ul>
  );
}
```

### Filtered subscription:

```tsx
function OnlineUsers() {
  const [onlineUsers, isReady] = useTable(
    tables.user.where(r => r.online.eq(true))
  );

  return <div>{onlineUsers.length} users online</div>;
}
```

### With callbacks:

```tsx
function ChatRoom() {
  const [messages, isReady] = useTable(tables.message);

  const [users, usersReady] = useTable(
    tables.user.where(r => r.online.eq(true)),
    {
      onInsert: (user) => {
        console.log(`${user.name} joined`);
      },
      onDelete: (user) => {
        console.log(`${user.name} left`);
      },
      onUpdate: (oldUser, newUser) => {
        console.log(`${oldUser.name} changed to ${newUser.name}`);
      },
    }
  );

  // ...
}
```

### UseTableCallbacks interface:

```typescript
interface UseTableCallbacks<RowType> {
  onInsert?: (row: RowType) => void;
  onDelete?: (row: RowType) => void;
  onUpdate?: (oldRow: RowType, newRow: RowType) => void;
}
```

### How callbacks interact with where filters:

When using a `where` clause with `onUpdate`, the hook classifies row changes:
- **enter**: Row didn't match filter before, now matches -> calls `onInsert`
- **leave**: Row matched before, no longer matches -> calls `onDelete`
- **stayIn**: Row matched before and after -> calls `onUpdate`
- **stayOut**: Row didn't match before or after -> no-op

---

## 6. useReducer Hook

Wraps a reducer definition into a callable async function. Queues calls made before the connection is ready.

### Signature:

```typescript
function useReducer<ReducerDef>(
  reducerDef: ReducerDef
): (...params: ParamsType<ReducerDef>) => Promise<void>
```

### Usage:

```tsx
import { useReducer } from 'spacetimedb/react';
import { reducers } from './module_bindings';

function ChatInput() {
  const sendMessage = useReducer(reducers.sendMessage);
  const [text, setText] = useState('');

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await sendMessage({ text });
      setText('');
    } catch (error) {
      console.error('Failed to send:', error);
    }
  };

  return (
    <form onSubmit={handleSubmit}>
      <input value={text} onChange={e => setText(e.target.value)} />
      <button type="submit">Send</button>
    </form>
  );
}
```

### Key details:
- Returns a `Promise<void>` that resolves when the reducer completes
- If called before connection is established, calls are **queued** and flushed when connected
- The returned function is stable (memoized via `useCallback`)

---

## 7. Query Builder API

The `tables` export provides type-safe query builders for each table.

### Basic queries:

```typescript
import { tables } from './module_bindings';

// All rows from a table
tables.user

// Filtered query
tables.user.where(r => r.online.eq(true))

// Multiple conditions
tables.user.where(r => r.age.gte(18).and(r.status.eq('active')))
```

### Available operators on column expressions:
- **Comparison**: `eq`, `ne`, `lt`, `lte`, `gt`, `gte`
- **Combinators**: `and()`, `or()`, `not()`

### Semijoins (advanced):

```typescript
tables.player
  .where(p => p.score.gte(1000))
  .leftSemijoin(tables.playerLevel, (p, pl) => p.id.eq(pl.playerId))
  .where(p => p.online.eq(true));
```

---

## 8. Connection Management (Non-React / Manual)

For cases outside the React provider (e.g., server-side fetching in Next.js):

```typescript
import { DbConnection, tables } from './module_bindings';

const conn = DbConnection.builder()
  .withUri('wss://maincloud.spacetimedb.com')
  .withDatabaseName('my-database')
  .withToken(savedToken)
  .onConnect((conn, identity, token) => {
    // Subscribe to tables after connection
    conn.subscriptionBuilder()
      .onApplied(() => {
        // Initial data is now in the client cache
        const users = Array.from(conn.db.user.iter());
        console.log('Users:', users);
      })
      .subscribe([tables.user, tables.message]);
  })
  .onDisconnect((ctx, error) => {
    console.log('Disconnected');
  })
  .build();

// Access client cache directly
for (const user of conn.db.user.iter()) {
  console.log(user.name);
}
const count = conn.db.user.count();
const user = conn.db.user.byId.find(userId);

// Register row callbacks
conn.db.user.onInsert((ctx, row) => console.log('Inserted:', row));
conn.db.user.onUpdate((ctx, old, updated) => console.log('Updated:', updated));
conn.db.user.onDelete((ctx, row) => console.log('Deleted:', row));

// Invoke reducers
conn.reducers.sendMessage({ text: 'Hello' });
conn.reducers.setName({ name: 'Alice' });

// Disconnect
conn.disconnect();
```

### DbConnection.builder() methods:
- `.withUri(uri)` - WebSocket URI (ws:// or wss://)
- `.withDatabaseName(name)` - Module/database name (replaces 1.0's `withModuleName`)
- `.withToken(token)` - Auth token for session resumption
- `.onConnect((conn, identity, token) => ...)` - Connection success callback
- `.onDisconnect((ctx, error) => ...)` - Disconnection callback
- `.onConnectError((ctx, error) => ...)` - Connection error callback
- `.withConfirmedReads(boolean)` - Default true in 2.0; waits for transaction durability

---

## 9. Authentication / Identity

### Anonymous identity (default):

SpacetimeDB assigns a unique `Identity` to each connection. The `token` returned in `onConnect` is a private key that can be stored and reused to reconnect as the same identity.

```typescript
// Save token on connect
const builder = DbConnection.builder()
  .withToken(localStorage.getItem('auth_token') || undefined)
  .onConnect((conn, identity, token) => {
    localStorage.setItem('auth_token', token);
  });
```

### OIDC Authentication (SpacetimeAuth):

For production apps with real user accounts, SpacetimeDB supports OpenID Connect:

```typescript
import { AuthProvider, useAuth } from 'react-oidc-context';

const oidcConfig = {
  authority: 'https://auth.spacetimedb.com/oidc',
  client_id: 'YOUR_CLIENT_ID',
  redirect_uri: `${window.location.origin}/callback`,
  scope: 'openid profile email',
  response_type: 'code',
  automaticSilentRenew: true,
};

// Wrap app with AuthProvider, then use the OIDC token
const auth = useAuth();
const builder = DbConnection.builder()
  .withToken(auth.user?.id_token);
```

### Identity in components:

```tsx
function Profile() {
  const { identity, token } = useSpacetimeDB();

  return (
    <div>
      <p>Identity: {identity?.toHexString()}</p>
      <p>Token stored: {token ? 'Yes' : 'No'}</p>
    </div>
  );
}
```

---

## 10. Migration Notes (1.0 -> 2.0)

Key breaking changes:

| 1.0 | 2.0 |
|-----|-----|
| `withModuleName()` | `withDatabaseName()` |
| `conn.reducers.onXyz(callback)` | Event tables + `conn.db.eventTable.onInsert()` |
| `withLightMode(true)` | Removed (no longer needed) |
| `CallReducerFlags` | Removed |
| `Event.UnknownTransaction` | `Event.Transaction` |
| Table `name` param | Table `accessor` param in schema |
| Confirmed reads opt-in | Confirmed reads default on |

### Event tables (replacing reducer callbacks):

In 2.0, global reducer callbacks are removed. Instead, create "event tables" on the server and subscribe to their inserts:

```typescript
// Server-side
const damageEvent = table({ event: true }, {
  target: t.identity(),
  amount: t.u32(),
});

// Client-side
conn.db.damageEvent.onInsert((ctx, { target, amount }) => {
  playDamageAnimation(target, amount);
});
```

---

## 11. Complete Example (Chat App Pattern)

```tsx
// main.tsx
import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import { SpacetimeDBProvider } from 'spacetimedb/react';
import { DbConnection } from './module_bindings';
import App from './App';

const HOST = import.meta.env.VITE_SPACETIMEDB_HOST ?? 'ws://localhost:3000';
const DB_NAME = import.meta.env.VITE_SPACETIMEDB_DB_NAME ?? 'my-chat';
const TOKEN_KEY = `${HOST}/${DB_NAME}/auth_token`;

const connectionBuilder = DbConnection.builder()
  .withUri(HOST)
  .withDatabaseName(DB_NAME)
  .withToken(localStorage.getItem(TOKEN_KEY) || undefined)
  .onConnect((_conn, identity, token) => {
    localStorage.setItem(TOKEN_KEY, token);
    console.log('Connected:', identity.toHexString());
  })
  .onDisconnect(() => console.log('Disconnected'))
  .onConnectError((_ctx, error) => console.error('Error:', error));

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <SpacetimeDBProvider connectionBuilder={connectionBuilder}>
      <App />
    </SpacetimeDBProvider>
  </StrictMode>
);
```

```tsx
// App.tsx
import { useState, useMemo, useEffect } from 'react';
import { useSpacetimeDB, useTable, useReducer } from 'spacetimedb/react';
import { tables, reducers } from './module_bindings';

export default function App() {
  const { identity, isActive, connectionError } = useSpacetimeDB();
  const [messages, messagesReady] = useTable(tables.message);
  const [onlineUsers, usersReady] = useTable(
    tables.user.where(r => r.online.eq(true)),
    {
      onInsert: (user) => console.log(`${user.name} joined`),
      onDelete: (user) => console.log(`${user.name} left`),
    }
  );

  const sendMessage = useReducer(reducers.sendMessage);
  const setName = useReducer(reducers.setName);

  const [text, setText] = useState('');

  const sortedMessages = useMemo(
    () => [...messages].sort((a, b) => Number(a.sent - b.sent)),
    [messages]
  );

  if (!isActive) return <div>Connecting...</div>;
  if (connectionError) return <div>Error: {connectionError.message}</div>;
  if (!messagesReady || !usersReady) return <div>Loading data...</div>;

  const handleSend = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!text.trim()) return;
    try {
      await sendMessage({ text: text.trim() });
      setText('');
    } catch (err) {
      console.error('Send failed:', err);
    }
  };

  return (
    <div>
      <h2>Online: {onlineUsers.length}</h2>
      <ul>
        {sortedMessages.map((msg, i) => (
          <li key={i}>{msg.text}</li>
        ))}
      </ul>
      <form onSubmit={handleSend}>
        <input value={text} onChange={e => setText(e.target.value)} />
        <button type="submit">Send</button>
      </form>
    </div>
  );
}
```

---

## 12. Existing Patterns in This Codebase (ikaria0)

The project already uses SpacetimeDB 2.0 React hooks in `client/src/`:

- **App.tsx**: Creates `DbConnection.builder()` with token persistence, wraps children in `SpacetimeDBProvider`
- **CharacterFlow.tsx**: Uses `useSpacetimeDB()`, `useTable(tables.vw_character_all_mine_v1)`, `useTable(tables.vw_character_me_v1)`, and `useReducer(reducers.createCharacterV1)` / `useReducer(reducers.selectCharacterV1)`
- Token storage: `localStorage` with key `ikaria.auth.token`
- Connection: `https://maincloud.spacetimedb.com` targeting `world-alpha-ikariadb`

> Note: The codebase uses `withLightMode(true)` in App.tsx which was removed in 2.0. This may need to be cleaned up.
