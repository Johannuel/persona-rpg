use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, ClearType},
    QueueableCommand,
};
use std::io::{self, Write};
use std::sync::OnceLock;

use crate::data::{fusionar, CartaShuffle, Efecto, Elemento, EstadoCombate, Personaje, Skill};

const ANCHO: usize = 60;
const ANCHO_INTERIOR: usize = ANCHO - 2;
const ANCHO_TOTAL: usize = ANCHO + 2;

fn margen_horizontal() -> usize {
    static MARGEN: OnceLock<usize> = OnceLock::new();
    *MARGEN.get_or_init(|| {
        let (ancho, _) = terminal::size().unwrap_or((80, 24));
        (ancho as usize).saturating_sub(ANCHO_TOTAL) / 2
    })
}

fn texto_margen(linea: &str) -> String {
    format!("{}{}", " ".repeat(margen_horizontal()), linea)
}

fn mover_contenido(stdout: &mut (impl Write + QueueableCommand), lineas: usize) {
    let (_, alto) = terminal::size().unwrap_or((80, 24));
    let y = (alto as usize).saturating_sub(lineas) / 2;
    let _ = stdout.queue(cursor::MoveTo(0, y as u16));
}

fn caja_superior() -> String {
    texto_margen(&format!("┌{}┐", "─".repeat(ANCHO)))
}

fn caja_medio() -> String {
    texto_margen(&format!("├{}┤", "─".repeat(ANCHO)))
}

fn caja_inferior() -> String {
    texto_margen(&format!("└{}┘", "─".repeat(ANCHO)))
}

fn caja_fila(texto: &str) -> String {
    texto_margen(&format!("│ {:<width$} │", texto, width = ANCHO_INTERIOR))
}

fn fila_sprite(fila: &str, extra: &str) -> String {
    texto_margen(&format!(
        "│ {:<width$} │",
        format!("{}  {}", fila, extra),
        width = ANCHO_INTERIOR
    ))
}

pub fn limpiar_pantalla() {
    let mut stdout = io::stdout();
    execute!(stdout, terminal::Clear(ClearType::All)).unwrap();
    execute!(stdout, cursor::MoveTo(0, 0)).unwrap();
}

pub fn ocultar_cursor() {
    execute!(io::stdout(), cursor::Hide).unwrap();
}

pub fn mostrar_cursor() {
    execute!(io::stdout(), cursor::Show).unwrap();
}

fn color_hp(porcentaje: f32) -> Color {
    if porcentaje > 0.6 {
        Color::Green
    } else if porcentaje > 0.3 {
        Color::Yellow
    } else {
        Color::Red
    }
}

fn color_mp(porcentaje: f32) -> Color {
    if porcentaje > 0.5 {
        Color::Blue
    } else {
        Color::DarkBlue
    }
}

fn sprite_art(filas: [&str; 5]) -> Vec<String> {
    filas.iter().map(|f| format!("{:<22}", f)).collect()
}

fn sprite_color_enemigo(nombre: &str) -> Color {
    match nombre.split(" Lv.").next().unwrap_or(nombre) {
        "Jack Frost" => Color::Cyan,
        "Pyro Jack" => Color::Red,
        "Pixie" => Color::Magenta,
        "Cowardly Maya" => Color::DarkGrey,
        "Belligerent Maya" => Color::Red,
        "Laughing Table" => Color::DarkYellow,
        "Black Raven" => Color::DarkGrey,
        "Maniac Book" => Color::DarkMagenta,
        "Naga" => Color::Green,
        "Succubus" => Color::Magenta,
        "Chimera" => Color::Yellow,
        "Weeping Tiara" => Color::Cyan,
        "Lilim" => Color::Yellow,
        _ => Color::DarkGrey,
    }
}

fn sprite_color_persona(persona: &str) -> Color {
    match persona {
        "Jack Frost" => Color::Cyan,
        "Pyro Jack" => Color::Red,
        "Pixie" => Color::Magenta,
        _ => Color::White,
    }
}

fn sprite_enemigo(nombre: &str) -> Vec<String> {
    let base = nombre.split(" Lv.").next().unwrap_or(nombre);
    match base {
        "Jack Frost" => sprite_art([
            "       /\\",
            "      /--\\",
            "     |o  o |",
            "     |  ⌄  |",
            "      \\___/",
        ]),
        "Pyro Jack" => sprite_art([
            "      /\\",
            "     /--\\",
            "    |o o o|",
            "    |_^^^_|",
            "     /|||\\",
        ]),
        "Pixie" => sprite_art([
            "    /\\    /\\",
            "   ( o ) ( o )",
            "    \\  ⌄  /",
            "   (_______)",
            "     |  |",
        ]),
        "Cowardly Maya" => sprite_art([
            "   .-------.",
            "  (         )",
            "  (    ?    )",
            "   '-------'",
            "    /  |  \\",
        ]),
        "Belligerent Maya" => sprite_art([
            "   .-------.",
            "  (         )",
            "  (    !    )",
            "   '-------'",
            "    /  |  \\",
        ]),
        "Laughing Table" => sprite_art([
            "  __________",
            " |  o    o  |",
            " | ⌄⌄  ⌄⌄⌄  |",
            " |__________|",
            "   |      |",
        ]),
        "Black Raven" => sprite_art([
            "    ___",
            "   (o o)>",
            "    \\ ⌄ /",
            "    |_|_|",
            "   /  |  \\",
        ]),
        "Maniac Book" => sprite_art([
            "  ________",
            " | o    o |",
            " |   ⌄⌄   |",
            " |________|",
            "   ||   ||",
        ]),
        "Naga" => sprite_art([
            "  __(  )__",
            " (  o  o  )",
            " (   ⌄⌄   )",
            " (________)",
            "   /|  |\\",
        ]),
        "Succubus" => sprite_art([
            "    /\\    /\\",
            "   ( o  o )",
            "   (  ⌄⌄  )",
            "  /|_   _|\\",
            " _/       \\_",
        ]),
        "Chimera" => sprite_art([
            "     .--.",
            "    (o  o)",
            "    _⌄⌄⌄⌄_",
            "   /|  |  |\\",
            "  / |__|__| \\",
        ]),
        "Weeping Tiara" => sprite_art([
            "   /\\  /\\  /\\",
            "  ( o    o )",
            "   \\  ⌄⌄  /",
            "  (  ~ ~  )",
            "  (_______)",
        ]),
        "Lilim" => sprite_art([
            "  (\\/)   (\\/)",
            "   o     o",
            "  (  ⌄⌄  )",
            "  (_______)",
            "    / | \\",
        ]),
        _ => sprite_art([
            "    .---.",
            "   / o  o \\",
            "   |  ⌄⌄  |",
            "   |_____|",
            "    / | \\",
        ]),
    }
}

fn sprite_persona(persona: &str) -> Vec<String> {
    match persona {
        "Jack Frost" => sprite_enemigo("Jack Frost"),
        "Pyro Jack" => sprite_enemigo("Pyro Jack"),
        "Pixie" => sprite_enemigo("Pixie"),
        "Orpheus" => sprite_art([
            "  \\  /\\  /    ",
            "   \\/  \\/     ",
            "   / o  o \\   ",
            "   \\  ^  /    ",
            "    \\__/      ",
        ]),
        _ => sprite_art([
            "    .---.     ",
            "   / o  o \\   ",
            "  |   ~~~   | ",
            "   \\_____/   ",
            "  _/     \\_  ",
        ]),
    }
}

fn etiquetas(elementos: &[Elemento]) -> String {
    if elementos.is_empty() {
        String::from("—")
    } else {
        elementos
            .iter()
            .map(|e| e.etiqueta())
            .collect::<Vec<_>>()
            .join(", ")
    }
}

fn formato_habilidad(hab: &Skill) -> String {
    match hab.efecto {
        Efecto::Danio => format!("{} x{}", hab.elemento.etiqueta(), hab.multiplicador_dano),
        Efecto::Curacion => format!("Heals {}% HP", (hab.multiplicador_dano * 100.0) as u32),
        Efecto::BuffAtaque => String::from("Raises your attack"),
        Efecto::BuffDefensa => String::from("Raises your defense"),
        Efecto::DebuffAtaque => String::from("Lowers the enemy's attack"),
        Efecto::DebuffDefensa => String::from("Lowers the enemy's defense"),
    }
}

fn colorear_registro(linea: &str) -> (Color, String) {
    if let Some(resto) = linea.strip_prefix('[').and_then(|r| r.split_once(']')) {
        let color = match resto.0 {
            "Fire" => Color::Red,
            "Ice" => Color::Cyan,
            "Wind" => Color::Green,
            "Electric" => Color::Yellow,
            _ => Color::White,
        };
        return (color, resto.1.trim_start().to_string());
    }
    if linea.contains("Critical") {
        (Color::Yellow, linea.to_string())
    } else if linea.contains("recovers") {
        (Color::Green, linea.to_string())
    } else if linea.contains("resists") {
        (Color::DarkGrey, linea.to_string())
    } else {
        (Color::White, linea.to_string())
    }
}

fn barra_str(actual: u32, maximo: u32, ancho: usize) -> String {
    let porcentaje = if maximo > 0 {
        actual as f32 / maximo as f32
    } else {
        0.0
    };
    let llenos = (porcentaje * ancho as f32).round() as usize;
    let vacios = ancho.saturating_sub(llenos);
    let mut resultado = String::with_capacity(ancho + 2);
    resultado.push('[');
    for _ in 0..llenos {
        resultado.push('█');
    }
    for _ in 0..vacios {
        resultado.push('░');
    }
    resultado.push(']');
    resultado
}

fn resumen_personaje(personaje: &Personaje) -> String {
    format!(
        "{} · {} · HP {} MP {} ATQ {} DEF {}",
        personaje.nombre,
        personaje.persona,
        personaje.hp_max,
        personaje.mp_max,
        personaje.ataque,
        personaje.defensa
    )
}

const LUNA: [&str; 6] = [
    "    .-''-.     ",
    "   /  ..  \\    ",
    "  |  .  .  |   ",
    "  | .    . |   ",
    "   \\  ..  /    ",
    "    '-..-'     ",
];

const TORRE: [&str; 6] = [
    "     ______________",
    "    |   __   __   |",
    "    |  |__| |__|  |",
    "    |     []      |",
    "    |__[]_____[]__|",
    "       |      |   ",
];

fn escena_nocturna(stdout: &mut (impl Write + QueueableCommand)) {
    for i in 0..6 {
        let _ = stdout.queue(SetForegroundColor(Color::Yellow));
        let _ = stdout.queue(Print(&format!(
            "{}{}",
            " ".repeat(margen_horizontal()),
            LUNA[i]
        )));
        let _ = stdout.queue(SetForegroundColor(Color::DarkGrey));
        let _ = stdout.queue(Print(&format!("  {}\r\n", TORRE[i])));
        let _ = stdout.queue(ResetColor);
    }
}

fn luna_titulo(stdout: &mut (impl Write + QueueableCommand)) {
    let luna = [
        "        ______        ",
        "     .-'      '-.     ",
        "    /   .    .   \\    ",
        "   |   .      .   |   ",
        "   |  .        .  |   ",
        "    \\   .    .   /    ",
        "     '-.______.-'     ",
    ];
    let _ = stdout.queue(SetForegroundColor(Color::Yellow));
    for fila in luna {
        let _ = stdout.queue(Print(&format!(
            "{}{}\r\n",
            " ".repeat(margen_horizontal()),
            fila
        )));
    }
    let _ = stdout.queue(ResetColor);
}

pub fn render_titulo() {
    limpiar_pantalla();
    let mut stdout = io::stdout();

    mover_contenido(&mut stdout, 18);
    luna_titulo(&mut stdout);
    let _ = stdout.queue(Print("\r\n"));

    let _ = stdout.queue(SetForegroundColor(Color::Yellow));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_superior())));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_fila(""))));
    let _ = stdout.queue(Print(&format!(
        "{}\r\n",
        caja_fila("  *  *  *  P E R S O N A   R P G  *  *  *")
    )));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_fila(""))));
    let _ = stdout.queue(Print(&format!(
        "{}\r\n",
        caja_fila("  A turn-based RPG inspired by Persona 3")
    )));
    let _ = stdout.queue(Print(&format!(
        "{}\r\n",
        caja_fila("  The Dark Hour awaits you...")
    )));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_inferior())));
    let _ = stdout.queue(ResetColor);

    let _ = stdout.queue(Print("\r\n"));
    let _ = stdout.queue(SetForegroundColor(Color::Cyan));
    let _ = stdout.queue(Print(&format!(
        "{}\r\n",
        texto_margen("Press any key to start")
    )));
    let _ = stdout.queue(Print(&format!("{}\r\n", texto_margen("or 'q' to quit"))));
    let _ = stdout.queue(ResetColor);
    let _ = stdout.flush();
}

pub fn render_seleccion_personaje(personajes: &[Personaje], seleccion: usize) {
    limpiar_pantalla();
    let mut stdout = io::stdout();

    mover_contenido(&mut stdout, 7 + personajes.len());
    let _ = stdout.queue(SetForegroundColor(Color::Cyan));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_superior())));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_fila(""))));
    let _ = stdout.queue(Print(&format!(
        "{}\r\n",
        caja_fila("  C H O O S E   C H A R A C T E R")
    )));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_medio())));
    let _ = stdout.queue(ResetColor);

    for (i, personaje) in personajes.iter().enumerate() {
        if i == seleccion {
            let _ = stdout.queue(SetForegroundColor(Color::Yellow));
            let _ = stdout.queue(SetBackgroundColor(Color::DarkGrey));
            let _ = stdout.queue(Print(&format!(
                "{}\r\n",
                caja_fila(&format!("> {}", resumen_personaje(personaje)))
            )));
            let _ = stdout.queue(ResetColor);
        } else {
            let _ = stdout.queue(Print(&format!(
                "{}\r\n",
                caja_fila(&format!("  {}", resumen_personaje(personaje)))
            )));
        }
    }

    let _ = stdout.queue(Print(&format!("{}\r\n", caja_medio())));
    let _ = stdout.queue(SetForegroundColor(Color::Green));
    let _ = stdout.queue(Print(&format!(
        "{}\r\n",
        caja_fila("↑/↓ Select   Enter Confirm   q Quit")
    )));
    let _ = stdout.queue(SetForegroundColor(Color::Cyan));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_inferior())));
    let _ = stdout.queue(ResetColor);
    let _ = stdout.flush();
}

pub fn render_seleccion_persona(personas: &[Personaje], seleccion: usize) {
    limpiar_pantalla();
    let mut stdout = io::stdout();

    mover_contenido(&mut stdout, 6 + 3 * personas.len());
    let _ = stdout.queue(SetForegroundColor(Color::Magenta));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_superior())));
    let _ = stdout.queue(Print(&format!(
        "{}\r\n",
        caja_fila("  C H A N G E   P E R S O N A")
    )));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_medio())));
    let _ = stdout.queue(ResetColor);

    for (i, persona) in personas.iter().enumerate() {
        let detalle = format!(
            "{} · {}  |  HP {}  MP {}  ATQ {}  DEF {}",
            persona.arcana.map(|a| a.etiqueta()).unwrap_or("—"),
            persona.persona,
            persona.hp_max,
            persona.mp_max,
            persona.ataque,
            persona.defensa
        );
        if i == seleccion {
            let _ = stdout.queue(SetForegroundColor(Color::Yellow));
            let _ = stdout.queue(SetBackgroundColor(Color::DarkGrey));
            let _ = stdout.queue(Print(&format!(
                "{}\r\n",
                caja_fila(&format!("> {}", detalle))
            )));
            let _ = stdout.queue(ResetColor);
        } else {
            let _ = stdout.queue(Print(&format!(
                "{}\r\n",
                caja_fila(&format!("  {}", detalle))
            )));
        }
        let nombres_habilidades: Vec<&str> = persona
            .habilidades
            .iter()
            .map(|h| h.nombre.as_str())
            .collect();
        let _ = stdout.queue(Print(&format!(
            "{}\r\n",
            caja_fila(&format!("    Skills: {}", nombres_habilidades.join(", ")))
        )));
        let _ = stdout.queue(Print(&format!(
            "{}\r\n",
            caja_fila(&format!(
                "    Weak: {}  Resists: {}",
                etiquetas(&persona.debilidades),
                etiquetas(&persona.resistencias)
            ))
        )));
    }

    let _ = stdout.queue(Print(&format!("{}\r\n", caja_medio())));
    let _ = stdout.queue(SetForegroundColor(Color::Green));
    let _ = stdout.queue(Print(&format!(
        "{}\r\n",
        caja_fila("↑/↓ Select   Enter Change   Esc Back")
    )));
    let _ = stdout.queue(SetForegroundColor(Color::Magenta));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_inferior())));
    let _ = stdout.queue(ResetColor);
    let _ = stdout.flush();
}

pub fn render_exploracion(
    jugador: &Personaje,
    mensaje: &str,
    con_personas: bool,
    con_fusion: bool,
) {
    limpiar_pantalla();
    let mut stdout = io::stdout();

    mover_contenido(&mut stdout, 16);
    escena_nocturna(&mut stdout);

    let _ = stdout.queue(SetForegroundColor(Color::Cyan));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_superior())));
    let _ = stdout.queue(Print(&format!(
        "{}\r\n",
        caja_fila(&format!(
            "{}  ·  {}   |   Lv.{}   |   Exp: {}",
            jugador.nombre, jugador.persona, jugador.nivel, jugador.experiencia
        ))
    )));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_medio())));
    let _ = stdout.queue(ResetColor);

    let _ = stdout.queue(SetForegroundColor(color_hp(
        jugador.hp as f32 / jugador.hp_max as f32,
    )));
    let _ = stdout.queue(Print(&format!(
        "{}\r\n",
        caja_fila(&format!(
            "HP  {}  {}/{}",
            barra_str(jugador.hp, jugador.hp_max, 18),
            jugador.hp,
            jugador.hp_max
        ))
    )));
    let _ = stdout.queue(ResetColor);
    let _ = stdout.queue(SetForegroundColor(color_mp(
        jugador.mp as f32 / jugador.mp_max as f32,
    )));
    let _ = stdout.queue(Print(&format!(
        "{}\r\n",
        caja_fila(&format!(
            "MP  {}  {}/{}",
            barra_str(jugador.mp, jugador.mp_max, 18),
            jugador.mp,
            jugador.mp_max
        ))
    )));
    let _ = stdout.queue(ResetColor);

    let _ = stdout.queue(SetForegroundColor(Color::DarkGrey));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_medio())));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_fila(mensaje))));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_medio())));
    let _ = stdout.queue(ResetColor);

    let _ = stdout.queue(SetForegroundColor(Color::Green));
    if con_personas {
        if con_fusion {
            let _ = stdout.queue(Print(&format!(
                "{}\r\n",
                caja_fila("[1] Advance   [2] Rest   [3] Persona   [4] Fuse")
            )));
        } else {
            let _ = stdout.queue(Print(&format!(
                "{}\r\n",
                caja_fila("[1] Advance   [2] Rest   [3] Persona   [q] Quit")
            )));
        }
    } else {
        let _ = stdout.queue(Print(&format!(
            "{}\r\n",
            caja_fila("[1] Advance   [2] Rest   [q] Quit")
        )));
    }
    let _ = stdout.queue(SetForegroundColor(Color::Cyan));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_inferior())));
    let _ = stdout.queue(ResetColor);
    let _ = stdout.flush();
}

pub fn render_combate(estado: &EstadoCombate) {
    limpiar_pantalla();
    let mut stdout = io::stdout();

    mover_contenido(
        &mut stdout,
        25 + estado.registro.len().min(5) + estado.opciones.len(),
    );
    let _ = stdout.queue(SetForegroundColor(Color::Red));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_superior())));
    let _ = stdout.queue(Print(&format!(
        "{}\r\n",
        caja_fila("  = = =   B A T T L E   = = =")
    )));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_medio())));
    let _ = stdout.queue(ResetColor);

    let enemigo = &estado.enemigo.personaje;
    let _ = stdout.queue(SetForegroundColor(Color::Red));
    let _ = stdout.queue(Print(&format!(
        "{}\r\n",
        caja_fila(&format!(
            "  {}  (Lv.{})  —  {}",
            enemigo.nombre, enemigo.nivel, enemigo.persona
        ))
    )));
    let _ = stdout.queue(ResetColor);

    let sprite = sprite_enemigo(&enemigo.nombre);
    for (i, fila) in sprite.iter().enumerate() {
        match i {
            1 => {
                let extra = format!(
                    "HP {}  {}/{}",
                    barra_str(enemigo.hp, enemigo.hp_max, 14),
                    enemigo.hp,
                    enemigo.hp_max
                );
                let _ = stdout.queue(SetForegroundColor(color_hp(
                    enemigo.hp as f32 / enemigo.hp_max as f32,
                )));
                let _ = stdout.queue(Print(&format!("{}\r\n", fila_sprite(fila, &extra))));
                let _ = stdout.queue(ResetColor);
            }
            2 => {
                let extra = format!(
                    "Weak: {}  Resists: {}",
                    etiquetas(&enemigo.debilidades),
                    etiquetas(&enemigo.resistencias)
                );
                let _ = stdout.queue(Print(&format!("{}\r\n", fila_sprite(fila, &extra))));
            }
            _ => {
                let _ = stdout.queue(Print(&format!("{}\r\n", fila_sprite(fila, ""))));
            }
        }
    }

    let _ = stdout.queue(Print(&format!("{}\r\n", caja_medio())));

    let jugador = &estado.jugador;
    let _ = stdout.queue(SetForegroundColor(Color::Cyan));
    let _ = stdout.queue(Print(&format!(
        "{}\r\n",
        caja_fila(&format!(
            "  {}  (Lv.{})  —  {}",
            jugador.nombre, jugador.nivel, jugador.persona
        ))
    )));
    let _ = stdout.queue(ResetColor);

    let sprite_jugador = sprite_persona(&jugador.persona);
    for (i, fila) in sprite_jugador.iter().enumerate() {
        match i {
            1 => {
                let extra = format!(
                    "HP {}  {}/{}",
                    barra_str(jugador.hp, jugador.hp_max, 14),
                    jugador.hp,
                    jugador.hp_max
                );
                let _ = stdout.queue(SetForegroundColor(color_hp(
                    jugador.hp as f32 / jugador.hp_max as f32,
                )));
                let _ = stdout.queue(Print(&format!("{}\r\n", fila_sprite(fila, &extra))));
                let _ = stdout.queue(ResetColor);
            }
            2 => {
                let extra = format!(
                    "MP {}  {}/{}",
                    barra_str(jugador.mp, jugador.mp_max, 14),
                    jugador.mp,
                    jugador.mp_max
                );
                let _ = stdout.queue(SetForegroundColor(color_mp(
                    jugador.mp as f32 / jugador.mp_max as f32,
                )));
                let _ = stdout.queue(Print(&format!("{}\r\n", fila_sprite(fila, &extra))));
                let _ = stdout.queue(ResetColor);
            }
            3 => {
                let extra = format!(
                    "Weak: {}  Resists: {}",
                    etiquetas(&jugador.debilidades),
                    etiquetas(&jugador.resistencias)
                );
                let _ = stdout.queue(Print(&format!("{}\r\n", fila_sprite(fila, &extra))));
            }
            _ => {
                let _ = stdout.queue(Print(&format!("{}\r\n", fila_sprite(fila, ""))));
            }
        }
    }

    let _ = stdout.queue(Print(&format!("{}\r\n", caja_medio())));
    let _ = stdout.queue(SetForegroundColor(Color::DarkGrey));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_fila("  ── Log ──"))));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_medio())));
    let _ = stdout.queue(ResetColor);

    let inicio = if estado.registro.len() > 5 {
        estado.registro.len() - 5
    } else {
        0
    };
    for linea in &estado.registro[inicio..] {
        let (color, texto) = colorear_registro(linea);
        let _ = stdout.queue(SetForegroundColor(color));
        let _ = stdout.queue(Print(&format!("{}\r\n", caja_fila(&texto))));
        let _ = stdout.queue(ResetColor);
    }

    let _ = stdout.queue(Print(&format!("{}\r\n", caja_medio())));
    let _ = stdout.queue(SetForegroundColor(Color::Yellow));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_fila("  ── Action ──"))));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_medio())));
    let _ = stdout.queue(ResetColor);

    for (i, opcion) in estado.opciones.iter().enumerate() {
        if i == estado.seleccion_actual {
            let _ = stdout.queue(SetForegroundColor(Color::Yellow));
            let _ = stdout.queue(SetBackgroundColor(Color::DarkGrey));
            let _ = stdout.queue(Print(&format!(
                "{}\r\n",
                caja_fila(&format!("  > {} <", opcion))
            )));
            let _ = stdout.queue(ResetColor);
        } else {
            let _ = stdout.queue(Print(&format!(
                "{}\r\n",
                caja_fila(&format!("    {}", opcion))
            )));
        }
    }

    let _ = stdout.queue(Print(&format!("{}\r\n", caja_medio())));
    let _ = stdout.queue(SetForegroundColor(Color::DarkGrey));
    let _ = stdout.queue(Print(&format!(
        "{}\r\n",
        caja_fila("↑/↓ move   Enter choose   q surrender")
    )));
    let _ = stdout.queue(SetForegroundColor(Color::Red));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_inferior())));
    let _ = stdout.queue(ResetColor);
    let _ = stdout.flush();
}

pub fn render_habilidades(jugador: &Personaje, seleccion: usize) {
    limpiar_pantalla();
    let mut stdout = io::stdout();

    mover_contenido(&mut stdout, 6 + 2 * jugador.habilidades.len());
    let _ = stdout.queue(SetForegroundColor(Color::Magenta));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_superior())));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_fila("  S K I L L S"))));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_medio())));
    let _ = stdout.queue(ResetColor);

    for (i, hab) in jugador.habilidades.iter().enumerate() {
        let texto = format!(
            "  {}  [{} MP]  {}",
            hab.nombre,
            hab.coste_mp,
            formato_habilidad(hab)
        );
        if i == seleccion {
            let _ = stdout.queue(SetForegroundColor(Color::Yellow));
            let _ = stdout.queue(SetBackgroundColor(Color::DarkGrey));
            let _ = stdout.queue(Print(&format!(
                "{}\r\n",
                caja_fila(&format!("> {}", texto))
            )));
            let _ = stdout.queue(ResetColor);
        } else {
            let _ = stdout.queue(Print(&format!("{}\r\n", caja_fila(&texto))));
        }
        let _ = stdout.queue(Print(&format!(
            "{}\r\n",
            caja_fila(&format!("      {}", hab.descripcion))
        )));
    }

    let _ = stdout.queue(Print(&format!("{}\r\n", caja_medio())));
    let _ = stdout.queue(SetForegroundColor(Color::Green));
    let _ = stdout.queue(Print(&format!(
        "{}\r\n",
        caja_fila("↑/↓ Select   Enter Use   Esc Back")
    )));
    let _ = stdout.queue(SetForegroundColor(Color::Magenta));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_inferior())));
    let _ = stdout.queue(ResetColor);
    let _ = stdout.flush();
}

fn recortar(texto: &str, ancho: usize) -> String {
    texto.chars().take(ancho).collect()
}

fn tarjeta_shuffle(carta: &CartaShuffle) -> [String; 4] {
    let (titulo, detalle) = match carta {
        CartaShuffle::Persona(persona) => {
            let arcana = persona.arcana.map(|a| a.etiqueta()).unwrap_or("—");
            (
                persona.persona.clone(),
                format!("{} · Nv.{}", arcana, persona.nivel),
            )
        }
        CartaShuffle::Copa => (String::from("CUP"), String::from("40% HP · +15 MP")),
        CartaShuffle::Vara => (String::from("ROD"), String::from("+25 EXP")),
        CartaShuffle::Vacia => (String::from("EMPTY"), String::from("no reward")),
    };
    let ancho = 16;
    [
        format!("┌{0:─<16}┐", ""),
        format!("│{:<16}│", recortar(&titulo, ancho)),
        format!("│{:<16}│", recortar(&detalle, ancho)),
        format!("└{}┘", "─".repeat(ancho)),
    ]
}

pub fn render_shuffle_time(cartas: &[CartaShuffle], seleccion: usize) {
    limpiar_pantalla();
    let mut stdout = io::stdout();

    mover_contenido(&mut stdout, 12);
    let _ = stdout.queue(SetForegroundColor(Color::Yellow));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_superior())));
    let _ = stdout.queue(Print(&format!(
        "{}\r\n",
        caja_fila("  S H U F F L E   T I M E")
    )));
    let _ = stdout.queue(Print(&format!(
        "{}\r\n",
        caja_fila("  The cards of fate reveal themselves...")
    )));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_medio())));
    let _ = stdout.queue(ResetColor);

    let tarjetas: Vec<[String; 4]> = cartas.iter().map(tarjeta_shuffle).collect();
    for fila in 0..4 {
        let mut linea = String::new();
        for (i, tarjeta) in tarjetas.iter().enumerate() {
            if i == seleccion {
                linea.push_str(&format!("▶ {} ", tarjeta[fila]));
            } else {
                linea.push_str(&format!("   {} ", tarjeta[fila]));
            }
        }
        let _ = stdout.queue(Print(&format!("{}\r\n", caja_fila(&linea))));
    }
    let mut marca = String::new();
    for i in 0..cartas.len() {
        if i == seleccion {
            marca.push_str("        ▲         ");
        } else {
            marca.push_str("                   ");
        }
    }
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_fila(&marca))));

    let _ = stdout.queue(Print(&format!("{}\r\n", caja_medio())));
    let _ = stdout.queue(SetForegroundColor(Color::Green));
    let _ = stdout.queue(Print(&format!(
        "{}\r\n",
        caja_fila("↑/↓ Choose   Enter Take the card")
    )));
    let _ = stdout.queue(SetForegroundColor(Color::Yellow));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_inferior())));
    let _ = stdout.queue(ResetColor);
    let _ = stdout.flush();
}

pub fn render_fusion(stock: &[Personaje], fase: usize, seleccion: usize, fusion_a: Option<usize>) {
    limpiar_pantalla();
    let mut stdout = io::stdout();

    let altura = if fase == 1 && fusion_a.is_some() {
        10 + stock.len()
    } else {
        7 + stock.len()
    };
    mover_contenido(&mut stdout, altura);
    let _ = stdout.queue(SetForegroundColor(Color::Magenta));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_superior())));
    let _ = stdout.queue(Print(&format!(
        "{}\r\n",
        caja_fila("  V E L V E T   R O O M")
    )));
    let _ = stdout.queue(Print(&format!(
        "{}\r\n",
        caja_fila("  Elizabeth: which Personas shall we fuse today?")
    )));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_medio())));
    let _ = stdout.queue(ResetColor);

    for (i, persona) in stock.iter().enumerate() {
        let marcador = if Some(i) == fusion_a {
            "  [1st]"
        } else if fase == 1 && i == seleccion {
            "  [2nd]"
        } else {
            ""
        };
        let detalle = format!(
            "{} · {} Nv.{}  |  HP {} MP {} ATQ {} DEF {}{}",
            persona.arcana.map(|a| a.etiqueta()).unwrap_or("—"),
            persona.persona,
            persona.nivel,
            persona.hp_max,
            persona.mp_max,
            persona.ataque,
            persona.defensa,
            marcador
        );
        if i == seleccion {
            let _ = stdout.queue(SetForegroundColor(Color::Yellow));
            let _ = stdout.queue(SetBackgroundColor(Color::DarkGrey));
            let _ = stdout.queue(Print(&format!(
                "{}\r\n",
                caja_fila(&format!("> {}", detalle))
            )));
            let _ = stdout.queue(ResetColor);
        } else {
            let _ = stdout.queue(Print(&format!(
                "{}\r\n",
                caja_fila(&format!("  {}", detalle))
            )));
        }
    }

    if fase == 1 {
        if let Some(a) = fusion_a {
            if a != seleccion && seleccion < stock.len() {
                let resultado = fusionar(&stock[a], &stock[seleccion]);
                let arcana = resultado.arcana.map(|x| x.etiqueta()).unwrap_or("—");
                let nombres: Vec<&str> = resultado
                    .habilidades
                    .iter()
                    .map(|h| h.nombre.as_str())
                    .collect();
                let _ = stdout.queue(SetForegroundColor(Color::Cyan));
                let _ = stdout.queue(Print(&format!("{}\r\n", caja_medio())));
                let _ = stdout.queue(Print(&format!(
                    "{}\r\n",
                    caja_fila(&format!(
                        "Result: {} · {} Lv.{}",
                        arcana, resultado.persona, resultado.nivel
                    ))
                )));
                let _ = stdout.queue(Print(&format!(
                    "{}\r\n",
                    caja_fila(&format!("Skills: {}", nombres.join(", ")))
                )));
                let _ = stdout.queue(ResetColor);
            }
        }
    }

    let _ = stdout.queue(Print(&format!("{}\r\n", caja_medio())));
    let _ = stdout.queue(SetForegroundColor(Color::Green));
    let indicaciones = if fase == 0 {
        "↑/↓ Choose 1st   Enter Confirm   Esc Exit"
    } else {
        "↑/↓ Choose 2nd   Enter Fuse   Esc Cancel"
    };
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_fila(indicaciones))));
    let _ = stdout.queue(SetForegroundColor(Color::Magenta));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_inferior())));
    let _ = stdout.queue(ResetColor);
    let _ = stdout.flush();
}

pub fn render_game_over() {
    limpiar_pantalla();
    let mut stdout = io::stdout();

    mover_contenido(&mut stdout, 12);
    let calavera = [
        "    .--------.    ",
        "   /  o    o  \\   ",
        "  |    ----    |  ",
        "   \\    --    /   ",
        "    '--------'    ",
    ];
    let _ = stdout.queue(SetForegroundColor(Color::Red));
    for fila in calavera {
        let _ = stdout.queue(Print(&format!("{}\r\n", texto_margen(fila))));
    }
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_superior())));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_fila("  G A M E   O V E R"))));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_medio())));
    let _ = stdout.queue(Print(&format!(
        "{}\r\n",
        caja_fila("The Dark Hour has claimed you...")
    )));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_inferior())));
    let _ = stdout.queue(ResetColor);
    let _ = stdout.queue(Print("\n"));
    let _ = stdout.queue(Print(&format!(
        "{}\r\n",
        texto_margen("Press any key to exit...")
    )));
    let _ = stdout.flush();
}

pub fn leer_tecla() -> Option<KeyCode> {
    loop {
        if event::poll(std::time::Duration::from_millis(50)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                if key.kind == KeyEventKind::Press {
                    return Some(key.code);
                }
            }
        }
    }
}
