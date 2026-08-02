use crate::data::{subir_nivel, AccionJugador, Enemigo, EstadoCombate, Personaje};
use rand::Rng;

pub fn iniciar_combate(jugador: &Personaje, enemigo: Enemigo) -> EstadoCombate {
    EstadoCombate {
        jugador: jugador.clone(),
        enemigo,
        defendiendo: false,
        defensa_debuffada: false,
        registro: vec![String::from("¡Comienza el combate!")],
        opciones: vec![
            String::from("Atacar"),
            String::from("Habilidades"),
            String::from("Defender"),
            String::from("Huir"),
        ],
        seleccion_actual: 0,
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ResultadoAccion {
    Realizada,
    NoValida,
    Huida,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ResultadoTurno {
    Continua,
    Victoria,
    Derrota,
    Huida,
}

pub fn ejecutar_accion_jugador(
    estado: &mut EstadoCombate,
    accion: &AccionJugador,
) -> ResultadoAccion {
    match accion {
        AccionJugador::Atacar => {
            let dano = calcular_dano(&estado.jugador, &estado.enemigo.personaje, 1.0);
            estado.enemigo.personaje.hp = estado.enemigo.personaje.hp.saturating_sub(dano);
            estado.registro.push(format!(
                "{} golpea por {} de daño!",
                estado.jugador.nombre, dano
            ));
        }
        AccionJugador::Habilidad(idx) => {
            let Some(habilidad) = estado.jugador.habilidades.get(*idx) else {
                estado.registro.push(String::from("Habilidad inválida"));
                return ResultadoAccion::NoValida;
            };
            if estado.jugador.mp < habilidad.coste_mp {
                estado.registro.push(String::from("MP insuficiente"));
                return ResultadoAccion::NoValida;
            }
            estado.jugador.mp -= habilidad.coste_mp;
            if habilidad.multiplicador_dano > 0.0 {
                let dano = calcular_dano(
                    &estado.jugador,
                    &estado.enemigo.personaje,
                    habilidad.multiplicador_dano,
                );
                estado.enemigo.personaje.hp = estado.enemigo.personaje.hp.saturating_sub(dano);
                estado.registro.push(format!(
                    "{} usa {} y causa {} de daño!",
                    estado.jugador.nombre, habilidad.nombre, dano
                ));
            } else {
                let curacion = (estado.jugador.hp_max as f32 * 0.3) as u32;
                estado.jugador.hp = (estado.jugador.hp + curacion).min(estado.jugador.hp_max);
                estado.registro.push(format!(
                    "{} usa {} y recupera {} HP!",
                    estado.jugador.nombre, habilidad.nombre, curacion
                ));
            }
        }
        AccionJugador::Defender => {
            estado
                .registro
                .push(format!("{} se defiende", estado.jugador.nombre));
            estado.jugador.defensa += 3;
            estado.defendiendo = true;
        }
        AccionJugador::Huir => {
            if rand::thread_rng().gen_bool(0.5) {
                estado.registro.push(String::from("¡Huiste con éxito!"));
                return ResultadoAccion::Huida;
            }
            estado.registro.push(String::from("¡No pudiste escapar!"));
        }
    }
    ResultadoAccion::Realizada
}

pub fn resolver_turno(estado: &mut EstadoCombate, accion: &AccionJugador) -> ResultadoTurno {
    match ejecutar_accion_jugador(estado, accion) {
        ResultadoAccion::NoValida => return ResultadoTurno::Continua,
        ResultadoAccion::Huida => return ResultadoTurno::Huida,
        ResultadoAccion::Realizada => {}
    }
    if enemigo_derrotado(estado) {
        return ResultadoTurno::Victoria;
    }
    turno_enemigo(estado);
    limpiar_defensa(estado);
    if jugador_derrotado(estado) {
        ResultadoTurno::Derrota
    } else {
        ResultadoTurno::Continua
    }
}

pub fn turno_enemigo(estado: &mut EstadoCombate) {
    let enemigo = &mut estado.enemigo.personaje;
    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0..enemigo.habilidades.len());
    let habilidad = &enemigo.habilidades[idx];

    if enemigo.mp >= habilidad.coste_mp {
        enemigo.mp -= habilidad.coste_mp;
        if habilidad.multiplicador_dano > 0.0 {
            let dano = calcular_dano(enemigo, &estado.jugador, habilidad.multiplicador_dano);
            estado.jugador.hp = estado.jugador.hp.saturating_sub(dano);
            estado.registro.push(format!(
                "{} usa {} y causa {} de daño!",
                enemigo.nombre, habilidad.nombre, dano
            ));
        } else {
            if !estado.defensa_debuffada {
                estado.jugador.defensa = estado.jugador.defensa.saturating_sub(2);
                estado.defensa_debuffada = true;
            }
            estado.registro.push(format!(
                "{} usa {} y reduce la defensa de {}!",
                enemigo.nombre, habilidad.nombre, estado.jugador.nombre
            ));
        }
    } else {
        let dano = calcular_dano(enemigo, &estado.jugador, 1.0);
        estado.jugador.hp = estado.jugador.hp.saturating_sub(dano);
        estado.registro.push(format!(
            "{} ataca y causa {} de daño!",
            enemigo.nombre, dano
        ));
    }
}

pub fn limpiar_defensa(estado: &mut EstadoCombate) {
    if estado.defendiendo {
        estado.jugador.defensa = estado.jugador.defensa.saturating_sub(3);
        estado.defendiendo = false;
    }
    if estado.defensa_debuffada {
        estado.jugador.defensa += 2;
        estado.defensa_debuffada = false;
    }
}

pub fn cambiar_seleccion(estado: &mut EstadoCombate, delta: isize) {
    let max = estado.opciones.len().saturating_sub(1);
    if delta > 0 {
        estado.seleccion_actual = (estado.seleccion_actual + 1).min(max);
    } else {
        estado.seleccion_actual = estado.seleccion_actual.saturating_sub(1);
    }
}

pub fn obtener_seleccion(estado: &EstadoCombate) -> usize {
    estado.seleccion_actual
}

fn enemigo_derrotado(estado: &EstadoCombate) -> bool {
    estado.enemigo.personaje.hp == 0
}

fn jugador_derrotado(estado: &EstadoCombate) -> bool {
    estado.jugador.hp == 0
}

fn calcular_dano(atacante: &Personaje, defensor: &Personaje, multiplicador: f32) -> u32 {
    let base = atacante.ataque as f32 * multiplicador;
    let reduccion = defensor.defensa as f32 * 0.3;
    (base - reduccion).max(1.0) as u32
}

pub fn registrar_experiencia(estado: &mut EstadoCombate) -> bool {
    let exp_ganada = 10 + estado.enemigo.personaje.nivel * 5;
    estado.jugador.experiencia += exp_ganada;
    estado.registro.push(format!("+{} experiencia", exp_ganada));
    subir_nivel(&mut estado.jugador)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::{crear_jugador, Skill};

    fn enemigo_prueba(hp: u32) -> Enemigo {
        Enemigo {
            personaje: Personaje {
                nombre: String::from("Muñeco de prueba"),
                hp,
                hp_max: 100,
                mp: 100,
                mp_max: 100,
                ataque: 10,
                defensa: 5,
                nivel: 1,
                experiencia: 0,
                habilidades: vec![Skill {
                    nombre: String::from("Golpe de prueba"),
                    coste_mp: 0,
                    multiplicador_dano: 1.0,
                    descripcion: String::from("Golpe básico"),
                }],
            },
        }
    }

    fn combate_prueba() -> EstadoCombate {
        iniciar_combate(&crear_jugador(), enemigo_prueba(50))
    }

    #[test]
    fn atacar_debe_reducir_hp_del_enemigo() {
        let mut combate = combate_prueba();
        assert_eq!(
            resolver_turno(&mut combate, &AccionJugador::Atacar),
            ResultadoTurno::Continua
        );
        assert_eq!(combate.enemigo.personaje.hp, 37);
    }

    #[test]
    fn golpe_mortal_debe_terminar_sin_turno_del_enemigo() {
        let mut combate = combate_prueba();
        combate.enemigo.personaje.hp = 1;
        assert_eq!(
            resolver_turno(&mut combate, &AccionJugador::Atacar),
            ResultadoTurno::Victoria
        );
        assert_eq!(combate.jugador.hp, 100);
    }

    #[test]
    fn habilidad_invalida_no_debe_causar_dano_ni_consumir_turno() {
        let mut combate = combate_prueba();
        assert_eq!(
            resolver_turno(&mut combate, &AccionJugador::Habilidad(99)),
            ResultadoTurno::Continua
        );
        assert_eq!(combate.enemigo.personaje.hp, 50);
        assert_eq!(combate.jugador.hp, 100);
    }

    #[test]
    fn habilidad_sin_mp_no_debe_ejecutarse_ni_consumir_turno() {
        let mut combate = combate_prueba();
        combate.jugador.mp = 0;
        assert_eq!(
            resolver_turno(&mut combate, &AccionJugador::Habilidad(1)),
            ResultadoTurno::Continua
        );
        assert_eq!(combate.enemigo.personaje.hp, 50);
        assert_eq!(combate.jugador.hp, 100);
    }

    #[test]
    fn habilidad_de_curacion_debe_recuperar_hp() {
        let mut combate = combate_prueba();
        combate.jugador.hp = 50;
        assert_eq!(
            resolver_turno(&mut combate, &AccionJugador::Habilidad(2)),
            ResultadoTurno::Continua
        );
        assert_eq!(combate.jugador.mp, 40);
        assert_eq!(combate.jugador.hp, 73);
    }

    #[test]
    fn defender_debe_mitigar_el_dano_y_restaurar_la_defensa() {
        let mut combate = combate_prueba();
        assert_eq!(
            resolver_turno(&mut combate, &AccionJugador::Defender),
            ResultadoTurno::Continua
        );
        assert_eq!(combate.jugador.hp, 94);
        assert_eq!(combate.jugador.defensa, 8);
    }

    #[test]
    fn habilidad_debuff_del_enemigo_debe_reducir_y_restaurar_la_defensa() {
        let mut combate = combate_prueba();
        combate.enemigo.personaje.habilidades = vec![Skill {
            nombre: String::from("Aullido"),
            coste_mp: 0,
            multiplicador_dano: 0.0,
            descripcion: String::from("Reduce la defensa"),
        }];
        turno_enemigo(&mut combate);
        assert_eq!(combate.jugador.defensa, 6);
        assert!(combate.defensa_debuffada);
        limpiar_defensa(&mut combate);
        assert_eq!(combate.jugador.defensa, 8);
        assert!(!combate.defensa_debuffada);
    }

    #[test]
    fn calcular_dano_minimo_debe_ser_uno() {
        let atacante = Personaje {
            ataque: 5,
            ..crear_jugador()
        };
        let defensor = Personaje {
            defensa: 100,
            ..crear_jugador()
        };
        assert_eq!(calcular_dano(&atacante, &defensor, 1.0), 1);
    }

    #[test]
    fn registrar_experiencia_debe_subir_de_nivel_al_alcanzar_el_umbral() {
        let mut combate = combate_prueba();
        combate.jugador.experiencia = 24;
        assert!(registrar_experiencia(&mut combate));
        assert_eq!(combate.jugador.nivel, 2);
    }

    #[test]
    fn registrar_experiencia_no_debe_subir_de_nivel_con_poca_experiencia() {
        let mut combate = combate_prueba();
        assert!(!registrar_experiencia(&mut combate));
        assert_eq!(combate.jugador.nivel, 1);
    }
}
