# Ikaria Authentication Strategy (Desktop + Web)

## Status

**Decision:** Use **Auth0 as the single OpenID Connect (OIDC) provider** for both:
- The Bevy desktop client (PC/macOS)
- The website (account + coin purchases)

This gives one user identity across platforms and avoids custom password/auth infrastructure inside the game backend.

---

## Why this direction

### Problems with the current flow

- The current desktop client persists a SpacetimeDB token in a local file (`.ikaria_token`).
- If that token is missing, a new identity is created on connect.
- This makes account portability across devices difficult and increases accidental account fragmentation.

### Why Auth0 + OIDC

- SpacetimeDB supports OIDC tokens and computes identity from JWT claims (`iss` + `sub`).
- If desktop and web use the same Auth0 tenant/provider, the same person resolves to the same SpacetimeDB identity.
- Auth0 supports social login providers (including Google and Apple), which addresses stronger identity assurance and cross-platform sign-in.
- This enables website purchases to map to the same game account without building custom auth primitives first.

---

## Scope and non-goals

### In scope

- Identity provider and login architecture for game + website
- How to keep one account identity across clients
- Security checks required in SpacetimeDB reducers
- Rollout guidance with phased implementation

### Out of scope (for now)

- OS keychain implementation details in Rust (explicitly deferred)
- Full payment provider integration
- Custom in-house identity provider

---

## Target architecture

### 1) Identity provider

- Use one Auth0 tenant for the product environment.
- Configure one application/client for desktop and one for web (or equivalent structure), both under the same tenant.
- Enable desired social connections (Google + Apple).

### 2) Desktop (Bevy) authentication flow

- Use **Authorization Code + PKCE** for native apps.
- Open system browser for login.
- Receive ID token after successful login.
- Pass ID token into SpacetimeDB connection (`with_token`).

> Note: The SpacetimeDB Auth0 documentation example is React-specific for convenience, not a protocol limitation. Bevy can follow the same OIDC principles using Rust-native OIDC/OAuth libraries.

### 3) Website authentication flow

- Website authenticates the user with the same Auth0 tenant.
- Website backend trusts Auth0 identity and stores/uses the stable user identifier (`iss` + `sub` pair, or equivalent normalized ID).
- Coin purchase records are linked to that identity.

### 4) SpacetimeDB backend trust boundary

- SpacetimeDB validates JWT signatures and provides claims.
- Module reducers should enforce:
  - JWT presence (no anonymous connect for production)
  - Allowed issuer(s)
  - Allowed audience (`aud`) for the expected client(s)

This prevents tokens minted for other apps/providers from being accepted unintentionally.

---

## Account linking model

Use **OIDC subject identity** as the primary account key:

- Canonical account key source: `iss` + `sub`
- Stable across devices for the same Auth0 user
- Shared between desktop and website when both use the same provider/tenant

### Auth0 multi-provider caveat (Google + Apple)

- In Auth0, identities from different providers are separate by default, even when they share the same email.
- A user signing in with Google and Apple can create two distinct Auth0 users unless you implement account linking.
- If you support both, add explicit/suggested account linking flow and require re-authentication for both accounts before linking.
- Do not use raw email as canonical identity key (Apple may use private relay emails).

For game/business data:
- Store a mapping between game profile/account records and canonical OIDC identity.
- Coins and entitlements reference that same canonical account identity.

---

## Security posture

### Recommended

- Require authenticated JWT on connect in production.
- Restrict accepted issuer(s) and audience(s) at `client_connected`.
- Keep token lifetime short where possible; rely on provider session/refresh behavior.
- Log auth failures with explicit reasons (missing token, invalid issuer, invalid audience).

### Avoid

- Custom email/password auth in reducers.
- Password hashing and credential lifecycle inside SpacetimeDB module logic.
- Long-lived plaintext token files as a long-term production storage strategy.

---

## OS keychain decision (deferred)

Keychain-backed secure token storage is the desired end state for desktop, but implementation is deferred for now.

Interim rule:
- During development, local token file usage can continue temporarily.
- Treat file-based token persistence as **development-only**, not final production posture.

Follow-up implementation should evaluate a cross-platform Rust approach for secure credential storage on macOS and Windows.

---

## Phased rollout plan

### Phase 1: Identity unification foundation

1. Finalize Auth0 tenant/apps and social providers (Google + Apple).
2. Implement desktop OIDC login (PKCE) and send ID token to SpacetimeDB.
3. Update backend `client_connected` authorization checks (JWT + issuer + audience).
4. Ensure website login uses same tenant and stable identity mapping.

### Phase 2: Commerce/account integration

1. Link website purchase records to canonical OIDC identity.
2. Add backend flow to apply purchased coins/entitlements to the mapped game account.
3. Add admin/support tooling for identity/account troubleshooting.

### Phase 3: Desktop credential hardening

1. Replace file token persistence with OS keychain/secure storage.
2. Add logout/session-revoke behavior aligned with provider semantics.
3. Document local development fallback behavior.

---

## Final recommendation

Adopt **Auth0-centered OIDC** as the single identity system across desktop and web, enforce claim validation in SpacetimeDB, and defer keychain implementation to a dedicated hardening phase.

This balances security, portability, and implementation complexity while keeping a clear path to cross-device accounts and website-based coin purchases.
