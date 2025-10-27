# Onion Architecture in Rust with Axum

## Architecture Overview

```
┌──────────────────────────┐
│  Presentation  (Axum)    │  ← HTTP disappears here
├──────────────────────────┤
│  Infrastructure          │  ← DB, MQ, SMTP…
├──────────────────────────┤
│  Application (Use-cases) │  ← "what the system does"
├──────────────────────────┤
│  Domain     (Entities)   │  ← business sentences, pure
└──────────────────────────┘
```

**Core Principle**: Nothing inside knows anything outside. Arrows point **inward only**.

## Rust Enforcement Mechanism

- **Crate Dependencies**: Crate A can't use crate B unless `Cargo.toml` explicitly declares `dependencies.B`
- **Direction Control**: Outer crates list inner crates as dependencies, **never the reverse**
- **Visibility**: `pub` visibility controls what crosses ring boundaries

## Benefits

### 1. Stability
**Example Requirement Change**: "Send Slack message after order paid."

**Impact**:
- Touch only Infrastructure (add Slack adapter)
- Touch Application (call the adapter)
- **Domain does not even recompile**

### 2. Test Speed
```bash
cargo test -p domain    # Zero I/O, milliseconds
cargo test -p application # FakeRepo trait implementation, no DB
```
Only the API crate needs real Postgres running.

### 3. Language Freedom
- Domain is `no_std` friendly
- Swap Axum for Warp, Rocket, or even Go by rewriting only Presentation layer
- Inner crates remain unchanged

### 4. Borrow-Checker Friendliness
- Pure logic in Domain: no `.await`, no `Arc<Mutex<_>>`, no lifetime fights
- Side-effects confined to edges where `async`, `tokio`, `sqlx` already require ownership consideration

## Axum Integration Without Leaks

### 1. Router as Value
- Router built in `main.rs`
- No business code sees `Router`, `Path`, `Json`, `State`

### 2. Handler as Adapter
```rust
fn(JsonDto, State<D>) -> impl IntoResponse
```
- Deserialize JSON → plain struct (DTO)
- Call use-case
- Map result to status code + body
- **3–5 lines total**, no business logic in handlers

### 3. Trait-Based State
```rust
struct AppState { 
    repo: Arc<dyn UserRepo>  // Trait defined in Domain
}
```
Axum only knows the trait, never concrete Postgres types.

### 4. Tower Middleware
Middleware wraps outermost service, leaving Domain unaffected.

## Typical Flow (One Sentence Per Layer)

1. **Presentation**: "Someone POST /users {name}"
2. **Infrastructure**: "I will INSERT INTO users"  
3. **Application**: "CreateUser use-case: validate name, save, return User"
4. **Domain**: "A User must have a non-empty name; here is the entity."

## Extreme Flexibility: Delete Axum Entirely

1. Remove crate `api`
2. Add crate `cli` that depends on `application`
3. ```rust
   fn main() { 
       CreateUser::new(FakeRepo).execute("Ada".into()) 
   }
   ```
**Everything inside still compiles** - you just peeled off one skin of the onion.

## One-Line Summary

**Onion + Rust crates = compiler-drawn boundary lines; Axum lives on the outer peel, never squeezing its types into the juicy interior.**
```
