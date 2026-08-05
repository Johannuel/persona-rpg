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

use crate::data::{
    fusionar, CartaShuffle, Efecto, Elemento, EstadoCombate, Personaje, Skill, PISO_TOTAL,
};
use crate::paleta::{
    color_arcana, color_carta, color_elemento, color_hp, color_mp, color_personaje,
    color_sprite_enemigo, color_sprite_persona, AZUL_MARCO, AZUL_SUAVE, AZUL_TITULO, BLANCO_SUAVE,
    CIELO_NOCTURNO, DORADO, ESTRELLA, FONDO_SELECCION, GRIS_SUAVE, LUNA_P3, ROJO_COMBATE,
    TEXTO_SELECCION, VERDE_OK, VIOLETA_VELVET,
};

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

/// Imprime una fila de la caja dividida en segmentos de colores distintos.
fn imprimir_fila_segmentos(
    stdout: &mut (impl Write + QueueableCommand),
    segmentos: &[(Color, String)],
) {
    let texto: String = segmentos.iter().map(|(_, t)| t.as_str()).collect();
    let relleno = ANCHO_INTERIOR.saturating_sub(texto.chars().count());
    let _ = stdout.queue(SetForegroundColor(AZUL_MARCO));
    let _ = stdout.queue(Print(&format!("{}{}", texto_margen("│ "), "")));
    for (color, texto) in segmentos {
        let _ = stdout.queue(SetForegroundColor(*color));
        let _ = stdout.queue(Print(texto));
    }
    let _ = stdout.queue(SetForegroundColor(AZUL_MARCO));
    let _ = stdout.queue(Print(&format!("{} │\r\n", " ".repeat(relleno))));
    let _ = stdout.queue(ResetColor);
}

/// Segmentos para una lista de elementos, cada uno con el color de su elemento.
fn segmentos_elementos(prefijo: &str, lista: &[Elemento]) -> Vec<(Color, String)> {
    let mut segs = vec![(BLANCO_SUAVE, prefijo.to_string())];
    if lista.is_empty() {
        segs.push((GRIS_SUAVE, "—".to_string()));
    } else {
        for (i, elemento) in lista.iter().enumerate() {
            if i > 0 {
                segs.push((BLANCO_SUAVE, ", ".to_string()));
            }
            segs.push((color_elemento(*elemento), elemento.etiqueta().to_string()));
        }
    }
    segs
}

/// Segmentos de la fila "Weak: ...  Resists: ..." con los elementos coloreados.
fn segmentos_debilidad_resistencia(
    debilidades: &[Elemento],
    resistencias: &[Elemento],
) -> Vec<(Color, String)> {
    let mut segs = segmentos_elementos("Weak: ", debilidades);
    segs.push((BLANCO_SUAVE, "  Resists: ".to_string()));
    segs.extend(segmentos_elementos("", resistencias));
    segs
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

fn sprite_art(filas: [&str; 5]) -> Vec<String> {
    filas.iter().map(|f| format!("{:<22}", f)).collect()
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
        "Guillotine" => sprite_art([
            "    _  __  _",
            "   / \\/  \\/ \\",
            "  |   ⌄⌄   |",
            "  |  ⌄⌄⌄  |",
            "  |_     _|",
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
            "    .---.",
            "   | o  o |",
            "   | ==== |",
            "   |  ⌄   |",
            "    \\___/",
        ]),
        _ => sprite_art([
            "    .---.",
            "   / o  o \\",
            "  |   ~~~   |",
            "   \\_____/",
            "  _/     \\_",
        ]),
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
            "Fire" => color_elemento(Elemento::Fuego),
            "Ice" => color_elemento(Elemento::Hielo),
            "Wind" => color_elemento(Elemento::Viento),
            "Electric" => color_elemento(Elemento::Electrico),
            _ => BLANCO_SUAVE,
        };
        return (color, resto.1.trim_start().to_string());
    }
    if linea.contains("Critical") {
        (
            Color::Rgb {
                r: 255,
                g: 220,
                b: 96,
            },
            linea.to_string(),
        )
    } else if linea.contains("recovers") {
        (VERDE_OK, linea.to_string())
    } else if linea.contains("resists") {
        (GRIS_SUAVE, linea.to_string())
    } else {
        (BLANCO_SUAVE, linea.to_string())
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

const ESTRELLAS: [&str; 6] = [
    "✦      ·       ",
    "     ·    ✦    ",
    "  ✦     ·      ",
    "    ·     ✦    ",
    "✦      ·       ",
    "     ✦   ·     ",
];

fn escena_nocturna(stdout: &mut (impl Write + QueueableCommand)) {
    for i in 0..6 {
        let _ = stdout.queue(SetForegroundColor(LUNA_P3));
        let _ = stdout.queue(Print(&format!(
            "{}{}",
            " ".repeat(margen_horizontal()),
            LUNA[i]
        )));
        let _ = stdout.queue(SetForegroundColor(CIELO_NOCTURNO));
        let _ = stdout.queue(Print(&format!("  {}", TORRE[i])));
        let _ = stdout.queue(SetForegroundColor(ESTRELLA));
        let _ = stdout.queue(Print(&format!("{}\r\n", ESTRELLAS[i])));
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
    let _ = stdout.queue(SetForegroundColor(LUNA_P3));
    for fila in luna {
        let _ = stdout.queue(Print(&format!(
            "{}{}\r\n",
            " ".repeat(margen_horizontal()),
            fila
        )));
    }
    let _ = stdout.queue(ResetColor);
}

pub fn render_titulo(con_save: bool) {
    limpiar_pantalla();
    let mut stdout = io::stdout();

    let lineas = if con_save { 20 } else { 18 };
    mover_contenido(&mut stdout, lineas);
    luna_titulo(&mut stdout);
    let _ = stdout.queue(Print("\r\n"));

    let _ = stdout.queue(SetForegroundColor(AZUL_MARCO));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_superior())));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_fila(""))));
    let _ = stdout.queue(SetForegroundColor(AZUL_TITULO));
    let _ = stdout.queue(Print(&format!(
        "{}\r\n",
        caja_fila("  *  *  *  P E R S O N A   R P G  *  *  *")
    )));
    let _ = stdout.queue(SetForegroundColor(AZUL_MARCO));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_fila(""))));
    let _ = stdout.queue(SetForegroundColor(BLANCO_SUAVE));
    let _ = stdout.queue(Print(&format!(
        "{}\r\n",
        caja_fila("  A turn-based RPG inspired by Persona 3")
    )));
    let _ = stdout.queue(SetForegroundColor(AZUL_SUAVE));
    let _ = stdout.queue(Print(&format!(
        "{}\r\n",
        caja_fila("  The Dark Hour awaits you...")
    )));
    let _ = stdout.queue(SetForegroundColor(AZUL_MARCO));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_inferior())));
    let _ = stdout.queue(ResetColor);

    let _ = stdout.queue(Print("\r\n"));
    let _ = stdout.queue(SetForegroundColor(AZUL_TITULO));
    let _ = stdout.queue(Print(&format!(
        "{}\r\n",
        texto_margen("Press any key to start")
    )));
    if con_save {
        let _ = stdout.queue(SetForegroundColor(VERDE_OK));
        let _ = stdout.queue(Print(&format!(
            "{}\r\n",
            texto_margen("'l' to load your saved game")
        )));
    }
    let _ = stdout.queue(SetForegroundColor(GRIS_SUAVE));
    let _ = stdout.queue(Print(&format!("{}\r\n", texto_margen("or 'q' to quit"))));
    let _ = stdout.queue(ResetColor);
    let _ = stdout.flush();
}

pub fn render_seleccion_personaje(personajes: &[Personaje], seleccion: usize) {
    limpiar_pantalla();
    let mut stdout = io::stdout();

    mover_contenido(&mut stdout, 7 + personajes.len());
    let _ = stdout.queue(SetForegroundColor(AZUL_MARCO));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_superior())));
    let _ = stdout.queue(SetForegroundColor(AZUL_TITULO));
    let _ = stdout.queue(Print(&format!(
        "{}\r\n",
        caja_fila("  C H O O S E   C H A R A C T E R")
    )));
    let _ = stdout.queue(SetForegroundColor(AZUL_MARCO));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_medio())));
    let _ = stdout.queue(ResetColor);

    for (i, personaje) in personajes.iter().enumerate() {
        if i == seleccion {
            let _ = stdout.queue(SetForegroundColor(TEXTO_SELECCION));
            let _ = stdout.queue(SetBackgroundColor(FONDO_SELECCION));
            let _ = stdout.queue(Print(&format!(
                "{}\r\n",
                caja_fila(&format!("> {}", resumen_personaje(personaje)))
            )));
            let _ = stdout.queue(ResetColor);
        } else {
            let _ = stdout.queue(SetForegroundColor(color_personaje(&personaje.nombre)));
            let _ = stdout.queue(Print(&format!(
                "{}\r\n",
                caja_fila(&format!("  {}", resumen_personaje(personaje)))
            )));
            let _ = stdout.queue(ResetColor);
        }
    }

    let _ = stdout.queue(SetForegroundColor(AZUL_MARCO));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_medio())));
    let _ = stdout.queue(SetForegroundColor(VERDE_OK));
    let _ = stdout.queue(Print(&format!(
        "{}\r\n",
        caja_fila("↑/↓ Select   Enter Confirm   q Quit")
    )));
    let _ = stdout.queue(SetForegroundColor(AZUL_MARCO));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_inferior())));
    let _ = stdout.queue(ResetColor);
    let _ = stdout.flush();
}

pub fn render_seleccion_persona(personas: &[Personaje], seleccion: usize) {
    limpiar_pantalla();
    let mut stdout = io::stdout();

    mover_contenido(&mut stdout, 6 + 3 * personas.len());
    let _ = stdout.queue(SetForegroundColor(AZUL_MARCO));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_superior())));
    let _ = stdout.queue(SetForegroundColor(AZUL_TITULO));
    let _ = stdout.queue(Print(&format!(
        "{}\r\n",
        caja_fila("  C H A N G E   P E R S O N A")
    )));
    let _ = stdout.queue(SetForegroundColor(AZUL_MARCO));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_medio())));
    let _ = stdout.queue(ResetColor);

    for (i, persona) in personas.iter().enumerate() {
        if i == seleccion {
            let detalle = format!(
                "{} · {}  |  HP {}  MP {}  ATQ {}  DEF {}",
                persona.arcana.map(|a| a.etiqueta()).unwrap_or("—"),
                persona.persona,
                persona.hp_max,
                persona.mp_max,
                persona.ataque,
                persona.defensa
            );
            let _ = stdout.queue(SetForegroundColor(TEXTO_SELECCION));
            let _ = stdout.queue(SetBackgroundColor(FONDO_SELECCION));
            let _ = stdout.queue(Print(&format!(
                "{}\r\n",
                caja_fila(&format!("> {}", detalle))
            )));
            let _ = stdout.queue(ResetColor);
        } else {
            let mut segs: Vec<(Color, String)> = vec![(GRIS_SUAVE, "  ".to_string())];
            if let Some(arcana) = persona.arcana {
                segs.push((color_arcana(arcana), format!("{} · ", arcana.etiqueta())));
            }
            segs.push((
                color_sprite_persona(&persona.persona),
                persona.persona.clone(),
            ));
            segs.push((
                AZUL_SUAVE,
                format!(
                    "  |  HP {}  MP {}  ATQ {}  DEF {}",
                    persona.hp_max, persona.mp_max, persona.ataque, persona.defensa
                ),
            ));
            imprimir_fila_segmentos(&mut stdout, &segs);
        }
        let nombres_habilidades: Vec<&str> = persona
            .habilidades
            .iter()
            .map(|h| h.nombre.as_str())
            .collect();
        let _ = stdout.queue(SetForegroundColor(BLANCO_SUAVE));
        let _ = stdout.queue(Print(&format!(
            "{}\r\n",
            caja_fila(&format!("    Skills: {}", nombres_habilidades.join(", ")))
        )));
        let _ = stdout.queue(ResetColor);
        let mut segs = vec![(BLANCO_SUAVE, "    ".to_string())];
        segs.extend(segmentos_debilidad_resistencia(
            &persona.debilidades,
            &persona.resistencias,
        ));
        imprimir_fila_segmentos(&mut stdout, &segs);
    }

    let _ = stdout.queue(SetForegroundColor(AZUL_MARCO));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_medio())));
    let _ = stdout.queue(SetForegroundColor(VERDE_OK));
    let _ = stdout.queue(Print(&format!(
        "{}\r\n",
        caja_fila("↑/↓ Select   Enter Change   Esc Back")
    )));
    let _ = stdout.queue(SetForegroundColor(AZUL_MARCO));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_inferior())));
    let _ = stdout.queue(ResetColor);
    let _ = stdout.flush();
}

fn color_mensaje_exploracion(mensaje: &str) -> Color {
    if mensaje.contains("Couldn't") {
        ROJO_COMBATE
    } else if mensaje.contains("saved")
        || mensaje.contains("recover")
        || mensaje.contains("obtained")
        || mensaje.contains("awaken")
        || mensaje.contains("summon")
    {
        VERDE_OK
    } else if mensaje.contains("leveled up") || mensaje.contains("defeated") {
        DORADO
    } else if mensaje.contains("fled") {
        GRIS_SUAVE
    } else {
        BLANCO_SUAVE
    }
}

pub fn render_exploracion(
    jugador: &Personaje,
    mensaje: &str,
    piso: u32,
    con_personas: bool,
    con_fusion: bool,
) {
    limpiar_pantalla();
    let mut stdout = io::stdout();

    mover_contenido(&mut stdout, 17);
    escena_nocturna(&mut stdout);

    let _ = stdout.queue(SetForegroundColor(AZUL_MARCO));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_superior())));
    let _ = stdout.queue(SetForegroundColor(color_personaje(&jugador.nombre)));
    let _ = stdout.queue(Print(&format!(
        "{}\r\n",
        caja_fila(&format!(
            "{} · {}  |  Lv.{}  Exp: {}",
            jugador.nombre, jugador.persona, jugador.nivel, jugador.experiencia
        ))
    )));
    let _ = stdout.queue(SetForegroundColor(AZUL_SUAVE));
    let _ = stdout.queue(Print(&format!(
        "{}\r\n",
        caja_fila(&format!(
            "Location: Tartarus — Floor {}/{}",
            piso, PISO_TOTAL
        ))
    )));
    let _ = stdout.queue(SetForegroundColor(AZUL_MARCO));
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

    let _ = stdout.queue(SetForegroundColor(AZUL_MARCO));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_medio())));
    let _ = stdout.queue(SetForegroundColor(color_mensaje_exploracion(mensaje)));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_fila(mensaje))));
    let _ = stdout.queue(SetForegroundColor(AZUL_MARCO));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_medio())));
    let _ = stdout.queue(ResetColor);

    let _ = stdout.queue(SetForegroundColor(VERDE_OK));
    let avanzar = if piso >= PISO_TOTAL {
        "[1] Face the Boss"
    } else {
        "[1] Descend"
    };
    if con_personas {
        if con_fusion {
            let _ = stdout.queue(Print(&format!(
                "{}\r\n",
                caja_fila(&format!(
                    "{} [2] Rest [3] Persona [4] Fuse [s] Save",
                    avanzar
                ))
            )));
        } else {
            let _ = stdout.queue(Print(&format!(
                "{}\r\n",
                caja_fila(&format!("{} [2] Rest [3] Persona [s] Save", avanzar))
            )));
        }
    } else {
        let _ = stdout.queue(Print(&format!(
            "{}\r\n",
            caja_fila(&format!("{} [2] Rest [s] Save", avanzar))
        )));
    }
    let _ = stdout.queue(SetForegroundColor(GRIS_SUAVE));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_fila("press q to quit"))));
    let _ = stdout.queue(SetForegroundColor(AZUL_MARCO));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_inferior())));
    let _ = stdout.queue(ResetColor);
    let _ = stdout.flush();
}

pub fn render_combate(estado: &EstadoCombate) {
    limpiar_pantalla();
    let mut stdout = io::stdout();

    mover_contenido(
        &mut stdout,
        26 + estado.registro.len().min(5) + estado.opciones.len(),
    );
    let _ = stdout.queue(SetForegroundColor(AZUL_MARCO));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_superior())));
    let _ = stdout.queue(SetForegroundColor(ROJO_COMBATE));
    let _ = stdout.queue(Print(&format!(
        "{}\r\n",
        caja_fila("  = = =   B A T T L E   = = =")
    )));
    let _ = stdout.queue(SetForegroundColor(AZUL_MARCO));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_medio())));
    let _ = stdout.queue(ResetColor);

    let enemigo = &estado.enemigo.personaje;
    let _ = stdout.queue(SetForegroundColor(color_sprite_enemigo(&enemigo.nombre)));
    let marca_jefe = if enemigo.persona == "Floor Boss" {
        " ★ BOSS ★"
    } else {
        ""
    };
    let _ = stdout.queue(Print(&format!(
        "{}\r\n",
        caja_fila(&format!(
            "  {}  (Lv.{})  —  {}{}",
            enemigo.nombre, enemigo.nivel, enemigo.persona, marca_jefe
        ))
    )));
    let _ = stdout.queue(ResetColor);

    let color_sprite = color_sprite_enemigo(&enemigo.nombre);
    let sprite = sprite_enemigo(&enemigo.nombre);
    for (i, fila) in sprite.iter().enumerate() {
        if i == 2 {
            let mut segs = vec![(color_sprite, fila.clone())];
            segs.push((BLANCO_SUAVE, "  ".to_string()));
            segs.extend(segmentos_debilidad_resistencia(
                &enemigo.debilidades,
                &enemigo.resistencias,
            ));
            imprimir_fila_segmentos(&mut stdout, &segs);
        } else {
            let color_fila = if i == 1 {
                color_hp(enemigo.hp as f32 / enemigo.hp_max as f32)
            } else {
                color_sprite
            };
            let extra = if i == 1 {
                format!(
                    "HP {}  {}/{}",
                    barra_str(enemigo.hp, enemigo.hp_max, 14),
                    enemigo.hp,
                    enemigo.hp_max
                )
            } else {
                String::new()
            };
            let _ = stdout.queue(SetForegroundColor(color_fila));
            let _ = stdout.queue(Print(&format!("{}\r\n", fila_sprite(fila, &extra))));
            let _ = stdout.queue(ResetColor);
        }
    }

    let _ = stdout.queue(SetForegroundColor(AZUL_MARCO));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_medio())));
    let _ = stdout.queue(ResetColor);

    let jugador = &estado.jugador;
    let _ = stdout.queue(SetForegroundColor(color_personaje(&jugador.nombre)));
    let _ = stdout.queue(Print(&format!(
        "{}\r\n",
        caja_fila(&format!(
            "  {}  (Lv.{})  —  {}",
            jugador.nombre, jugador.nivel, jugador.persona
        ))
    )));
    let _ = stdout.queue(ResetColor);

    let color_sprite = color_sprite_persona(&jugador.persona);
    let sprite_jugador = sprite_persona(&jugador.persona);
    for (i, fila) in sprite_jugador.iter().enumerate() {
        if i == 3 {
            let mut segs = vec![(color_sprite, fila.clone())];
            segs.push((BLANCO_SUAVE, "  ".to_string()));
            segs.extend(segmentos_debilidad_resistencia(
                &jugador.debilidades,
                &jugador.resistencias,
            ));
            imprimir_fila_segmentos(&mut stdout, &segs);
        } else {
            let color_fila = match i {
                1 => color_hp(jugador.hp as f32 / jugador.hp_max as f32),
                2 => color_mp(jugador.mp as f32 / jugador.mp_max as f32),
                _ => color_sprite,
            };
            let extra = match i {
                1 => format!(
                    "HP {}  {}/{}",
                    barra_str(jugador.hp, jugador.hp_max, 14),
                    jugador.hp,
                    jugador.hp_max
                ),
                2 => format!(
                    "MP {}  {}/{}",
                    barra_str(jugador.mp, jugador.mp_max, 14),
                    jugador.mp,
                    jugador.mp_max
                ),
                _ => String::new(),
            };
            let _ = stdout.queue(SetForegroundColor(color_fila));
            let _ = stdout.queue(Print(&format!("{}\r\n", fila_sprite(fila, &extra))));
            let _ = stdout.queue(ResetColor);
        }
    }

    let _ = stdout.queue(SetForegroundColor(AZUL_MARCO));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_medio())));
    let _ = stdout.queue(SetForegroundColor(AZUL_SUAVE));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_fila("  ── Log ──"))));
    let _ = stdout.queue(SetForegroundColor(AZUL_MARCO));
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

    let _ = stdout.queue(SetForegroundColor(AZUL_MARCO));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_medio())));
    let _ = stdout.queue(SetForegroundColor(AZUL_TITULO));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_fila("  ── Action ──"))));
    let _ = stdout.queue(SetForegroundColor(AZUL_MARCO));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_medio())));
    let _ = stdout.queue(ResetColor);

    for (i, opcion) in estado.opciones.iter().enumerate() {
        if i == estado.seleccion_actual {
            let _ = stdout.queue(SetForegroundColor(TEXTO_SELECCION));
            let _ = stdout.queue(SetBackgroundColor(FONDO_SELECCION));
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

    let _ = stdout.queue(SetForegroundColor(AZUL_MARCO));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_medio())));
    let _ = stdout.queue(ResetColor);

    let mut leyenda = vec![(GRIS_SUAVE, "Elements: ".to_string())];
    leyenda.push((color_elemento(Elemento::Fuego), " Fire ".to_string()));
    leyenda.push((color_elemento(Elemento::Hielo), " Ice ".to_string()));
    leyenda.push((color_elemento(Elemento::Viento), " Wind ".to_string()));
    leyenda.push((color_elemento(Elemento::Electrico), " Electric".to_string()));
    imprimir_fila_segmentos(&mut stdout, &leyenda);

    let _ = stdout.queue(SetForegroundColor(GRIS_SUAVE));
    let _ = stdout.queue(Print(&format!(
        "{}\r\n",
        caja_fila("↑/↓ move   Enter choose   q surrender")
    )));
    let _ = stdout.queue(SetForegroundColor(ROJO_COMBATE));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_inferior())));
    let _ = stdout.queue(ResetColor);
    let _ = stdout.flush();
}

pub fn render_habilidades(jugador: &Personaje, seleccion: usize) {
    limpiar_pantalla();
    let mut stdout = io::stdout();

    mover_contenido(&mut stdout, 6 + 2 * jugador.habilidades.len());
    let _ = stdout.queue(SetForegroundColor(AZUL_MARCO));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_superior())));
    let _ = stdout.queue(SetForegroundColor(AZUL_TITULO));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_fila("  S K I L L S"))));
    let _ = stdout.queue(SetForegroundColor(AZUL_MARCO));
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
            let _ = stdout.queue(SetForegroundColor(TEXTO_SELECCION));
            let _ = stdout.queue(SetBackgroundColor(FONDO_SELECCION));
            let _ = stdout.queue(Print(&format!(
                "{}\r\n",
                caja_fila(&format!("> {}", texto))
            )));
            let _ = stdout.queue(ResetColor);
        } else {
            let mut segs = vec![(color_elemento(hab.elemento), format!("  {}", hab.nombre))];
            segs.push((AZUL_SUAVE, format!("  [{} MP]  ", hab.coste_mp)));
            segs.push((BLANCO_SUAVE, formato_habilidad(hab)));
            imprimir_fila_segmentos(&mut stdout, &segs);
        }
        let _ = stdout.queue(SetForegroundColor(GRIS_SUAVE));
        let _ = stdout.queue(Print(&format!(
            "{}\r\n",
            caja_fila(&format!("      {}", hab.descripcion))
        )));
        let _ = stdout.queue(ResetColor);
    }

    let _ = stdout.queue(SetForegroundColor(AZUL_MARCO));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_medio())));
    let _ = stdout.queue(SetForegroundColor(VERDE_OK));
    let _ = stdout.queue(Print(&format!(
        "{}\r\n",
        caja_fila("↑/↓ Select   Enter Use   Esc Back")
    )));
    let _ = stdout.queue(SetForegroundColor(AZUL_MARCO));
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
    let _ = stdout.queue(SetForegroundColor(AZUL_MARCO));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_superior())));
    let _ = stdout.queue(SetForegroundColor(AZUL_TITULO));
    let _ = stdout.queue(Print(&format!(
        "{}\r\n",
        caja_fila("  S H U F F L E   T I M E")
    )));
    let _ = stdout.queue(SetForegroundColor(BLANCO_SUAVE));
    let _ = stdout.queue(Print(&format!(
        "{}\r\n",
        caja_fila("  The cards of fate reveal themselves...")
    )));
    let _ = stdout.queue(SetForegroundColor(AZUL_MARCO));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_medio())));
    let _ = stdout.queue(ResetColor);

    let tarjetas: Vec<[String; 4]> = cartas.iter().map(tarjeta_shuffle).collect();
    for fila in 0..4 {
        let mut segs: Vec<(Color, String)> = Vec::new();
        for (i, tarjeta) in tarjetas.iter().enumerate() {
            let color_carta_i = color_carta(&cartas[i]);
            if i == seleccion {
                segs.push((AZUL_TITULO, "▶ ".to_string()));
            } else {
                segs.push((GRIS_SUAVE, "   ".to_string()));
            }
            segs.push((color_carta_i, tarjeta[fila].clone()));
            segs.push((BLANCO_SUAVE, " ".to_string()));
        }
        imprimir_fila_segmentos(&mut stdout, &segs);
    }
    let mut segs: Vec<(Color, String)> = Vec::new();
    for i in 0..cartas.len() {
        if i == seleccion {
            segs.push((AZUL_TITULO, "        ▲         ".to_string()));
        } else {
            segs.push((GRIS_SUAVE, "                   ".to_string()));
        }
    }
    imprimir_fila_segmentos(&mut stdout, &segs);

    let _ = stdout.queue(SetForegroundColor(AZUL_MARCO));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_medio())));
    let _ = stdout.queue(SetForegroundColor(VERDE_OK));
    let _ = stdout.queue(Print(&format!(
        "{}\r\n",
        caja_fila("↑/↓ Choose   Enter Take the card")
    )));
    let _ = stdout.queue(SetForegroundColor(DORADO));
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
    let _ = stdout.queue(SetForegroundColor(AZUL_MARCO));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_superior())));
    let _ = stdout.queue(SetForegroundColor(VIOLETA_VELVET));
    let _ = stdout.queue(Print(&format!(
        "{}\r\n",
        caja_fila("  V E L V E T   R O O M")
    )));
    let _ = stdout.queue(SetForegroundColor(BLANCO_SUAVE));
    let _ = stdout.queue(Print(&format!(
        "{}\r\n",
        caja_fila("  Elizabeth: which Personas shall we fuse today?")
    )));
    let _ = stdout.queue(SetForegroundColor(AZUL_MARCO));
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
        if i == seleccion {
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
            let _ = stdout.queue(SetForegroundColor(TEXTO_SELECCION));
            let _ = stdout.queue(SetBackgroundColor(FONDO_SELECCION));
            let _ = stdout.queue(Print(&format!(
                "{}\r\n",
                caja_fila(&format!("> {}", detalle))
            )));
            let _ = stdout.queue(ResetColor);
        } else {
            let mut segs: Vec<(Color, String)> = vec![(GRIS_SUAVE, "  ".to_string())];
            if let Some(arcana) = persona.arcana {
                segs.push((color_arcana(arcana), format!("{} · ", arcana.etiqueta())));
            }
            segs.push((BLANCO_SUAVE, persona.persona.clone()));
            segs.push((
                AZUL_SUAVE,
                format!(
                    " Nv.{}  |  HP {} MP {} ATQ {} DEF {}",
                    persona.nivel, persona.hp_max, persona.mp_max, persona.ataque, persona.defensa
                ),
            ));
            if !marcador.is_empty() {
                segs.push((DORADO, marcador.to_string()));
            }
            imprimir_fila_segmentos(&mut stdout, &segs);
        }
    }

    if fase == 1 {
        if let Some(a) = fusion_a {
            if a != seleccion && seleccion < stock.len() {
                let resultado = fusionar(&stock[a], &stock[seleccion]);
                let nombres: Vec<&str> = resultado
                    .habilidades
                    .iter()
                    .map(|h| h.nombre.as_str())
                    .collect();
                let _ = stdout.queue(SetForegroundColor(AZUL_MARCO));
                let _ = stdout.queue(Print(&format!("{}\r\n", caja_medio())));
                let _ = stdout.queue(ResetColor);
                let mut segs = vec![(AZUL_TITULO, "Result: ".to_string())];
                if let Some(arcana) = resultado.arcana {
                    segs.push((color_arcana(arcana), format!("{} · ", arcana.etiqueta())));
                }
                segs.push((
                    BLANCO_SUAVE,
                    format!("{} Lv.{}", resultado.persona, resultado.nivel),
                ));
                imprimir_fila_segmentos(&mut stdout, &segs);
                let _ = stdout.queue(SetForegroundColor(BLANCO_SUAVE));
                let _ = stdout.queue(Print(&format!(
                    "{}\r\n",
                    caja_fila(&format!("Skills: {}", nombres.join(", ")))
                )));
                let _ = stdout.queue(ResetColor);
            }
        }
    }

    let _ = stdout.queue(SetForegroundColor(AZUL_MARCO));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_medio())));
    let _ = stdout.queue(SetForegroundColor(VERDE_OK));
    let indicaciones = if fase == 0 {
        "↑/↓ Choose 1st   Enter Confirm   Esc Exit"
    } else {
        "↑/↓ Choose 2nd   Enter Fuse   Esc Cancel"
    };
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_fila(indicaciones))));
    let _ = stdout.queue(SetForegroundColor(VIOLETA_VELVET));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_inferior())));
    let _ = stdout.queue(ResetColor);
    let _ = stdout.flush();
}

pub fn render_game_over() {
    limpiar_pantalla();
    let mut stdout = io::stdout();

    mover_contenido(&mut stdout, 12);
    let calavera = [
        "     .-----.",
        "    / o   o \\",
        "   |   ⌄⌄   |",
        "   | |   | |",
        "   |_|___|_|",
    ];
    let _ = stdout.queue(SetForegroundColor(ROJO_COMBATE));
    for fila in calavera {
        let _ = stdout.queue(Print(&format!("{}\r\n", texto_margen(fila))));
    }
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_superior())));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_fila("  G A M E   O V E R"))));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_medio())));
    let _ = stdout.queue(SetForegroundColor(BLANCO_SUAVE));
    let _ = stdout.queue(Print(&format!(
        "{}\r\n",
        caja_fila("The Dark Hour has claimed you...")
    )));
    let _ = stdout.queue(SetForegroundColor(ROJO_COMBATE));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_inferior())));
    let _ = stdout.queue(ResetColor);
    let _ = stdout.queue(Print("\n"));
    let _ = stdout.queue(SetForegroundColor(GRIS_SUAVE));
    let _ = stdout.queue(Print(&format!(
        "{}\r\n",
        texto_margen("Press any key to exit...")
    )));
    let _ = stdout.flush();
}

pub fn render_victoria(jugador: &Personaje, pisos: u32) {
    limpiar_pantalla();
    let mut stdout = io::stdout();

    mover_contenido(&mut stdout, 14);
    let sol = [
        "     ______",
        "   .-      -.",
        "  /    ..    \\",
        " |   .    .   |",
        " |  .      .  |",
        "  \\    ..    /",
        "   '-.____.-'",
    ];
    let _ = stdout.queue(SetForegroundColor(DORADO));
    for fila in sol {
        let _ = stdout.queue(Print(&format!("{}\r\n", texto_margen(fila))));
    }
    let _ = stdout.queue(SetForegroundColor(AZUL_MARCO));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_superior())));
    let _ = stdout.queue(SetForegroundColor(DORADO));
    let _ = stdout.queue(Print(&format!(
        "{}\r\n",
        caja_fila("  T A R T A R U S   C O N Q U E R E D")
    )));
    let _ = stdout.queue(SetForegroundColor(AZUL_MARCO));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_medio())));
    let _ = stdout.queue(ResetColor);

    let _ = stdout.queue(SetForegroundColor(VERDE_OK));
    let _ = stdout.queue(Print(&format!(
        "{}\r\n",
        caja_fila(&format!(
            "{} defeated the Floor Boss and sealed the Dark Hour!",
            jugador.nombre
        ))
    )));
    let _ = stdout.queue(Print(&format!(
        "{}\r\n",
        caja_fila(&format!(
            "Final stats: Lv.{} · {} EXP · {} floors cleared",
            jugador.nivel, jugador.experiencia, pisos
        ))
    )));
    let _ = stdout.queue(ResetColor);

    let _ = stdout.queue(SetForegroundColor(AZUL_MARCO));
    let _ = stdout.queue(Print(&format!("{}\r\n", caja_inferior())));
    let _ = stdout.queue(SetForegroundColor(GRIS_SUAVE));
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
