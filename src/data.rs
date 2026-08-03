use rand::Rng;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Elemento {
    Fisico,
    Fuego,
    Hielo,
    Viento,
    Electrico,
}

impl Elemento {
    pub fn etiqueta(&self) -> &'static str {
        match self {
            Elemento::Fisico => "Físico",
            Elemento::Fuego => "Fuego",
            Elemento::Hielo => "Hielo",
            Elemento::Viento => "Viento",
            Elemento::Electrico => "Eléctrico",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arcana {
    Fool,
    Magician,
    Priestess,
    Empress,
    Emperor,
    Hierophant,
    Lovers,
    Chariot,
    Justice,
    Hermit,
    Fortune,
    Strength,
    Hanged,
    Death,
    Temperance,
    Devil,
    Tower,
    Star,
    Moon,
    Sun,
}

impl Arcana {
    pub fn etiqueta(&self) -> &'static str {
        match self {
            Arcana::Fool => "Fool",
            Arcana::Magician => "Magician",
            Arcana::Priestess => "Priestess",
            Arcana::Empress => "Empress",
            Arcana::Emperor => "Emperor",
            Arcana::Hierophant => "Hierophant",
            Arcana::Lovers => "Lovers",
            Arcana::Chariot => "Chariot",
            Arcana::Justice => "Justice",
            Arcana::Hermit => "Hermit",
            Arcana::Fortune => "Fortune",
            Arcana::Strength => "Strength",
            Arcana::Hanged => "Hanged Man",
            Arcana::Death => "Death",
            Arcana::Temperance => "Temperance",
            Arcana::Devil => "Devil",
            Arcana::Tower => "Tower",
            Arcana::Star => "Star",
            Arcana::Moon => "Moon",
            Arcana::Sun => "Sun",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Efecto {
    Danio,
    Curacion,
    BuffAtaque,
    BuffDefensa,
    DebuffAtaque,
    DebuffDefensa,
}

#[derive(Clone, Debug)]
pub struct Skill {
    pub nombre: String,
    pub coste_mp: u32,
    pub multiplicador_dano: f32,
    pub descripcion: String,
    pub elemento: Elemento,
    pub efecto: Efecto,
}

fn skill(
    nombre: &str,
    coste_mp: u32,
    multiplicador_dano: f32,
    descripcion: &str,
    elemento: Elemento,
    efecto: Efecto,
) -> Skill {
    Skill {
        nombre: nombre.to_string(),
        coste_mp,
        multiplicador_dano,
        descripcion: descripcion.to_string(),
        elemento,
        efecto,
    }
}

#[derive(Clone, Debug)]
pub struct Personaje {
    pub nombre: String,
    pub persona: String,
    pub arcana: Option<Arcana>,
    pub hp: u32,
    pub hp_max: u32,
    pub mp: u32,
    pub mp_max: u32,
    pub ataque: u32,
    pub defensa: u32,
    pub nivel: u32,
    pub experiencia: u32,
    pub habilidades: Vec<Skill>,
    pub debilidades: Vec<Elemento>,
    pub resistencias: Vec<Elemento>,
}

#[derive(Clone, Debug)]
pub struct Enemigo {
    pub personaje: Personaje,
}

#[derive(Clone, Debug, PartialEq)]
pub enum EstadoJuego {
    Titulo,
    SeleccionPersonaje,
    SeleccionPersona,
    Explorando,
    ShuffleTime,
    Fusion,
    Combate,
    SeleccionHabilidad,
    GameOver,
}

#[derive(Clone, Debug)]
pub struct EstadoCombate {
    pub jugador: Personaje,
    pub enemigo: Enemigo,
    pub defendiendo: bool,
    pub debuff_defensa_jugador: u32,
    pub buff_ataque: u32,
    pub buff_defensa: u32,
    pub debuff_ataque: u32,
    pub debuff_defensa: u32,
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

#[derive(Clone, Debug)]
pub enum CartaShuffle {
    Persona(Personaje),
    Copa,
    Vara,
    Vacia,
}

#[expect(
    clippy::too_many_arguments,
    reason = "constructor de datos del roster con una fila por personaje"
)]
fn personaje(
    nombre: &str,
    persona: &str,
    arcana: Option<Arcana>,
    hp: u32,
    mp: u32,
    ataque: u32,
    defensa: u32,
    habilidades: Vec<Skill>,
    debilidades: Vec<Elemento>,
    resistencias: Vec<Elemento>,
) -> Personaje {
    Personaje {
        nombre: nombre.to_string(),
        persona: persona.to_string(),
        arcana,
        hp,
        hp_max: hp,
        mp,
        mp_max: mp,
        ataque,
        defensa,
        nivel: 1,
        experiencia: 0,
        habilidades,
        debilidades,
        resistencias,
    }
}

#[expect(
    clippy::too_many_arguments,
    reason = "constructor de datos de personas con una fila por persona"
)]
fn roster_persona(
    nombre_persona: &str,
    arcana: Arcana,
    nivel: u32,
    hp: u32,
    mp: u32,
    ataque: u32,
    defensa: u32,
    habilidades: Vec<Skill>,
    debilidades: Vec<Elemento>,
    resistencias: Vec<Elemento>,
) -> Personaje {
    let mut p = personaje(
        "Makoto",
        nombre_persona,
        Some(arcana),
        hp,
        mp,
        ataque,
        defensa,
        habilidades,
        debilidades,
        resistencias,
    );
    p.nivel = nivel;
    p
}

pub fn crear_personajes() -> Vec<Personaje> {
    vec![
        crear_makoto(),
        crear_yukari(),
        crear_junpei(),
        crear_akihiko(),
        crear_mitsuru(),
    ]
}

#[cfg(test)]
pub fn crear_jugador() -> Personaje {
    crear_makoto()
}

fn crear_makoto() -> Personaje {
    personaje(
        "Makoto",
        "Orpheus",
        Some(Arcana::Fool),
        100,
        50,
        15,
        8,
        vec![
            skill(
                "Golpe",
                0,
                1.0,
                "Un ataque básico",
                Elemento::Fisico,
                Efecto::Danio,
            ),
            skill(
                "Agi",
                4,
                1.4,
                "Un destello de fuego",
                Elemento::Fuego,
                Efecto::Danio,
            ),
            skill(
                "Dia",
                5,
                0.3,
                "Restaura un 30% del HP",
                Elemento::Fisico,
                Efecto::Curacion,
            ),
            skill(
                "Tarunda",
                6,
                0.0,
                "Baja el ataque enemigo",
                Elemento::Fisico,
                Efecto::DebuffAtaque,
            ),
        ],
        vec![Elemento::Electrico],
        vec![],
    )
}

fn crear_yukari() -> Personaje {
    personaje(
        "Yukari",
        "Io",
        Some(Arcana::Lovers),
        85,
        60,
        12,
        6,
        vec![
            skill(
                "Golpe",
                0,
                1.0,
                "Un ataque básico",
                Elemento::Fisico,
                Efecto::Danio,
            ),
            skill(
                "Garu",
                4,
                1.4,
                "Una ráfaga de viento",
                Elemento::Viento,
                Efecto::Danio,
            ),
            skill(
                "Dia",
                5,
                0.3,
                "Restaura un 30% del HP",
                Elemento::Fisico,
                Efecto::Curacion,
            ),
            skill(
                "Media",
                10,
                0.45,
                "Restaura un 45% del HP",
                Elemento::Fisico,
                Efecto::Curacion,
            ),
        ],
        vec![Elemento::Electrico],
        vec![],
    )
}

fn crear_junpei() -> Personaje {
    personaje(
        "Junpei",
        "Hermes",
        Some(Arcana::Magician),
        115,
        35,
        17,
        9,
        vec![
            skill(
                "Golpe",
                0,
                1.0,
                "Un ataque básico",
                Elemento::Fisico,
                Efecto::Danio,
            ),
            skill(
                "Puñetazo",
                5,
                1.8,
                "Un golpe poderoso",
                Elemento::Fisico,
                Efecto::Danio,
            ),
            skill(
                "Agi",
                4,
                1.3,
                "Un destello de fuego",
                Elemento::Fuego,
                Efecto::Danio,
            ),
            skill(
                "Tarukaja",
                6,
                0.0,
                "Sube tu ataque",
                Elemento::Fisico,
                Efecto::BuffAtaque,
            ),
        ],
        vec![Elemento::Viento],
        vec![Elemento::Fuego],
    )
}

fn crear_akihiko() -> Personaje {
    personaje(
        "Akihiko",
        "Polydeuces",
        Some(Arcana::Star),
        105,
        45,
        18,
        8,
        vec![
            skill(
                "Golpe",
                0,
                1.0,
                "Un ataque básico",
                Elemento::Fisico,
                Efecto::Danio,
            ),
            skill(
                "Zio",
                5,
                1.5,
                "Una descarga eléctrica",
                Elemento::Electrico,
                Efecto::Danio,
            ),
            skill(
                "Mazio",
                8,
                1.2,
                "Electricidad potente",
                Elemento::Electrico,
                Efecto::Danio,
            ),
            skill(
                "Rakukaja",
                6,
                0.0,
                "Sube tu defensa",
                Elemento::Fisico,
                Efecto::BuffDefensa,
            ),
        ],
        vec![Elemento::Hielo],
        vec![Elemento::Electrico],
    )
}

fn crear_mitsuru() -> Personaje {
    personaje(
        "Mitsuru",
        "Penthesilea",
        Some(Arcana::Empress),
        90,
        65,
        13,
        7,
        vec![
            skill(
                "Golpe",
                0,
                1.0,
                "Un ataque básico",
                Elemento::Fisico,
                Efecto::Danio,
            ),
            skill(
                "Bufu",
                5,
                1.5,
                "Escarcha cortante",
                Elemento::Hielo,
                Efecto::Danio,
            ),
            skill(
                "Mabufu",
                8,
                1.2,
                "Escarcha sobre el enemigo",
                Elemento::Hielo,
                Efecto::Danio,
            ),
            skill(
                "Rakukaja",
                6,
                0.0,
                "Sube tu defensa",
                Elemento::Fisico,
                Efecto::BuffDefensa,
            ),
        ],
        vec![Elemento::Fuego],
        vec![Elemento::Hielo],
    )
}

pub fn roster_personas() -> Vec<Personaje> {
    vec![
        roster_persona(
            "Orpheus",
            Arcana::Fool,
            1,
            100,
            50,
            15,
            8,
            vec![
                skill(
                    "Golpe",
                    0,
                    1.0,
                    "Un ataque básico",
                    Elemento::Fisico,
                    Efecto::Danio,
                ),
                skill(
                    "Agi",
                    4,
                    1.4,
                    "Un destello de fuego",
                    Elemento::Fuego,
                    Efecto::Danio,
                ),
                skill(
                    "Dia",
                    5,
                    0.3,
                    "Restaura un 30% del HP",
                    Elemento::Fisico,
                    Efecto::Curacion,
                ),
            ],
            vec![Elemento::Electrico],
            vec![],
        ),
        roster_persona(
            "Pixie",
            Arcana::Lovers,
            2,
            80,
            60,
            11,
            7,
            vec![
                skill(
                    "Garu",
                    4,
                    1.4,
                    "Una ráfaga de viento",
                    Elemento::Viento,
                    Efecto::Danio,
                ),
                skill(
                    "Zio",
                    5,
                    1.5,
                    "Una descarga eléctrica",
                    Elemento::Electrico,
                    Efecto::Danio,
                ),
                skill(
                    "Dia",
                    5,
                    0.3,
                    "Restaura un 30% del HP",
                    Elemento::Fisico,
                    Efecto::Curacion,
                ),
            ],
            vec![Elemento::Electrico],
            vec![Elemento::Viento],
        ),
        roster_persona(
            "Apsaras",
            Arcana::Priestess,
            3,
            85,
            60,
            10,
            7,
            vec![
                skill(
                    "Bufu",
                    5,
                    1.5,
                    "Escarcha cortante",
                    Elemento::Hielo,
                    Efecto::Danio,
                ),
                skill(
                    "Dia",
                    5,
                    0.3,
                    "Restaura un 30% del HP",
                    Elemento::Fisico,
                    Efecto::Curacion,
                ),
            ],
            vec![Elemento::Fuego],
            vec![Elemento::Hielo],
        ),
        roster_persona(
            "Angel",
            Arcana::Justice,
            4,
            90,
            55,
            12,
            8,
            vec![
                skill(
                    "Zio",
                    5,
                    1.5,
                    "Una descarga eléctrica",
                    Elemento::Electrico,
                    Efecto::Danio,
                ),
                skill(
                    "Dia",
                    5,
                    0.3,
                    "Restaura un 30% del HP",
                    Elemento::Fisico,
                    Efecto::Curacion,
                ),
                skill(
                    "Tarunda",
                    6,
                    0.0,
                    "Baja el ataque enemigo",
                    Elemento::Fisico,
                    Efecto::DebuffAtaque,
                ),
            ],
            vec![Elemento::Hielo],
            vec![],
        ),
        roster_persona(
            "Nekomata",
            Arcana::Magician,
            5,
            95,
            55,
            13,
            8,
            vec![
                skill(
                    "Agi",
                    4,
                    1.4,
                    "Un destello de fuego",
                    Elemento::Fuego,
                    Efecto::Danio,
                ),
                skill(
                    "Zio",
                    5,
                    1.5,
                    "Una descarga eléctrica",
                    Elemento::Electrico,
                    Efecto::Danio,
                ),
                skill(
                    "Tarukaja",
                    6,
                    0.0,
                    "Sube tu ataque",
                    Elemento::Fisico,
                    Efecto::BuffAtaque,
                ),
            ],
            vec![Elemento::Viento],
            vec![],
        ),
        roster_persona(
            "Alp",
            Arcana::Lovers,
            6,
            90,
            65,
            11,
            7,
            vec![
                skill(
                    "Garu",
                    4,
                    1.4,
                    "Una ráfaga de viento",
                    Elemento::Viento,
                    Efecto::Danio,
                ),
                skill(
                    "Dia",
                    5,
                    0.3,
                    "Restaura un 30% del HP",
                    Elemento::Fisico,
                    Efecto::Curacion,
                ),
                skill(
                    "Aullido",
                    6,
                    0.0,
                    "Reduce la defensa del rival",
                    Elemento::Fisico,
                    Efecto::DebuffDefensa,
                ),
            ],
            vec![Elemento::Fuego],
            vec![],
        ),
        roster_persona(
            "Silky",
            Arcana::Priestess,
            7,
            95,
            70,
            12,
            7,
            vec![
                skill(
                    "Bufu",
                    5,
                    1.5,
                    "Escarcha cortante",
                    Elemento::Hielo,
                    Efecto::Danio,
                ),
                skill(
                    "Dia",
                    5,
                    0.3,
                    "Restaura un 30% del HP",
                    Elemento::Fisico,
                    Efecto::Curacion,
                ),
                skill(
                    "Aullido",
                    6,
                    0.0,
                    "Reduce la defensa del rival",
                    Elemento::Fisico,
                    Efecto::DebuffDefensa,
                ),
            ],
            vec![Elemento::Fuego],
            vec![Elemento::Hielo],
        ),
        roster_persona(
            "Jack Frost",
            Arcana::Magician,
            8,
            95,
            75,
            13,
            9,
            vec![
                skill(
                    "Bufu",
                    5,
                    1.5,
                    "Escarcha cortante",
                    Elemento::Hielo,
                    Efecto::Danio,
                ),
                skill(
                    "Mabufu",
                    8,
                    1.2,
                    "Escarcha sobre el enemigo",
                    Elemento::Hielo,
                    Efecto::Danio,
                ),
                skill(
                    "Rakukaja",
                    6,
                    0.0,
                    "Sube tu defensa",
                    Elemento::Fisico,
                    Efecto::BuffDefensa,
                ),
            ],
            vec![Elemento::Fuego],
            vec![Elemento::Hielo],
        ),
        roster_persona(
            "Chimera",
            Arcana::Chariot,
            9,
            130,
            40,
            17,
            10,
            vec![
                skill(
                    "Golpe",
                    0,
                    1.0,
                    "Un ataque básico",
                    Elemento::Fisico,
                    Efecto::Danio,
                ),
                skill(
                    "Puñetazo",
                    5,
                    1.8,
                    "Un golpe poderoso",
                    Elemento::Fisico,
                    Efecto::Danio,
                ),
                skill(
                    "Aullido",
                    6,
                    0.0,
                    "Reduce la defensa del rival",
                    Elemento::Fisico,
                    Efecto::DebuffDefensa,
                ),
            ],
            vec![Elemento::Viento],
            vec![Elemento::Fisico],
        ),
        roster_persona(
            "Slime",
            Arcana::Fool,
            10,
            120,
            40,
            12,
            9,
            vec![
                skill(
                    "Golpe",
                    0,
                    1.0,
                    "Un ataque básico",
                    Elemento::Fisico,
                    Efecto::Danio,
                ),
                skill(
                    "Aullido",
                    6,
                    0.0,
                    "Reduce la defensa del rival",
                    Elemento::Fisico,
                    Efecto::DebuffDefensa,
                ),
            ],
            vec![Elemento::Fuego],
            vec![],
        ),
        roster_persona(
            "Unicorn",
            Arcana::Priestess,
            11,
            100,
            65,
            13,
            9,
            vec![
                skill(
                    "Zio",
                    5,
                    1.5,
                    "Una descarga eléctrica",
                    Elemento::Electrico,
                    Efecto::Danio,
                ),
                skill(
                    "Dia",
                    5,
                    0.3,
                    "Restaura un 30% del HP",
                    Elemento::Fisico,
                    Efecto::Curacion,
                ),
                skill(
                    "Aullido",
                    6,
                    0.0,
                    "Reduce la defensa del rival",
                    Elemento::Fisico,
                    Efecto::DebuffDefensa,
                ),
            ],
            vec![Elemento::Hielo],
            vec![],
        ),
        roster_persona(
            "Genbu",
            Arcana::Hermit,
            12,
            110,
            60,
            12,
            11,
            vec![
                skill(
                    "Bufu",
                    5,
                    1.5,
                    "Escarcha cortante",
                    Elemento::Hielo,
                    Efecto::Danio,
                ),
                skill(
                    "Rakukaja",
                    6,
                    0.0,
                    "Sube tu defensa",
                    Elemento::Fisico,
                    Efecto::BuffDefensa,
                ),
                skill(
                    "Aullido",
                    6,
                    0.0,
                    "Reduce la defensa del rival",
                    Elemento::Fisico,
                    Efecto::DebuffDefensa,
                ),
            ],
            vec![Elemento::Fuego],
            vec![Elemento::Hielo],
        ),
        roster_persona(
            "Berith",
            Arcana::Hierophant,
            13,
            120,
            50,
            16,
            10,
            vec![
                skill(
                    "Golpe",
                    0,
                    1.0,
                    "Un ataque básico",
                    Elemento::Fisico,
                    Efecto::Danio,
                ),
                skill(
                    "Puñetazo",
                    5,
                    1.8,
                    "Un golpe poderoso",
                    Elemento::Fisico,
                    Efecto::Danio,
                ),
                skill(
                    "Agi",
                    4,
                    1.4,
                    "Un destello de fuego",
                    Elemento::Fuego,
                    Efecto::Danio,
                ),
            ],
            vec![Elemento::Hielo],
            vec![],
        ),
        roster_persona(
            "Pyro Jack",
            Arcana::Magician,
            14,
            100,
            60,
            14,
            8,
            vec![
                skill(
                    "Agi",
                    4,
                    1.4,
                    "Un destello de fuego",
                    Elemento::Fuego,
                    Efecto::Danio,
                ),
                skill(
                    "Maragi",
                    8,
                    1.2,
                    "Fuego sobre el enemigo",
                    Elemento::Fuego,
                    Efecto::Danio,
                ),
                skill(
                    "Tarukaja",
                    6,
                    0.0,
                    "Sube tu ataque",
                    Elemento::Fisico,
                    Efecto::BuffAtaque,
                ),
            ],
            vec![Elemento::Hielo],
            vec![Elemento::Fuego],
        ),
        roster_persona(
            "Forneus",
            Arcana::Emperor,
            17,
            125,
            55,
            17,
            11,
            vec![
                skill(
                    "Golpe",
                    0,
                    1.0,
                    "Un ataque básico",
                    Elemento::Fisico,
                    Efecto::Danio,
                ),
                skill(
                    "Agi",
                    4,
                    1.4,
                    "Un destello de fuego",
                    Elemento::Fuego,
                    Efecto::Danio,
                ),
                skill(
                    "Maragi",
                    8,
                    1.2,
                    "Fuego sobre el enemigo",
                    Elemento::Fuego,
                    Efecto::Danio,
                ),
                skill(
                    "Tarukaja",
                    6,
                    0.0,
                    "Sube tu ataque",
                    Elemento::Fisico,
                    Efecto::BuffAtaque,
                ),
            ],
            vec![Elemento::Viento],
            vec![],
        ),
        roster_persona(
            "Hua Po",
            Arcana::Magician,
            20,
            105,
            75,
            15,
            9,
            vec![
                skill(
                    "Agi",
                    4,
                    1.4,
                    "Un destello de fuego",
                    Elemento::Fuego,
                    Efecto::Danio,
                ),
                skill(
                    "Maragi",
                    8,
                    1.2,
                    "Fuego sobre el enemigo",
                    Elemento::Fuego,
                    Efecto::Danio,
                ),
                skill(
                    "Dia",
                    5,
                    0.3,
                    "Restaura un 30% del HP",
                    Elemento::Fisico,
                    Efecto::Curacion,
                ),
                skill(
                    "Tarunda",
                    6,
                    0.0,
                    "Baja el ataque enemigo",
                    Elemento::Fisico,
                    Efecto::DebuffAtaque,
                ),
            ],
            vec![Elemento::Hielo],
            vec![Elemento::Fuego],
        ),
    ]
}

pub fn persona_orfeo() -> Personaje {
    roster_personas().remove(0)
}

pub fn arcana_resultado(a: Arcana, b: Arcana) -> Arcana {
    if a == b {
        return a;
    }
    use Arcana::*;
    match (a, b) {
        (Fool, Magician) | (Magician, Fool) => Hierophant,
        (Fool, Priestess) | (Priestess, Fool) => Magician,
        (Fool, Empress) | (Empress, Fool) => Star,
        (Fool, Emperor) | (Emperor, Fool) => Temperance,
        (Fool, Hierophant) | (Hierophant, Fool) => Hanged,
        (Fool, Lovers) | (Lovers, Fool) => Justice,
        (Fool, Chariot) | (Chariot, Fool) => Emperor,
        (Fool, Justice) | (Justice, Fool) => Lovers,
        (Fool, Hermit) | (Hermit, Fool) => Priestess,
        (Fool, Star) | (Star, Fool) => Devil,
        (Magician, Priestess) | (Priestess, Magician) => Justice,
        (Magician, Empress) | (Empress, Magician) => Hanged,
        (Magician, Emperor) | (Emperor, Magician) => Lovers,
        (Magician, Hierophant) | (Hierophant, Magician) => Hermit,
        (Magician, Lovers) | (Lovers, Magician) => Chariot,
        (Magician, Chariot) | (Chariot, Magician) => Devil,
        (Magician, Justice) | (Justice, Magician) => Hierophant,
        (Magician, Hermit) | (Hermit, Magician) => Moon,
        (Magician, Star) | (Star, Magician) => Strength,
        (Priestess, Empress) | (Empress, Priestess) => Temperance,
        (Priestess, Emperor) | (Emperor, Priestess) => Justice,
        (Priestess, Hierophant) | (Hierophant, Priestess) => Lovers,
        (Priestess, Lovers) | (Lovers, Priestess) => Magician,
        (Priestess, Chariot) | (Chariot, Priestess) => Fool,
        (Priestess, Justice) | (Justice, Priestess) => Lovers,
        (Priestess, Hermit) | (Hermit, Priestess) => Strength,
        (Priestess, Star) | (Star, Priestess) => Emperor,
        (Empress, Emperor) | (Emperor, Empress) => Chariot,
        (Empress, Hierophant) | (Hierophant, Empress) => Tower,
        (Empress, Lovers) | (Lovers, Empress) => Moon,
        (Empress, Chariot) | (Chariot, Empress) => Hermit,
        (Empress, Justice) | (Justice, Empress) => Emperor,
        (Empress, Hermit) | (Hermit, Empress) => Sun,
        (Empress, Star) | (Star, Empress) => Priestess,
        (Emperor, Hierophant) | (Hierophant, Emperor) => Strength,
        (Emperor, Lovers) | (Lovers, Emperor) => Chariot,
        (Emperor, Chariot) | (Chariot, Emperor) => Devil,
        (Emperor, Justice) | (Justice, Emperor) => Hanged,
        (Emperor, Hermit) | (Hermit, Emperor) => Hierophant,
        (Emperor, Star) | (Star, Emperor) => Hierophant,
        (Hierophant, Lovers) | (Lovers, Hierophant) => Magician,
        (Hierophant, Chariot) | (Chariot, Hierophant) => Justice,
        (Hierophant, Justice) | (Justice, Hierophant) => Fool,
        (Hierophant, Hermit) | (Hermit, Hierophant) => Chariot,
        (Hierophant, Star) | (Star, Hierophant) => Moon,
        (Lovers, Chariot) | (Chariot, Lovers) => Priestess,
        (Lovers, Justice) | (Justice, Lovers) => Emperor,
        (Lovers, Hermit) | (Hermit, Lovers) => Fool,
        (Lovers, Star) | (Star, Lovers) => Death,
        (Chariot, Justice) | (Justice, Chariot) => Magician,
        (Chariot, Hermit) | (Hermit, Chariot) => Lovers,
        (Chariot, Star) | (Star, Chariot) => Fortune,
        (Justice, Hermit) | (Hermit, Justice) => Magician,
        (Justice, Star) | (Star, Justice) => Hermit,
        (Hermit, Star) | (Star, Hermit) => Fool,
        (a, _) => a,
    }
}

fn heredar_habilidades(resultado: &mut Personaje, a: &Personaje, b: &Personaje) {
    let limite = 5;
    let mut disponibles: Vec<Skill> = a
        .habilidades
        .iter()
        .chain(b.habilidades.iter())
        .filter(|h| !resultado.habilidades.iter().any(|r| r.nombre == h.nombre))
        .cloned()
        .collect();
    disponibles.sort_by(|x, y| {
        y.multiplicador_dano
            .partial_cmp(&x.multiplicador_dano)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    for hab in disponibles {
        if resultado.habilidades.len() >= limite {
            break;
        }
        resultado.habilidades.push(hab);
    }
}

pub fn fusionar(a: &Personaje, b: &Personaje) -> Personaje {
    let (Some(arcana_a), Some(arcana_b)) = (a.arcana, b.arcana) else {
        return a.clone();
    };
    let arcana = arcana_resultado(arcana_a, arcana_b);
    let objetivo = (a.nivel + b.nivel).div_ceil(2);

    let mut candidatas: Vec<Personaje> = roster_personas()
        .into_iter()
        .filter(|p| p.arcana == Some(arcana))
        .collect();
    if candidatas.is_empty() {
        candidatas = roster_personas();
    }

    let elegida = if arcana_a == arcana_b {
        let maximo = a.nivel.max(b.nivel);
        candidatas
            .iter()
            .filter(|p| p.nivel < maximo)
            .max_by_key(|p| p.nivel)
            .or_else(|| candidatas.iter().min_by_key(|p| p.nivel))
    } else {
        candidatas
            .iter()
            .filter(|p| p.nivel >= objetivo)
            .min_by_key(|p| p.nivel)
            .or_else(|| candidatas.iter().max_by_key(|p| p.nivel))
    };

    let Some(elegida) = elegida else {
        return a.clone();
    };
    let mut resultado = elegida.clone();
    heredar_habilidades(&mut resultado, a, b);
    resultado
}

fn persona_aleatoria(nivel: u32) -> Personaje {
    let mut rng = rand::thread_rng();
    let pool: Vec<Personaje> = roster_personas()
        .into_iter()
        .filter(|p| p.nivel <= nivel + 4)
        .collect();
    let pool = if pool.is_empty() {
        roster_personas()
    } else {
        pool
    };
    pool[rng.gen_range(0..pool.len())].clone()
}

pub fn generar_cartas(nivel: u32) -> Vec<CartaShuffle> {
    let mut rng = rand::thread_rng();
    (0..3)
        .map(|_| {
            let tirada: u32 = rng.gen_range(0..100);
            if tirada < 50 {
                CartaShuffle::Persona(persona_aleatoria(nivel))
            } else if tirada < 70 {
                CartaShuffle::Copa
            } else if tirada < 85 {
                CartaShuffle::Vara
            } else {
                CartaShuffle::Vacia
            }
        })
        .collect()
}

pub fn aplicar_carta(
    carta: CartaShuffle,
    jugador: &mut Personaje,
    stock: &mut Vec<Personaje>,
) -> String {
    let mensaje = match carta {
        CartaShuffle::Persona(persona) => {
            if stock.iter().any(|p| p.persona == persona.persona) {
                let exp = 15;
                jugador.experiencia += exp;
                format!(
                    "Ya posees a {}. Se convierte en {} EXP.",
                    persona.persona, exp
                )
            } else {
                stock.push(persona.clone());
                format!("¡Obtuviste a {} (Nv.{})!", persona.persona, persona.nivel)
            }
        }
        CartaShuffle::Copa => {
            let curacion = (jugador.hp_max as f32 * 0.4) as u32;
            jugador.hp = (jugador.hp + curacion).min(jugador.hp_max);
            jugador.mp = (jugador.mp + 15).min(jugador.mp_max);
            format!("Copa: recuperas {} HP y 15 MP.", curacion)
        }
        CartaShuffle::Vara => {
            let exp = 25;
            jugador.experiencia += exp;
            format!("Vara: ganas {} EXP.", exp)
        }
        CartaShuffle::Vacia => String::from("Carta vacía... no obtuviste nada."),
    };
    if subir_nivel(jugador) {
        format!("{} ¡Subiste a nivel {}!", mensaje, jugador.nivel)
    } else {
        mensaje
    }
}

#[expect(
    clippy::too_many_arguments,
    reason = "constructor de datos de enemigos con una fila por enemigo"
)]
fn enemigo_nivel(
    nombre: &str,
    nivel: u32,
    mod_hp: u32,
    mod_atk: u32,
    mod_def: u32,
    mod_mp: u32,
    habilidades: Vec<Skill>,
    debilidades: Vec<Elemento>,
    resistencias: Vec<Elemento>,
) -> Enemigo {
    let hp = 30 + nivel * 15 + mod_hp;
    Enemigo {
        personaje: Personaje {
            nombre: format!("{} Nv.{}", nombre, nivel),
            persona: String::from("Sombra"),
            arcana: None,
            hp,
            hp_max: hp,
            mp: 10 + mod_mp,
            mp_max: 10 + mod_mp,
            ataque: 5 + nivel * 3 + mod_atk,
            defensa: 3 + nivel * 2 + mod_def,
            nivel,
            experiencia: 0,
            habilidades,
            debilidades,
            resistencias,
        },
    }
}

pub fn crear_enemigos_nivel(nivel: u32) -> Vec<Enemigo> {
    let golpe = || {
        skill(
            "Golpe",
            0,
            1.0,
            "Un ataque básico",
            Elemento::Fisico,
            Efecto::Danio,
        )
    };
    let aullido = || {
        skill(
            "Aullido",
            6,
            0.0,
            "Reduce la defensa del rival",
            Elemento::Fisico,
            Efecto::DebuffDefensa,
        )
    };

    let mut enemigos = vec![
        enemigo_nivel(
            "Maya Cobarde",
            nivel,
            0,
            0,
            0,
            0,
            vec![golpe()],
            vec![Elemento::Fuego],
            vec![],
        ),
        enemigo_nivel(
            "Maya Implacable",
            nivel,
            20,
            2,
            1,
            5,
            vec![
                golpe(),
                skill(
                    "Bola Oscura",
                    4,
                    1.3,
                    "Una esfera de energía",
                    Elemento::Fisico,
                    Efecto::Danio,
                ),
            ],
            vec![Elemento::Hielo],
            vec![],
        ),
        enemigo_nivel(
            "Jack Frost",
            nivel,
            20,
            2,
            2,
            20,
            vec![
                skill(
                    "Bufu",
                    4,
                    1.5,
                    "Escarcha cortante",
                    Elemento::Hielo,
                    Efecto::Danio,
                ),
                skill(
                    "Mabufu",
                    8,
                    1.2,
                    "Escarcha sobre todos",
                    Elemento::Hielo,
                    Efecto::Danio,
                ),
            ],
            vec![Elemento::Fuego],
            vec![Elemento::Hielo],
        ),
        enemigo_nivel(
            "Pyro Jack",
            nivel,
            15,
            2,
            1,
            15,
            vec![
                skill(
                    "Agi",
                    4,
                    1.4,
                    "Un destello de fuego",
                    Elemento::Fuego,
                    Efecto::Danio,
                ),
                skill(
                    "Maragi",
                    7,
                    1.2,
                    "Fuego sobre todos",
                    Elemento::Fuego,
                    Efecto::Danio,
                ),
            ],
            vec![Elemento::Hielo],
            vec![Elemento::Fuego],
        ),
        enemigo_nivel(
            "Pixie",
            nivel,
            10,
            1,
            0,
            10,
            vec![
                skill(
                    "Garu",
                    4,
                    1.4,
                    "Una ráfaga de viento",
                    Elemento::Viento,
                    Efecto::Danio,
                ),
                skill(
                    "Dia",
                    5,
                    0.3,
                    "Restaura un 30% del HP",
                    Elemento::Fisico,
                    Efecto::Curacion,
                ),
            ],
            vec![Elemento::Electrico],
            vec![Elemento::Viento],
        ),
        enemigo_nivel(
            "Mesa Reidora",
            nivel,
            10,
            0,
            0,
            5,
            vec![golpe(), aullido()],
            vec![Elemento::Viento],
            vec![],
        ),
    ];

    if nivel >= 3 {
        enemigos.push(enemigo_nivel(
            "Tiara Llorosa",
            nivel,
            30,
            3,
            2,
            10,
            vec![
                skill(
                    "Garras",
                    5,
                    1.6,
                    "Garras afiladas",
                    Elemento::Fisico,
                    Efecto::Danio,
                ),
                aullido(),
            ],
            vec![Elemento::Fuego],
            vec![],
        ));
        enemigos.push(enemigo_nivel(
            "Cuervo Negro",
            nivel,
            25,
            3,
            1,
            10,
            vec![
                skill(
                    "Garu",
                    4,
                    1.4,
                    "Una ráfaga de viento",
                    Elemento::Viento,
                    Efecto::Danio,
                ),
                skill(
                    "Magaru",
                    7,
                    1.2,
                    "Viento sobre todos",
                    Elemento::Viento,
                    Efecto::Danio,
                ),
            ],
            vec![Elemento::Electrico],
            vec![Elemento::Viento],
        ));
        enemigos.push(enemigo_nivel(
            "Libro Maníaco",
            nivel,
            40,
            4,
            2,
            10,
            vec![
                skill(
                    "Agi",
                    4,
                    1.4,
                    "Un destello de fuego",
                    Elemento::Fuego,
                    Efecto::Danio,
                ),
                aullido(),
            ],
            vec![Elemento::Fuego],
            vec![],
        ));
        enemigos.push(enemigo_nivel(
            "Lilim",
            nivel,
            35,
            3,
            1,
            15,
            vec![
                skill(
                    "Zio",
                    5,
                    1.5,
                    "Una descarga eléctrica",
                    Elemento::Electrico,
                    Efecto::Danio,
                ),
                aullido(),
            ],
            vec![Elemento::Hielo],
            vec![],
        ));
    }

    if nivel >= 5 {
        enemigos.push(enemigo_nivel(
            "Naga",
            nivel,
            40,
            4,
            3,
            15,
            vec![
                skill(
                    "Zio",
                    5,
                    1.5,
                    "Una descarga eléctrica",
                    Elemento::Electrico,
                    Efecto::Danio,
                ),
                skill(
                    "Mazio",
                    8,
                    1.2,
                    "Electricidad potente",
                    Elemento::Electrico,
                    Efecto::Danio,
                ),
            ],
            vec![Elemento::Hielo],
            vec![Elemento::Electrico],
        ));
        enemigos.push(enemigo_nivel(
            "Súcubo",
            nivel,
            35,
            4,
            2,
            15,
            vec![
                skill(
                    "Bufu",
                    5,
                    1.5,
                    "Escarcha cortante",
                    Elemento::Hielo,
                    Efecto::Danio,
                ),
                aullido(),
            ],
            vec![Elemento::Fuego],
            vec![],
        ));
        enemigos.push(enemigo_nivel(
            "Quimera",
            nivel,
            60,
            5,
            4,
            10,
            vec![
                skill(
                    "Puñetazo",
                    6,
                    1.8,
                    "Un golpe poderoso",
                    Elemento::Fisico,
                    Efecto::Danio,
                ),
                aullido(),
            ],
            vec![Elemento::Viento],
            vec![Elemento::Fisico],
        ));
    }

    enemigos
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

    #[test]
    fn akihiko_tiene_polydeuces_y_habilidades_de_electricidad() {
        let akihiko = crear_personajes()[3].clone();
        assert_eq!(akihiko.persona, "Polydeuces");
        assert!(akihiko.habilidades.iter().any(|h| h.nombre == "Zio"));
        assert!(akihiko.resistencias.contains(&Elemento::Electrico));
        assert!(akihiko.debilidades.contains(&Elemento::Hielo));
    }

    #[test]
    fn jack_frost_tiene_habilidades_de_hielo() {
        let jack_frost = roster_personas()
            .into_iter()
            .find(|p| p.persona == "Jack Frost")
            .expect("Jack Frost debe estar en el roster");
        assert!(jack_frost.habilidades.iter().any(|h| h.nombre == "Bufu"));
        assert!(jack_frost.habilidades.iter().any(|h| h.nombre == "Mabufu"));
    }

    #[test]
    fn arcanos_diferentes_producen_el_arcanum_de_la_tabla() {
        assert_eq!(
            arcana_resultado(Arcana::Fool, Arcana::Magician),
            Arcana::Hierophant
        );
        assert_eq!(
            arcana_resultado(Arcana::Magician, Arcana::Lovers),
            Arcana::Chariot
        );
    }

    #[test]
    fn fusionar_jack_frost_y_pixie_da_quimera_con_habilidades_heredadas() {
        let personas = roster_personas();
        let jack_frost = personas
            .iter()
            .find(|p| p.persona == "Jack Frost")
            .expect("Jack Frost debe estar en el roster");
        let pixie = personas
            .iter()
            .find(|p| p.persona == "Pixie")
            .expect("Pixie debe estar en el roster");
        let resultado = fusionar(jack_frost, pixie);
        assert_eq!(resultado.persona, "Chimera");
        assert_eq!(resultado.nivel, 9);
        assert!(resultado.habilidades.iter().any(|h| h.nombre == "Bufu"));
        assert!(resultado.habilidades.iter().any(|h| h.nombre == "Zio"));
    }

    #[test]
    fn fusionar_personas_del_mismo_arcanum_no_puede_subir_de_nivel() {
        let personas = roster_personas();
        let jack_frost = personas
            .iter()
            .find(|p| p.persona == "Jack Frost")
            .expect("Jack Frost debe estar en el roster");
        let pyro_jack = personas
            .iter()
            .find(|p| p.persona == "Pyro Jack")
            .expect("Pyro Jack debe estar en el roster");
        let resultado = fusionar(jack_frost, pyro_jack);
        assert_eq!(resultado.persona, "Jack Frost");
        assert!(resultado.nivel < 14);
    }

    #[test]
    fn cartas_de_shuffle_solo_ofrecen_personas_de_nivel_apropiado() {
        let mut rng = rand::thread_rng();
        for _ in 0..50 {
            let nivel: u32 = rng.gen_range(1..10);
            let pool: Vec<Personaje> = roster_personas()
                .into_iter()
                .filter(|p| p.nivel <= nivel + 4)
                .collect();
            assert!(!pool.is_empty());
            assert!(pool.iter().all(|p| p.nivel <= nivel + 4));
        }
    }

    #[test]
    fn carta_de_persona_nueva_se_agrega_al_stock() {
        let mut jugador = crear_jugador();
        let mut stock = vec![persona_orfeo()];
        let pixie = roster_personas()
            .into_iter()
            .find(|p| p.persona == "Pixie")
            .expect("Pixie debe estar en el roster");
        let mensaje = aplicar_carta(
            CartaShuffle::Persona(pixie.clone()),
            &mut jugador,
            &mut stock,
        );
        assert_eq!(stock.len(), 2);
        assert!(mensaje.contains("Pixie"));
    }
}
