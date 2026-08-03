use crate::data::{
    subir_nivel, AccionJugador, Efecto, Elemento, Enemigo, EstadoCombate, Personaje, Skill,
};
use rand::Rng;

pub fn iniciar_combate(jugador: &Personaje, enemigo: Enemigo) -> EstadoCombate {
    EstadoCombate {
        jugador: jugador.clone(),
        enemigo,
        defendiendo: false,
        debuff_defensa_jugador: 0,
        buff_ataque: 0,
        buff_defensa: 0,
        debuff_ataque: 0,
        debuff_defensa: 0,
        registro: vec![String::from("The battle begins!")],
        opciones: vec![
            String::from("Attack"),
            String::from("Skills"),
            String::from("Defend"),
            String::from("Flee"),
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

#[derive(Clone, Copy, Debug, PartialEq)]
enum ResultadoGolpe {
    Normal,
    Debil,
    Resistido,
}

pub fn ejecutar_accion_jugador(
    estado: &mut EstadoCombate,
    accion: &AccionJugador,
) -> ResultadoAccion {
    match accion {
        AccionJugador::Atacar => {
            let nombre_jugador = estado.jugador.nombre.clone();
            let nombre_enemigo = estado.enemigo.personaje.nombre.clone();
            let (dano, golpe) = calcular_dano_con_elemento(
                ataque_jugador(estado),
                defensa_enemigo(estado),
                1.0,
                Elemento::Fisico,
                &estado.enemigo.personaje.debilidades,
                &estado.enemigo.personaje.resistencias,
            );
            estado.enemigo.personaje.hp = estado.enemigo.personaje.hp.saturating_sub(dano);
            estado
                .registro
                .push(format!("{} strikes for {} damage!", nombre_jugador, dano));
            registrar_resultado_golpe(estado, &golpe, &nombre_enemigo);
        }
        AccionJugador::Habilidad(idx) => {
            let Some(habilidad) = estado.jugador.habilidades.get(*idx).cloned() else {
                estado.registro.push(String::from("Invalid skill"));
                return ResultadoAccion::NoValida;
            };
            if estado.jugador.mp < habilidad.coste_mp {
                estado.registro.push(String::from("Not enough MP"));
                return ResultadoAccion::NoValida;
            }
            estado.jugador.mp -= habilidad.coste_mp;
            let nombre_jugador = estado.jugador.nombre.clone();
            aplicar_habilidad(&nombre_jugador, &habilidad, estado);
        }
        AccionJugador::Defender => {
            estado
                .registro
                .push(format!("{} defends", estado.jugador.nombre));
            estado.defendiendo = true;
        }
        AccionJugador::Huir => {
            if rand::thread_rng().gen_bool(0.5) {
                estado.registro.push(String::from("You fled successfully!"));
                return ResultadoAccion::Huida;
            }
            estado.registro.push(String::from("You couldn't escape!"));
        }
    }
    ResultadoAccion::Realizada
}

fn aplicar_habilidad(usuario: &str, habilidad: &Skill, estado: &mut EstadoCombate) {
    match habilidad.efecto {
        Efecto::Danio => {
            let nombre_enemigo = estado.enemigo.personaje.nombre.clone();
            let (dano, golpe) = calcular_dano_con_elemento(
                ataque_jugador(estado),
                defensa_enemigo(estado),
                habilidad.multiplicador_dano,
                habilidad.elemento,
                &estado.enemigo.personaje.debilidades,
                &estado.enemigo.personaje.resistencias,
            );
            estado.enemigo.personaje.hp = estado.enemigo.personaje.hp.saturating_sub(dano);
            estado.registro.push(format!(
                "{}{} uses {} and deals {} damage!",
                etiqueta_dano(habilidad.elemento),
                usuario,
                habilidad.nombre,
                dano
            ));
            registrar_resultado_golpe(estado, &golpe, &nombre_enemigo);
        }
        Efecto::Curacion => {
            let curacion = (estado.jugador.hp_max as f32 * habilidad.multiplicador_dano) as u32;
            estado.jugador.hp = (estado.jugador.hp + curacion).min(estado.jugador.hp_max);
            estado.registro.push(format!(
                "{} uses {} and recovers {} HP!",
                usuario, habilidad.nombre, curacion
            ));
        }
        Efecto::BuffAtaque => {
            estado.buff_ataque = 3;
            estado.registro.push(format!(
                "{} uses {} and raises its attack!",
                usuario, habilidad.nombre
            ));
        }
        Efecto::BuffDefensa => {
            estado.buff_defensa = 3;
            estado.registro.push(format!(
                "{} uses {} and raises its defense!",
                usuario, habilidad.nombre
            ));
        }
        Efecto::DebuffAtaque => {
            estado.debuff_ataque = 3;
            estado.registro.push(format!(
                "{} lowers the enemy's attack with {}!",
                usuario, habilidad.nombre
            ));
        }
        Efecto::DebuffDefensa => {
            estado.debuff_defensa = 3;
            estado.registro.push(format!(
                "{} lowers the enemy's defense with {}!",
                usuario, habilidad.nombre
            ));
        }
    }
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
    if jugador_derrotado(estado) {
        return ResultadoTurno::Derrota;
    }
    finalizar_round(estado);
    ResultadoTurno::Continua
}

pub fn turno_enemigo(estado: &mut EstadoCombate) {
    let mut rng = rand::thread_rng();
    let habilidades = estado.enemigo.personaje.habilidades.clone();
    let nombre_enemigo = estado.enemigo.personaje.nombre.clone();

    let idx = rng.gen_range(0..habilidades.len());
    let habilidad = &habilidades[idx];

    if estado.enemigo.personaje.mp >= habilidad.coste_mp {
        estado.enemigo.personaje.mp -= habilidad.coste_mp;
        match habilidad.efecto {
            Efecto::Danio => {
                let nombre_jugador = estado.jugador.nombre.clone();
                let (dano, golpe) = calcular_dano_con_elemento(
                    ataque_enemigo(estado),
                    defensa_jugador(estado),
                    habilidad.multiplicador_dano,
                    habilidad.elemento,
                    &estado.jugador.debilidades,
                    &estado.jugador.resistencias,
                );
                estado.jugador.hp = estado.jugador.hp.saturating_sub(dano);
                estado.registro.push(format!(
                    "{}{} uses {} and deals {} damage!",
                    etiqueta_dano(habilidad.elemento),
                    nombre_enemigo,
                    habilidad.nombre,
                    dano
                ));
                registrar_resultado_golpe(estado, &golpe, &nombre_jugador);
            }
            Efecto::Curacion => {
                let curacion =
                    (estado.enemigo.personaje.hp_max as f32 * habilidad.multiplicador_dano) as u32;
                estado.enemigo.personaje.hp =
                    (estado.enemigo.personaje.hp + curacion).min(estado.enemigo.personaje.hp_max);
                estado.registro.push(format!(
                    "{} uses {} and recovers {} HP!",
                    nombre_enemigo, habilidad.nombre, curacion
                ));
            }
            Efecto::DebuffDefensa => {
                estado.debuff_defensa_jugador = 3;
                estado.registro.push(format!(
                    "{} lowers {}'s defense!",
                    nombre_enemigo, estado.jugador.nombre
                ));
            }
            _ => {}
        }
    } else {
        let dano = calcular_dano(ataque_enemigo(estado), defensa_jugador(estado), 1.0);
        estado.jugador.hp = estado.jugador.hp.saturating_sub(dano);
        estado.registro.push(format!(
            "{} attacks and deals {} damage!",
            nombre_enemigo, dano
        ));
    }
}

pub fn finalizar_round(estado: &mut EstadoCombate) {
    estado.defendiendo = false;
    estado.buff_ataque = estado.buff_ataque.saturating_sub(1);
    estado.buff_defensa = estado.buff_defensa.saturating_sub(1);
    estado.debuff_ataque = estado.debuff_ataque.saturating_sub(1);
    estado.debuff_defensa = estado.debuff_defensa.saturating_sub(1);
    estado.debuff_defensa_jugador = estado.debuff_defensa_jugador.saturating_sub(1);
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

fn ataque_jugador(estado: &EstadoCombate) -> u32 {
    estado.jugador.ataque + if estado.buff_ataque > 0 { 2 } else { 0 }
}

fn defensa_jugador(estado: &EstadoCombate) -> u32 {
    let mut defensa = estado.jugador.defensa;
    if estado.defendiendo {
        defensa += 3;
    }
    if estado.buff_defensa > 0 {
        defensa += 3;
    }
    defensa.saturating_sub(if estado.debuff_defensa_jugador > 0 {
        3
    } else {
        0
    })
}

fn ataque_enemigo(estado: &EstadoCombate) -> u32 {
    estado
        .enemigo
        .personaje
        .ataque
        .saturating_sub(if estado.debuff_ataque > 0 { 2 } else { 0 })
}

fn defensa_enemigo(estado: &EstadoCombate) -> u32 {
    estado
        .enemigo
        .personaje
        .defensa
        .saturating_sub(if estado.debuff_defensa > 0 { 3 } else { 0 })
}

fn calcular_dano(ataque: u32, defensa: u32, multiplicador: f32) -> u32 {
    let base = ataque as f32 * multiplicador;
    let reduccion = defensa as f32 * 0.3;
    (base - reduccion).max(1.0) as u32
}

fn calcular_dano_con_elemento(
    ataque: u32,
    defensa: u32,
    multiplicador: f32,
    elemento: Elemento,
    debilidades: &[Elemento],
    resistencias: &[Elemento],
) -> (u32, ResultadoGolpe) {
    let factor = aplicar_elemento(multiplicador, elemento, debilidades, resistencias);
    (calcular_dano(ataque, defensa, factor.0), factor.1)
}

fn aplicar_elemento(
    multiplicador: f32,
    elemento: Elemento,
    debilidades: &[Elemento],
    resistencias: &[Elemento],
) -> (f32, ResultadoGolpe) {
    if elemento == Elemento::Fisico {
        return (multiplicador, ResultadoGolpe::Normal);
    }
    if debilidades.contains(&elemento) {
        (multiplicador * 1.5, ResultadoGolpe::Debil)
    } else if resistencias.contains(&elemento) {
        (multiplicador * 0.5, ResultadoGolpe::Resistido)
    } else {
        (multiplicador, ResultadoGolpe::Normal)
    }
}

fn registrar_resultado_golpe(estado: &mut EstadoCombate, golpe: &ResultadoGolpe, objetivo: &str) {
    match golpe {
        ResultadoGolpe::Debil => estado
            .registro
            .push(format!("{} is weak! Critical hit!", objetivo)),
        ResultadoGolpe::Resistido => {
            estado
                .registro
                .push(format!("{} resists this attack!", objetivo));
        }
        ResultadoGolpe::Normal => {}
    }
}

fn etiqueta_dano(elemento: Elemento) -> String {
    if elemento == Elemento::Fisico {
        String::new()
    } else {
        format!("[{}] ", elemento.etiqueta())
    }
}

pub fn registrar_experiencia(estado: &mut EstadoCombate) -> bool {
    let exp_ganada = 10 + estado.enemigo.personaje.nivel * 5;
    estado.jugador.experiencia += exp_ganada;
    estado.registro.push(format!("+{} experience", exp_ganada));
    subir_nivel(&mut estado.jugador)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::{crear_jugador, Skill};

    fn enemigo_prueba(hp: u32) -> Enemigo {
        Enemigo {
            personaje: Personaje {
                nombre: String::from("Test dummy"),
                persona: String::from("Test shadow"),
                arcana: None,
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
                    descripcion: String::from("Basic strike"),
                    elemento: Elemento::Fisico,
                    efecto: Efecto::Danio,
                }],
                debilidades: vec![],
                resistencias: vec![],
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
        assert_eq!(combate.jugador.mp, 45);
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
        assert!(!combate.defendiendo);
    }

    #[test]
    fn debuff_del_enemigo_debe_reducir_la_defensa_por_unos_turnos() {
        let mut combate = combate_prueba();
        combate.enemigo.personaje.habilidades = vec![Skill {
            nombre: String::from("Aullido"),
            coste_mp: 0,
            multiplicador_dano: 0.0,
            descripcion: String::from("Reduce la defensa"),
            elemento: Elemento::Fisico,
            efecto: Efecto::DebuffDefensa,
        }];
        turno_enemigo(&mut combate);
        assert_eq!(combate.debuff_defensa_jugador, 3);
        assert_eq!(defensa_jugador(&combate), 5);
        finalizar_round(&mut combate);
        assert_eq!(combate.debuff_defensa_jugador, 2);
    }

    #[test]
    fn golpear_una_debilidad_debe_hacer_dano_critico() {
        let mut combate = combate_prueba();
        combate.enemigo.personaje.debilidades = vec![Elemento::Fuego];
        combate.jugador.habilidades[1].elemento = Elemento::Fuego;
        let (dano, golpe) = calcular_dano_con_elemento(
            ataque_jugador(&combate),
            defensa_enemigo(&combate),
            combate.jugador.habilidades[1].multiplicador_dano,
            combate.jugador.habilidades[1].elemento,
            &combate.enemigo.personaje.debilidades,
            &combate.enemigo.personaje.resistencias,
        );
        assert_eq!(golpe, ResultadoGolpe::Debil);
        assert!(dano > 20);
    }

    #[test]
    fn calcular_dano_minimo_debe_ser_uno() {
        assert_eq!(calcular_dano(5, 100, 1.0), 1);
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
