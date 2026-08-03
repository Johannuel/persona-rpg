mod combat;
mod data;
mod ui;

use crate::combat::{
    cambiar_seleccion, iniciar_combate, obtener_seleccion, registrar_experiencia, resolver_turno,
    ResultadoTurno,
};
use crate::data::{
    aplicar_carta, crear_personajes, enemigo_aleatorio, fusionar, generar_cartas, persona_orfeo,
    AccionJugador, CartaShuffle, EstadoCombate, EstadoJuego, Personaje,
};
use crate::ui::{
    leer_tecla, limpiar_pantalla, mostrar_cursor, ocultar_cursor, render_combate,
    render_exploracion, render_fusion, render_game_over, render_habilidades,
    render_seleccion_persona, render_seleccion_personaje, render_shuffle_time, render_titulo,
};
use crossterm::{event::KeyCode, terminal};

fn main() {
    terminal::enable_raw_mode().unwrap();
    ocultar_cursor();
    let mut estado = EstadoJuego::Titulo;
    let personajes = crear_personajes();
    let mut jugador = crear_personajes().remove(0);
    let mut stock = vec![persona_orfeo()];
    let mut seleccion_personaje = 0;
    let mut seleccion_persona = 0;
    let mut estado_combate: Option<EstadoCombate> = None;
    let mut seleccion_habilidad: usize = 0;
    let mut cartas: Vec<CartaShuffle> = Vec::new();
    let mut seleccion_shuffle = 0;
    let mut fusion_fase = 0;
    let mut fusion_a: Option<usize> = None;
    let mut fusion_seleccion = 0;
    let mut mensaje_exploracion = String::from("You're on the street. What will you do?");

    loop {
        match estado {
            EstadoJuego::Titulo => {
                render_titulo();
                match leer_tecla() {
                    Some(KeyCode::Char('q')) | None => break,
                    Some(_) => estado = EstadoJuego::SeleccionPersonaje,
                }
            }
            EstadoJuego::SeleccionPersonaje => {
                render_seleccion_personaje(&personajes, seleccion_personaje);
                match leer_tecla() {
                    Some(KeyCode::Up) => {
                        seleccion_personaje = seleccion_personaje.saturating_sub(1)
                    }
                    Some(KeyCode::Down) => {
                        if seleccion_personaje < personajes.len().saturating_sub(1) {
                            seleccion_personaje += 1;
                        }
                    }
                    Some(KeyCode::Enter) => {
                        jugador = personajes[seleccion_personaje].clone();
                        mensaje_exploracion = format!(
                            "{} awakens their Persona: {}.",
                            jugador.nombre, jugador.persona
                        );
                        estado = EstadoJuego::Explorando;
                    }
                    Some(KeyCode::Char('q')) => break,
                    _ => {}
                }
            }
            EstadoJuego::SeleccionPersona => {
                render_seleccion_persona(&stock, seleccion_persona);
                match leer_tecla() {
                    Some(KeyCode::Up) => seleccion_persona = seleccion_persona.saturating_sub(1),
                    Some(KeyCode::Down) => {
                        if seleccion_persona < stock.len().saturating_sub(1) {
                            seleccion_persona += 1;
                        }
                    }
                    Some(KeyCode::Enter) => {
                        aplicar_persona(&mut jugador, &stock[seleccion_persona]);
                        mensaje_exploracion = format!(
                            "{} summons their Persona: {}.",
                            jugador.nombre, jugador.persona
                        );
                        estado = EstadoJuego::Explorando;
                    }
                    Some(KeyCode::Esc) => estado = EstadoJuego::Explorando,
                    Some(KeyCode::Char('q')) => break,
                    _ => {}
                }
            }
            EstadoJuego::Explorando => {
                let con_personas = jugador.nombre == "Makoto";
                let con_fusion = con_personas && stock.len() >= 2;
                render_exploracion(&jugador, &mensaje_exploracion, con_personas, con_fusion);
                match leer_tecla() {
                    Some(KeyCode::Char('1')) => {
                        let enemigo = enemigo_aleatorio(jugador.nivel);
                        estado_combate = Some(iniciar_combate(&jugador, enemigo));
                        estado = EstadoJuego::Combate;
                    }
                    Some(KeyCode::Char('2')) => {
                        let curacion = (jugador.hp_max as f32 * 0.2) as u32;
                        jugador.hp = (jugador.hp + curacion).min(jugador.hp_max);
                        jugador.mp = (jugador.mp + 5).min(jugador.mp_max);
                        mensaje_exploracion =
                            format!("You rest and recover {} HP and 5 MP.", curacion);
                    }
                    Some(KeyCode::Char('3')) if con_personas => {
                        seleccion_persona = 0;
                        estado = EstadoJuego::SeleccionPersona;
                    }
                    Some(KeyCode::Char('4')) if con_fusion => {
                        fusion_fase = 0;
                        fusion_a = None;
                        fusion_seleccion = 0;
                        estado = EstadoJuego::Fusion;
                    }
                    Some(KeyCode::Char('q')) => break,
                    _ => {}
                }
            }
            EstadoJuego::Combate => {
                if let Some(ref mut combate) = estado_combate {
                    render_combate(combate);
                    match leer_tecla() {
                        Some(KeyCode::Up) => cambiar_seleccion(combate, -1),
                        Some(KeyCode::Down) => cambiar_seleccion(combate, 1),
                        Some(KeyCode::Enter) => {
                            let accion = match obtener_seleccion(combate) {
                                0 => AccionJugador::Atacar,
                                1 => {
                                    seleccion_habilidad = 0;
                                    estado = EstadoJuego::SeleccionHabilidad;
                                    continue;
                                }
                                2 => AccionJugador::Defender,
                                _ => AccionJugador::Huir,
                            };
                            match resolver_turno_del_jugador(
                                combate,
                                &accion,
                                &mut jugador,
                                &mut mensaje_exploracion,
                            ) {
                                ResultadoTurno::Continua => {}
                                ResultadoTurno::Huida => {
                                    mensaje_exploracion = String::from("You fled the battle.");
                                    estado = EstadoJuego::Explorando;
                                    estado_combate = None;
                                }
                                ResultadoTurno::Victoria => {
                                    let nivel = combate.jugador.nivel;
                                    estado_combate = None;
                                    cartas = generar_cartas(nivel);
                                    seleccion_shuffle = 0;
                                    estado = EstadoJuego::ShuffleTime;
                                }
                                ResultadoTurno::Derrota => {
                                    estado = EstadoJuego::GameOver;
                                    estado_combate = None;
                                }
                            }
                        }
                        Some(KeyCode::Char('q')) => {
                            estado = EstadoJuego::GameOver;
                            estado_combate = None;
                        }
                        _ => {}
                    }
                }
            }
            EstadoJuego::ShuffleTime => {
                render_shuffle_time(&cartas, seleccion_shuffle);
                match leer_tecla() {
                    Some(KeyCode::Up) => seleccion_shuffle = seleccion_shuffle.saturating_sub(1),
                    Some(KeyCode::Down) => {
                        if seleccion_shuffle < cartas.len().saturating_sub(1) {
                            seleccion_shuffle += 1;
                        }
                    }
                    Some(KeyCode::Enter) => {
                        let carta = cartas.remove(seleccion_shuffle);
                        mensaje_exploracion = format!(
                            "{} {}",
                            mensaje_exploracion,
                            aplicar_carta(carta, &mut jugador, &mut stock)
                        );
                        estado = EstadoJuego::Explorando;
                    }
                    Some(KeyCode::Char('q')) => break,
                    _ => {}
                }
            }
            EstadoJuego::Fusion => {
                render_fusion(&stock, fusion_fase, fusion_seleccion, fusion_a);
                match leer_tecla() {
                    Some(KeyCode::Up) => fusion_seleccion = fusion_seleccion.saturating_sub(1),
                    Some(KeyCode::Down) => {
                        if fusion_seleccion < stock.len().saturating_sub(1) {
                            fusion_seleccion += 1;
                        }
                    }
                    Some(KeyCode::Enter) => {
                        if fusion_fase == 0 {
                            fusion_a = Some(fusion_seleccion);
                            fusion_fase = 1;
                            fusion_seleccion = 0;
                        } else if let Some(a) = fusion_a {
                            if a != fusion_seleccion {
                                let resultado = fusionar(&stock[a], &stock[fusion_seleccion]);
                                mensaje_exploracion = format!(
                                    "Elizabeth fused {} and {}: {} is born!",
                                    stock[a].persona,
                                    stock[fusion_seleccion].persona,
                                    resultado.persona
                                );
                                let (mayor, menor) =
                                    (a.max(fusion_seleccion), a.min(fusion_seleccion));
                                stock.remove(mayor);
                                stock.remove(menor);
                                stock.push(resultado);
                                fusion_a = None;
                                fusion_fase = 0;
                                fusion_seleccion = 0;
                                estado = EstadoJuego::Explorando;
                            }
                        }
                    }
                    Some(KeyCode::Esc) => {
                        if fusion_fase == 1 {
                            fusion_a = None;
                            fusion_fase = 0;
                            fusion_seleccion = 0;
                        } else {
                            estado = EstadoJuego::Explorando;
                        }
                    }
                    Some(KeyCode::Char('q')) => break,
                    _ => {}
                }
            }
            EstadoJuego::SeleccionHabilidad => {
                if let Some(ref mut combate) = estado_combate {
                    render_habilidades(&combate.jugador, seleccion_habilidad);
                    match leer_tecla() {
                        Some(KeyCode::Up) => {
                            seleccion_habilidad = seleccion_habilidad.saturating_sub(1);
                        }
                        Some(KeyCode::Down) => {
                            if seleccion_habilidad
                                < combate.jugador.habilidades.len().saturating_sub(1)
                            {
                                seleccion_habilidad += 1;
                            }
                        }
                        Some(KeyCode::Enter) => {
                            match resolver_turno_del_jugador(
                                combate,
                                &AccionJugador::Habilidad(seleccion_habilidad),
                                &mut jugador,
                                &mut mensaje_exploracion,
                            ) {
                                ResultadoTurno::Continua => estado = EstadoJuego::Combate,
                                ResultadoTurno::Huida => {}
                                ResultadoTurno::Victoria => {
                                    let nivel = combate.jugador.nivel;
                                    estado_combate = None;
                                    cartas = generar_cartas(nivel);
                                    seleccion_shuffle = 0;
                                    estado = EstadoJuego::ShuffleTime;
                                }
                                ResultadoTurno::Derrota => {
                                    estado = EstadoJuego::GameOver;
                                    estado_combate = None;
                                }
                            }
                        }
                        Some(KeyCode::Esc) => estado = EstadoJuego::Combate,
                        _ => {}
                    }
                }
            }
            EstadoJuego::GameOver => {
                render_game_over();
                let _ = leer_tecla();
                break;
            }
        }
    }

    mostrar_cursor();
    terminal::disable_raw_mode().unwrap();
    limpiar_pantalla();
}

fn aplicar_persona(jugador: &mut Personaje, persona: &Personaje) {
    jugador.persona = persona.persona.clone();
    jugador.hp_max = persona.hp_max;
    jugador.mp_max = persona.mp_max;
    jugador.ataque = persona.ataque;
    jugador.defensa = persona.defensa;
    jugador.habilidades = persona.habilidades.clone();
    jugador.debilidades = persona.debilidades.clone();
    jugador.resistencias = persona.resistencias.clone();
    jugador.hp = jugador.hp_max;
    jugador.mp = jugador.mp_max;
}

fn resolver_turno_del_jugador(
    combate: &mut EstadoCombate,
    accion: &AccionJugador,
    jugador: &mut Personaje,
    mensaje_exploracion: &mut String,
) -> ResultadoTurno {
    let resultado = resolver_turno(combate, accion);
    if resultado == ResultadoTurno::Victoria {
        let subio = registrar_experiencia(combate);
        *mensaje_exploracion = if subio {
            format!("You leveled up to level {}!", combate.jugador.nivel)
        } else {
            String::from("You defeated the enemy!")
        };
        *jugador = combate.jugador.clone();
    }
    resultado
}
