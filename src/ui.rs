use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, ClearType},
    QueueableCommand,
};
use std::io::{self, Write};

use crate::data::{EstadoCombate, Personaje};

const ANCHO: usize = 50;

fn linea() -> String {
    format!("+{}+", "-".repeat(ANCHO))
}

fn linea_doble() -> String {
    format!("+{}+", "=".repeat(ANCHO))
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

pub fn render_titulo() {
    limpiar_pantalla();
    let mut stdout = io::stdout();
    let _ = stdout.queue(SetForegroundColor(Color::Yellow));
    let _ = stdout.queue(Print(&format!("{}\r\n", linea_doble())));
    let _ = stdout.queue(Print("  *  *  *  P E R S O N A   R P G  *  *  *\r\n"));
    let _ = stdout.queue(Print(&format!("{}\r\n", linea_doble())));
    let _ = stdout.queue(ResetColor);
    let _ = stdout.queue(Print("\r\n"));
    let _ = stdout.queue(Print("  Un juego de RPG por turnos\r\n"));
    let _ = stdout.queue(Print("  inspirado en Persona 3 Portable\r\n"));
    let _ = stdout.queue(Print("\r\n"));
    let _ = stdout.queue(SetForegroundColor(Color::Cyan));
    let _ = stdout.queue(Print("  Presiona cualquier tecla para comenzar\r\n"));
    let _ = stdout.queue(Print("  o 'q' para salir\r\n"));
    let _ = stdout.queue(ResetColor);
    let _ = stdout.flush();
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

pub fn render_exploracion(jugador: &Personaje, mensaje: &str) {
    limpiar_pantalla();
    let mut stdout = io::stdout();

    let _ = stdout.queue(SetForegroundColor(Color::Cyan));
    let _ = stdout.queue(Print(&format!("{}\r\n", linea())));
    let _ = stdout.queue(Print(&format!(
        "  {}  |  Nv.{}  |  Exp: {}\r\n",
        jugador.nombre, jugador.nivel, jugador.experiencia
    )));
    let _ = stdout.queue(Print(&format!("{}\r\n", linea())));
    let _ = stdout.queue(ResetColor);

    let hp_pct = jugador.hp as f32 / jugador.hp_max as f32;
    let _ = stdout.queue(SetForegroundColor(color_hp(hp_pct)));
    let _ = stdout.queue(Print(&format!(
        "  HP  {}/{}  {}\r\n",
        jugador.hp,
        jugador.hp_max,
        barra(jugador.hp, jugador.hp_max, 20)
    )));
    let _ = stdout.queue(ResetColor);

    let _ = stdout.queue(SetForegroundColor(Color::DarkGrey));
    let _ = stdout.queue(Print(&format!("{}\r\n", linea())));
    let _ = stdout.queue(ResetColor);

    let mp_pct = jugador.mp as f32 / jugador.mp_max as f32;
    let _ = stdout.queue(SetForegroundColor(color_mp(mp_pct)));
    let _ = stdout.queue(Print(&format!(
        "  MP  {}/{}  {}\r\n",
        jugador.mp,
        jugador.mp_max,
        barra(jugador.mp, jugador.mp_max, 10)
    )));
    let _ = stdout.queue(ResetColor);

    let _ = stdout.queue(SetForegroundColor(Color::White));
    let _ = stdout.queue(Print(&format!("{}\r\n", linea())));
    let _ = stdout.queue(Print(&format!("  {}\r\n", mensaje)));
    let _ = stdout.queue(Print(&format!("{}\r\n", linea())));
    let _ = stdout.queue(ResetColor);

    let _ = stdout.queue(SetForegroundColor(Color::Green));
    let _ = stdout.queue(Print("  [1] Avanzar   [2] Descansar   [q] Salir\r\n"));
    let _ = stdout.queue(ResetColor);
    let _ = stdout.flush();
}

pub fn render_combate(estado: &EstadoCombate) {
    limpiar_pantalla();
    let mut stdout = io::stdout();

    let _ = stdout.queue(SetForegroundColor(Color::Red));
    let _ = stdout.queue(Print(&format!("{}\r\n", linea_doble())));
    let _ = stdout.queue(Print("  =  C O M B A T E  =\r\n"));
    let _ = stdout.queue(Print(&format!("{}\r\n", linea_doble())));
    let _ = stdout.queue(ResetColor);

    let enemigo = &estado.enemigo.personaje;
    let hp_pct_enemigo = enemigo.hp as f32 / enemigo.hp_max as f32;
    let _ = stdout.queue(SetForegroundColor(color_hp(hp_pct_enemigo)));
    let _ = stdout.queue(Print(&format!(
        "  {}  (Nv.{})\r\n",
        enemigo.nombre, enemigo.nivel
    )));
    let _ = stdout.queue(Print(&format!(
        "  HP: {}/{}  {}\r\n",
        enemigo.hp,
        enemigo.hp_max,
        barra(enemigo.hp, enemigo.hp_max, 20)
    )));
    let _ = stdout.queue(ResetColor);

    let _ = stdout.queue(SetForegroundColor(Color::DarkGrey));
    let _ = stdout.queue(Print(&format!("{}\r\n", linea())));
    let _ = stdout.queue(ResetColor);

    let jugador = &estado.jugador;
    let hp_pct_jugador = jugador.hp as f32 / jugador.hp_max as f32;
    let _ = stdout.queue(SetForegroundColor(color_hp(hp_pct_jugador)));
    let _ = stdout.queue(Print(&format!(
        "  {}  (Nv.{})\r\n",
        jugador.nombre, jugador.nivel
    )));
    let _ = stdout.queue(Print(&format!(
        "  HP: {}/{}  {}\r\n",
        jugador.hp,
        jugador.hp_max,
        barra(jugador.hp, jugador.hp_max, 20)
    )));
    let _ = stdout.queue(ResetColor);

    let mp_pct = jugador.mp as f32 / jugador.mp_max as f32;
    let _ = stdout.queue(SetForegroundColor(color_mp(mp_pct)));
    let _ = stdout.queue(Print(&format!(
        "  MP: {}/{}  {}\r\n",
        jugador.mp,
        jugador.mp_max,
        barra(jugador.mp, jugador.mp_max, 10)
    )));
    let _ = stdout.queue(ResetColor);

    let _ = stdout.queue(SetForegroundColor(Color::DarkGrey));
    let _ = stdout.queue(Print(&format!("{}\r\n", linea())));
    let _ = stdout.queue(Print("  Registro:\r\n"));
    let _ = stdout.queue(Print(&format!("{}\r\n", linea())));
    let _ = stdout.queue(ResetColor);

    let inicio = if estado.registro.len() > 5 {
        estado.registro.len() - 5
    } else {
        0
    };
    for linea in &estado.registro[inicio..] {
        let _ = stdout.queue(Print(&format!("  {}\r\n", linea)));
    }

    let _ = stdout.queue(SetForegroundColor(Color::Yellow));
    let _ = stdout.queue(Print(&format!("{}\r\n", linea())));
    let _ = stdout.queue(Print("  Accion:\r\n"));
    let _ = stdout.queue(Print(&format!("{}\r\n", linea())));
    let _ = stdout.queue(ResetColor);

    for (i, opcion) in estado.opciones.iter().enumerate() {
        if i == estado.seleccion_actual {
            let _ = stdout.queue(SetForegroundColor(Color::Yellow));
            let _ = stdout.queue(SetBackgroundColor(Color::DarkGrey));
            let _ = stdout.queue(Print(&format!("  > {} <\r\n", opcion)));
            let _ = stdout.queue(ResetColor);
        } else {
            let _ = stdout.queue(Print(&format!("    {}\r\n", opcion)));
        }
    }

    let _ = stdout.flush();
}

pub fn render_habilidades(jugador: &Personaje, seleccion: usize) {
    limpiar_pantalla();
    let mut stdout = io::stdout();

    let _ = stdout.queue(SetForegroundColor(Color::Magenta));
    let _ = stdout.queue(Print(&format!("{}\r\n", linea_doble())));
    let _ = stdout.queue(Print("  H A B I L I D A D E S\r\n"));
    let _ = stdout.queue(Print(&format!("{}\r\n", linea_doble())));
    let _ = stdout.queue(ResetColor);

    for (i, hab) in jugador.habilidades.iter().enumerate() {
        let estado_str = if hab.multiplicador_dano > 0.0 {
            format!("x{}", hab.multiplicador_dano)
        } else {
            "curacion".to_string()
        };
        if i == seleccion {
            let _ = stdout.queue(SetForegroundColor(Color::Yellow));
            let _ = stdout.queue(SetBackgroundColor(Color::DarkGrey));
            let _ = stdout.queue(Print(&format!(
                "  > {}  - {}  {}\r\n",
                hab.nombre, hab.descripcion, estado_str
            )));
            let _ = stdout.queue(ResetColor);
        } else {
            let _ = stdout.queue(Print(&format!(
                "    {}  - {}  {}\r\n",
                hab.nombre, hab.descripcion, estado_str
            )));
        }
    }

    let _ = stdout.queue(SetForegroundColor(Color::Green));
    let _ = stdout.queue(Print(&format!("{}\r\n", linea())));
    let _ = stdout.queue(Print("  ↑/↓ Seleccionar  Enter Usar  Esc Volver\r\n"));
    let _ = stdout.queue(ResetColor);
    let _ = stdout.flush();
}

pub fn render_game_over() {
    limpiar_pantalla();
    let mut stdout = io::stdout();

    let _ = stdout.queue(SetForegroundColor(Color::Red));
    let _ = stdout.queue(Print(&format!("{}\r\n", linea_doble())));
    let _ = stdout.queue(Print("  G A M E   O V E R\r\n"));
    let _ = stdout.queue(Print(&format!("{}\r\n", linea_doble())));
    let _ = stdout.queue(ResetColor);
    let _ = stdout.queue(Print("\n  Presiona cualquier tecla para salir...\r\n"));
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

fn barra(actual: u32, maximo: u32, ancho: usize) -> String {
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
        resultado.push('#');
    }
    for _ in 0..vacios {
        resultado.push('.');
    }
    resultado.push(']');
    resultado
}
