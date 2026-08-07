<div align="center">

# 🎮 persona-rpg

*A turn-based RPG for your terminal, inspired by Persona 3 Portable*

[![Rust](https://img.shields.io/badge/rust-1.81%2B-orange?style=flat-square)](https://www.rust-lang.org)
[![crossterm](https://img.shields.io/badge/TUI-crossterm-blueviolet?style=flat-square)](https://crates.io/crates/crossterm)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue?style=flat-square)](LICENSE)

⭐ If you like this project, star it on GitHub!

[Demo](#-demo) • [Features](#features) • [Screenshots](#screenshots) • [Run](#run) • [Controls](#controls) • [Roadmap](#roadmap) • [Contributing](#contributing)

</div>

Fight shadows in Tartarus, collect Personas, fuse them in the Velvet Room, and survive the Dark Hour — all in a 60-column terminal window. No game engine, no GUI: just Rust, crossterm and rand.

## 📸 Demo

<p align="center">
  <img src="videos/demo.gif" alt="Game demo — turn-based combat, Persona fusion and Shuffle Time" width="100%">
</p>

## Features

- 🎭 **5 playable characters** — Makoto, Yukari, Junpei, Akihiko, Mitsuru, each with a signature Persona
- 🗡️ **16 collectible Personas** with their P3 arcana: Orpheus, Pixie, Jack Frost, Pyro Jack, Forneus, Hua Po...
- 💠 **Velvet Room fusion** — combine two Personas (P3 arcana chart, skill inheritance, level math)
- 🃏 **Shuffle Time** — after every victory, pick one of three cards: a new Persona, HP/MP recovery, or bonus EXP
- ⚡ **Elemental combat** — 5 elements; hit a weakness for a critical, resist to halve
- 👾 **13 Tartarus shadows** scaled to your level, plus a **Floor Boss**, with weaknesses, resistances, buffs and debuffs
- 🎨 **Persona 3 "Dark Hour" palette** — RGB colors for elements, arcana, party members, sprites and the night sky, all in one module
- 🎨 **ASCII-art sprites**, colored HP/MP bars, and a colored combat log
- 📈 **Leveling** — gain EXP, grow stats, up to level 20
- 🗼 **Tartarus floors** — descend 5 floors; the Floor Boss (HMS Guillotine) guards the top
- 💾 **Save game** — save to `save.bin` anytime and load it back from the title screen

## Screenshots

> Note: the ASCII art below predates the Persona 3 "Dark Hour" palette update — borders are now blue and elements/arcana/party members are color-coded.

Combat against a Pyro Jack shadow in Tartarus:

```text
┌────────────────────────────────────────────────────────────┐
│   = = =   B A T T L E   = = =                              │
├────────────────────────────────────────────────────────────┤
│   Pyro Jack Lv.1  (Lv.1)  —  Shadow                        │
│       /\                                                   │
│      /--\               HP [████████░░░░░░]  34/60          │
│     |o o o|             Weak: Ice  Resists: Fire           │
│     |_^^^_|                                                │
│      /|||\                                                 │
├────────────────────────────────────────────────────────────┤
│   Makoto  (Lv.1)  —  Orpheus                               │
│     .---.                                                  │
│    | o  o |             HP [███████████░░░]  80/100         │
│    | ==== |             MP [██████████████]  50/50          │
│    |  ⌄   |             Weak: Electric  Resists: —         │
│     \___/                                                  │
├────────────────────────────────────────────────────────────┤
│   ── Log ──                                                │
├────────────────────────────────────────────────────────────┤
│ The battle begins!                                         │
│ Makoto strikes for 13 damage!                              │
│ Pyro Jack Lv.1 uses Agi and deals 11 damage!               │
│ Makoto strikes for 13 damage!                              │
│ Pyro Jack Lv.1 uses Maragi and deals 9 damage!             │
├────────────────────────────────────────────────────────────┤
│   ── Action ──                                             │
├────────────────────────────────────────────────────────────┤
│   > Attack <                                               │
│     Skills                                                 │
│     Defend                                                 │
│     Flee                                                   │
├────────────────────────────────────────────────────────────┤
│ ↑/↓ move   Enter choose   q surrender                      │
└────────────────────────────────────────────────────────────┘
```

Title screen:

```text
                 ______
              .-'      '-.
             /   .    .   \
            |   .      .   |
            |  .        .  |
             \   .    .   /
              '-.______.-'

         ┌────────────────────────────────────────────────────────────┐
         │                                                            │
         │   *  *  *  P E R S O N A   R P G  *  *  *                  │
         │                                                            │
         │   A turn-based RPG inspired by Persona 3                   │
         │   The Dark Hour awaits you...                              │
         └────────────────────────────────────────────────────────────┘

         Press any key to start
         or 'q' to quit
```

## Run

Requires [Rust](https://www.rust-lang.org) 1.81+, a UTF-8 terminal with **truecolor (24-bit)** support, and `crossterm` + `rand` (pulled automatically). Tested on Linux.

```bash
cargo build --release    # compiles the game
cargo run                # starts it
```

Or run the tests to check the current build:

```bash
cargo test
```

Out of the box the game writes its save to `<project-dir>/save.bin`. Type `q` at any time to leave, `l` on the title screen to load a save, `s` while exploring to save.

## Controls

| Context | Key | Action |
|---|---|---|
| Menus | `↑` / `↓` | Move selection |
| Menus | `Enter` | Confirm |
| Exploration | `1` | Descend into Tartarus (shadow encounter; on the last floor, face the Boss) |
| Exploration | `2` | Rest: recover HP and MP |
| Exploration | `3` | Change Persona (Makoto only) |
| Exploration | `4` | Fusion: Velvet Room (Makoto only) |
| Exploration | `s` | Save the game to `save.bin` |
| Title | `l` | Load the saved game (if present) |
| Combat | `↑` / `↓` + `Enter` | Attack, skills, defend, flee |
| Anywhere | `q` | Quit |

## Structure

The whole game lives in six flat modules, one job each:

- `src/main.rs` — entry point, game loop, state machine
- `src/data.rs` — characters, personas, enemies, skills, fusion, shuffle cards, bosses
- `src/combat.rs` — turn-based combat, damage, leveling
- `src/paleta.rs` — Persona 3 "Dark Hour" color palette (RGB colors, elements, arcana, sprites)
- `src/ui.rs` — crossterm rendering (title, exploration, combat, shuffle, fusion, victory)
- `src/save.rs` — save and load the game to/from disk

## Tests

```bash
cargo test
```

26 unit tests covering combat, elemental damage, leveling, fusion, shuffle cards, bosses and save/load.

## Roadmap

- [x] Turn-based combat with elemental weaknesses
- [x] Persona fusion (Velvet Room) and Shuffle Time rewards
- [x] Tartarus floors with a boss
- [x] Save game
- [x] Persona 3 "Dark Hour" color palette
- [ ] Rewrite the TUI with Ratatui (crossterm stays as the backend)
- [ ] Full moon events
- [ ] Bug fixes and cleanups tracked in the [issue tracker](https://github.com/Johannuel/persona-rpg/issues)

## Contributing

Found a bug or want a feature? Open an [issue](https://github.com/Johannuel/persona-rpg/issues). Small, well-tested PRs are welcome — run `cargo test` and `cargo clippy` first. See `AGENTS.md` for the project's coding conventions.
