# Vendored dependencies

Two crates this workflow depends on are kept here rather than pulled from
crates.io or git. Both were written by **Hamid R. Ghadyani**, the original
author of this workflow, and both are unmaintained — no release since 2023.

| Crate | Vendored from | Upstream |
|---|---|---|
| `rusty-pin` 0.6.0 | git rev `2681a760` | [spamwax/rusty-pin](https://github.com/spamwax/rusty-pin) — MIT |
| `alfred-rs` 0.7.1 | crates.io | [spamwax/alfred-workflow](https://github.com/spamwax/alfred-workflow) — MIT/Apache-2.0 |

`rusty-pin`'s licence is preserved at [rusty-pin/LICENSE.md](rusty-pin/LICENSE.md).
`alfred-rs` declares MIT/Apache-2.0 in its `Cargo.toml`; the published crate
ships no separate licence file.

## Why vendored

**`rusty-pin` was the single biggest fragility in the project.** It is not on
crates.io, has no other fork, and its `master` branch is API-incompatible with
this workflow — so any `cargo update`, or a lost `Cargo.lock`, broke the build
outright. If the upstream repository were renamed, made private or deleted, the
build would simply stop working with no fallback.

Vendoring removes that class of problem entirely: there is no git dependency, no
revision to pin, and nothing external that can move or disappear. It also keeps
everything in one repository rather than spreading a personal fork across
several, which is a deliberate maintenance choice for a solo project.

## Local changes

Both crates are kept as close to upstream as possible. The changes are:

- **`env_logger` 0.9 → 0.10** in both. 0.9 depended on `atty`, which carries
  RUSTSEC-2021-0145 (unsound) and RUSTSEC-2024-0375 (unmaintained). Both crates
  only ever call `env_logger::try_init()`, so the bump is behaviour-neutral.
  This was the last thing keeping `atty` in the tree.
- **`rusty-pin`**: two return types in `src/pinboard/mod.rs` gained explicit
  lifetimes, applied by `cargo clippy --fix`, to silence
  `mismatched_lifetime_syntaxes` on current Rust.
- Dead CI configuration (`ci/`, `appveyor.yml`, `disable-travis`) was not
  copied across.

Nothing else was modified. If you ever need to compare against upstream, the
provenance in the table above is exact.
