use rand::Rng;

#[derive(Clone, Debug)]
pub struct Skill {
    pub nombre: String,
    pub coste_mp: u32,
    pub multiplicador_dano: f32,
    pub descripcion: String,
}

#[derive(Clone, Debug)]
pub struct Personaje {
    pub nombre: String,
    pub hp: u32,
    pub hp_max: u32,
    pub mp: u32,
    pub mp_max: u32,
    pub ataque: u32,
    pub defensa: u32,
    pub nivel: u32,
    pub experiencia: u32,
    pub habilidades: Vec<Skill>,
}

#[derive(Clone, Debug)]
pub struct Enemigo {
    pub personaje: Personaje,
}

#[derive(Clone, Debug, PartialEq)]
pub enum EstadoJuego {
    Titulo,
    Explorando,
    Combate,
    SeleccionHabilidad,
    GameOver,
}

#[derive(Clone, Debug)]
pub struct EstadoCombate {
    pub jugador: Personaje,
    pub enemigo: Enemigo,
    pub defendiendo: bool,
    pub defensa_debuffada: bool,
    pub registro: Vec<String>,
    pub opciones: Vec<String>,
    pub seleccion_actual: usize,
}

#[derive(Clone, Debug)]
pub enum AccionJugador {
    Atacar,
    Habilidad(usize),
    Defender,
    Huir,
}

pub fn crear_jugador() -> Personaje {
    Personaje {
        nombre: String::from("Makoto"),
        hp: 100,
        hp_max: 100,
        mp: 50,
        mp_max: 50,
        ataque: 15,
        defensa: 8,
        nivel: 1,
        experiencia: 0,
        habilidades: vec![
            Skill {
                nombre: String::from("Golpe"),
                coste_mp: 0,
                multiplicador_dano: 1.0,
                descripcion: String::from("Un ataque básico"),
            },
            Skill {
                nombre: String::from("Puñetazo"),
                coste_mp: 5,
                multiplicador_dano: 1.8,
                descripcion: String::from("Un golpe potente que consume MP"),
            },
            Skill {
                nombre: String::from("Curar"),
                coste_mp: 10,
                multiplicador_dano: 0.0,
                descripcion: String::from("Restaura 30 HP"),
            },
        ],
    }
}

pub fn crear_enemigos_nivel(nivel: u32) -> Vec<Enemigo> {
    let base_atk = 5 + nivel * 3;
    let base_hp = 30 + nivel * 15;
    let base_def = 3 + nivel * 2;

    vec![
        Enemigo {
            personaje: Personaje {
                nombre: format!("Slime Nv.{}", nivel),
                hp: base_hp,
                hp_max: base_hp,
                mp: 10,
                mp_max: 10,
                ataque: base_atk,
                defensa: base_def,
                nivel,
                experiencia: 0,
                habilidades: vec![Skill {
                    nombre: String::from("Bola de Slime"),
                    coste_mp: 3,
                    multiplicador_dano: 1.2,
                    descripcion: String::from("Una pegajosa bola de slime"),
                }],
            },
        },
        Enemigo {
            personaje: Personaje {
                nombre: format!("Sombras Nv.{}", nivel),
                hp: base_hp + 20,
                hp_max: base_hp + 20,
                mp: 20,
                mp_max: 20,
                ataque: base_atk + 2,
                defensa: base_def + 1,
                nivel,
                experiencia: 0,
                habilidades: vec![
                    Skill {
                        nombre: String::from("Garras Sombrías"),
                        coste_mp: 5,
                        multiplicador_dano: 2.0,
                        descripcion: String::from("Garras oscuras que cortan profundo"),
                    },
                    Skill {
                        nombre: String::from("Aullido"),
                        coste_mp: 8,
                        multiplicador_dano: 0.0,
                        descripcion: String::from("Reduce la defensa del rival"),
                    },
                ],
            },
        },
    ]
}

pub fn enemigo_aleatorio(nivel: u32) -> Enemigo {
    let enemigos = crear_enemigos_nivel(nivel);
    let mut rng = rand::thread_rng();
    enemigos[rng.gen_range(0..enemigos.len())].clone()
}

pub fn exp_para_nivel(nivel: u32) -> u32 {
    nivel * 25
}

pub fn subir_nivel(jugador: &mut Personaje) -> bool {
    let exp_necesaria = exp_para_nivel(jugador.nivel);
    if jugador.experiencia >= exp_necesaria && jugador.nivel < 20 {
        jugador.nivel += 1;
        jugador.hp_max += 10;
        jugador.mp_max += 5;
        jugador.ataque += 2;
        jugador.defensa += 1;
        jugador.hp = jugador.hp_max;
        jugador.mp = jugador.mp_max;
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exp_para_nivel_debe_crecer_con_el_nivel() {
        assert_eq!(exp_para_nivel(1), 25);
        assert!(exp_para_nivel(5) > exp_para_nivel(1));
    }

    #[test]
    fn subir_nivel_debe_mejorar_las_stats() {
        let mut jugador = crear_jugador();
        jugador.experiencia = 25;
        assert!(subir_nivel(&mut jugador));
        assert_eq!(jugador.nivel, 2);
        assert_eq!(jugador.hp_max, 110);
        assert_eq!(jugador.ataque, 17);
    }

    #[test]
    fn subir_nivel_no_debe_superar_el_nivel_maximo() {
        let mut jugador = crear_jugador();
        jugador.nivel = 20;
        jugador.experiencia = 10_000;
        assert!(!subir_nivel(&mut jugador));
        assert_eq!(jugador.nivel, 20);
    }

    #[test]
    fn enemigos_de_nivel_alto_deben_ser_mas_fuertes() {
        let nivel_1 = crear_enemigos_nivel(1);
        let nivel_5 = crear_enemigos_nivel(5);
        assert!(nivel_5[0].personaje.hp_max > nivel_1[0].personaje.hp_max);
        assert!(nivel_5[0].personaje.ataque > nivel_1[0].personaje.ataque);
    }
}
