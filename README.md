# persona-rpg

RPG por turnos en la terminal (TUI), inspirado en Persona 3 Portable.

## Ejecutar

```bash
cargo run
```

## Estructura

- `src/main.rs` — entrada, bucle del juego, máquina de estados
- `src/data.rs` — personaje, enemigos, skills, estados
- `src/combat.rs` — combate por turnos, daño, subida de nivel
- `src/ui.rs` — render con crossterm

## Dependencias

- `crossterm` — terminal I/O
- `rand` — encuentros y daño

## Tests

```bash
cargo test
```
