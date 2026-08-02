mod combat;
mod data;
mod ui;

use crate::combat::{
    cambiar_seleccion, iniciar_combate, obtener_seleccion, registrar_experiencia, resolver_turno,
    ResultadoTurno,
};
use crate::data::{
    crear_jugador, enemigo_aleatorio, AccionJugador, EstadoCombate, EstadoJuego, Personaje,
};
use crate::ui::{
    leer_tecla, limpiar_pantalla, mostrar_cursor, ocultar_cursor, render_combate,
    render_exploracion, render_game_over, render_habilidades, render_titulo,
};
use crossterm::{event::KeyCode, terminal};

fn main() {
    terminal::enable_raw_mode().unwrap();
    ocultar_cursor();
    let mut estado = EstadoJuego::Titulo;
    let mut jugador = crear_jugador();
    let mut estado_combate: Option<EstadoCombate> = None;
    let mut seleccion_habilidad: usize = 0;
    let mut mensaje_exploracion = String::from("Estás en la calle. ¿Qué harás?");

    loop {
        match estado {
            EstadoJuego::Titulo => {
                render_titulo();
                match leer_tecla() {
                    Some(KeyCode::Char('q')) | None => break,
                    Some(_) => estado = EstadoJuego::Explorando,
                }
            }
            EstadoJuego::Explorando => {
                render_exploracion(&jugador, &mensaje_exploracion);
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
                            format!("Descansas y recuperas {} HP, 5 MP.", curacion);
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
                                    mensaje_exploracion = String::from("Escapaste del combate.");
                                    estado = EstadoJuego::Explorando;
                                    estado_combate = None;
                                }
                                ResultadoTurno::Victoria => {
                                    estado = EstadoJuego::Explorando;
                                    estado_combate = None;
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
                                    estado = EstadoJuego::Explorando;
                                    estado_combate = None;
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
            format!("¡Subiste a nivel {}!", combate.jugador.nivel)
        } else {
            String::from("¡Derrotaste al enemigo!")
        };
        *jugador = combate.jugador.clone();
    }
    resultado
}
