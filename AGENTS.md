# AGENTS.md

## Project

Rust turn-based RPG inspired by **Persona 3 Portable** — easy to read, simple structure.

## Status

Repo was previously a web project (KWGT hub) and is being repurposed. Now contains a working Rust TUI turn-based RPG game inspired by Persona 3 Portable.

## Setup

```bash
cargo build
cargo run
```

## Structure

Flat module layout for a small RPG:
- `src/main.rs` — entry point, game loop, state machine
- `src/data.rs` — `Personaje`, `Enemigo`, `Skill`, `EstadoJuego`, `EstadoCombate`
- `src/combat.rs` — turn-based combat logic, damage calculation, level-up
- `src/ui.rs` — crossterm rendering (title, exploration, combat, skill selection)

Dependencies: `crossterm` (terminal I/O), `rand` (encounters/damage).

## Conventions

- `crossterm` and `rand` are the only external crates (necessary for TUI and randomness)
- Use `snake_case` for functions/variables, `PascalCase` for types
- Keep functions short; favor readability over cleverness
- Spanish-language identifiers are acceptable if they improve clarity

## Skills

Always load and apply these skills when their descriptions match the task:
- `clean-code` — when improving or reviewing code quality
- `rust-best-practices` — when writing, reviewing, or refactoring Rust code
- `write-concisely` — when writing documentation humans will read
- `think` — for architecture decisions or complex tradeoffs
- `omarchy` — for desktop/window manager/system config edits
- `find-skills` — when the user asks to find/install new skills

## Commands

| Task | Command |
|------|---------|
| Build | `cargo build` |
| Run | `cargo run` |
| Test | `cargo test` |
| Lint | `cargo clippy` |
| Format | `cargo fmt` |

## Notes

- `origin/main` points to a different project (KWGT-hub). This repo is a new direction.
- No CI yet — unit tests live in `src/combat.rs` and `src/data.rs` under `#[cfg(test)]`.
- The game uses crossterm for terminal manipulation (screen clearing, colors, input).
- Raw mode is enabled for proper arrow key detection.
- Combat is turn-based: player chooses Attack, Skill, Defend, or Flee.
- Enemy encounters are random during exploration.
- Level-up system: gain XP from defeating enemies, level up increases stats.