//! Paleta de colores inspirada en la *Dark Hour* de Persona 3.
//!
//! Todos los colores del juego viven en este módulo para poder ajustar
//! la estética desde un solo lugar: azules del tema, colores de los
//! elementos de combate, arcanos, personajes y barras de estado.

use crate::data::{Arcana, CartaShuffle, Elemento};
use crossterm::style::Color;

// ─── Colores base de la interfaz ────────────────────────────────
/// Azul principal de marcos y bordes.
pub const AZUL_MARCO: Color = Color::Rgb {
    r: 82,
    g: 170,
    b: 255,
};
/// Azul claro para títulos y acentos.
pub const AZUL_TITULO: Color = Color::Rgb {
    r: 152,
    g: 216,
    b: 255,
};
/// Azul tenue para texto secundario.
pub const AZUL_SUAVE: Color = Color::Rgb {
    r: 104,
    g: 140,
    b: 200,
};
/// Blanco suave para texto normal.
pub const BLANCO_SUAVE: Color = Color::Rgb {
    r: 228,
    g: 232,
    b: 240,
};
/// Gris para texto desactivado o pistas.
pub const GRIS_SUAVE: Color = Color::Rgb {
    r: 132,
    g: 144,
    b: 162,
};
/// Luna llena de la Dark Hour.
pub const LUNA_P3: Color = Color::Rgb {
    r: 206,
    g: 226,
    b: 255,
};
/// Cielo nocturno (torre de Tartarus).
pub const CIELO_NOCTURNO: Color = Color::Rgb {
    r: 72,
    g: 92,
    b: 150,
};
/// Estrellas del cielo nocturno.
pub const ESTRELLA: Color = Color::Rgb {
    r: 150,
    g: 180,
    b: 240,
};
/// Fondo del elemento seleccionado en los menús.
pub const FONDO_SELECCION: Color = Color::Rgb {
    r: 26,
    g: 58,
    b: 122,
};
/// Texto del elemento seleccionado.
pub const TEXTO_SELECCION: Color = Color::Rgb {
    r: 255,
    g: 226,
    b: 132,
};
/// Verde de éxito / curación.
pub const VERDE_OK: Color = Color::Rgb {
    r: 128,
    g: 226,
    b: 150,
};
/// Púrpura de la Velvet Room.
pub const VIOLETA_VELVET: Color = Color::Rgb {
    r: 198,
    g: 134,
    b: 255,
};
/// Rojo de combate / peligro.
pub const ROJO_COMBATE: Color = Color::Rgb {
    r: 255,
    g: 86,
    b: 86,
};
/// Dorado de victoria.
pub const DORADO: Color = Color::Rgb {
    r: 255,
    g: 218,
    b: 122,
};
/// Azul de la barra de MP alta.
pub const MP_ALTO: Color = Color::Rgb {
    r: 110,
    g: 180,
    b: 255,
};
/// Azul de la barra de MP baja.
pub const MP_BAJO: Color = Color::Rgb {
    r: 72,
    g: 122,
    b: 210,
};

/// Color de un elemento de combate.
pub fn color_elemento(elemento: Elemento) -> Color {
    match elemento {
        Elemento::Fisico => Color::Rgb {
            r: 202,
            g: 206,
            b: 216,
        },
        Elemento::Fuego => Color::Rgb {
            r: 255,
            g: 96,
            b: 64,
        },
        Elemento::Hielo => Color::Rgb {
            r: 130,
            g: 214,
            b: 255,
        },
        Elemento::Viento => Color::Rgb {
            r: 128,
            g: 226,
            b: 150,
        },
        Elemento::Electrico => Color::Rgb {
            r: 255,
            g: 226,
            b: 96,
        },
    }
}

/// Color del arcano de una Persona.
pub fn color_arcana(arcana: Arcana) -> Color {
    use Arcana::*;
    match arcana {
        Fool => Color::Rgb {
            r: 210,
            g: 210,
            b: 222,
        },
        Magician => Color::Rgb {
            r: 255,
            g: 172,
            b: 72,
        },
        Priestess => Color::Rgb {
            r: 130,
            g: 220,
            b: 255,
        },
        Empress => Color::Rgb {
            r: 255,
            g: 132,
            b: 192,
        },
        Emperor => Color::Rgb {
            r: 255,
            g: 206,
            b: 92,
        },
        Hierophant => Color::Rgb {
            r: 192,
            g: 162,
            b: 255,
        },
        Lovers => Color::Rgb {
            r: 255,
            g: 146,
            b: 202,
        },
        Chariot => Color::Rgb {
            r: 255,
            g: 132,
            b: 92,
        },
        Justice => Color::Rgb {
            r: 162,
            g: 206,
            b: 255,
        },
        Hermit => Color::Rgb {
            r: 178,
            g: 182,
            b: 192,
        },
        Fortune => Color::Rgb {
            r: 255,
            g: 230,
            b: 132,
        },
        Strength => Color::Rgb {
            r: 255,
            g: 162,
            b: 152,
        },
        Hanged => Color::Rgb {
            r: 206,
            g: 132,
            b: 255,
        },
        Death => Color::Rgb {
            r: 152,
            g: 152,
            b: 162,
        },
        Temperance => Color::Rgb {
            r: 136,
            g: 220,
            b: 196,
        },
        Devil => Color::Rgb {
            r: 255,
            g: 106,
            b: 106,
        },
        Tower => Color::Rgb {
            r: 255,
            g: 96,
            b: 96,
        },
        Star => Color::Rgb {
            r: 255,
            g: 216,
            b: 102,
        },
        Moon => Color::Rgb {
            r: 166,
            g: 186,
            b: 255,
        },
        Sun => Color::Rgb {
            r: 255,
            g: 236,
            b: 136,
        },
    }
}

/// Color asociado a cada personaje del grupo.
pub fn color_personaje(nombre: &str) -> Color {
    match nombre {
        "Makoto" => Color::Rgb {
            r: 96,
            g: 176,
            b: 255,
        },
        "Yukari" => Color::Rgb {
            r: 136,
            g: 230,
            b: 162,
        },
        "Junpei" => Color::Rgb {
            r: 255,
            g: 166,
            b: 66,
        },
        "Akihiko" => Color::Rgb {
            r: 255,
            g: 126,
            b: 116,
        },
        "Mitsuru" => Color::Rgb {
            r: 255,
            g: 132,
            b: 196,
        },
        _ => BLANCO_SUAVE,
    }
}

/// Color de la barra de HP según el porcentaje restante.
pub fn color_hp(porcentaje: f32) -> Color {
    if porcentaje > 0.6 {
        Color::Rgb {
            r: 128,
            g: 232,
            b: 128,
        }
    } else if porcentaje > 0.3 {
        Color::Rgb {
            r: 255,
            g: 216,
            b: 96,
        }
    } else {
        Color::Rgb {
            r: 255,
            g: 92,
            b: 92,
        }
    }
}

/// Color de la barra de MP según el porcentaje restante.
pub fn color_mp(porcentaje: f32) -> Color {
    if porcentaje > 0.5 {
        MP_ALTO
    } else {
        MP_BAJO
    }
}

/// Color del sprite de una sombra (enemigo).
pub fn color_sprite_enemigo(nombre: &str) -> Color {
    match nombre.split(" Lv.").next().unwrap_or(nombre) {
        "Jack Frost" => Color::Rgb {
            r: 150,
            g: 220,
            b: 255,
        },
        "Pyro Jack" => Color::Rgb {
            r: 255,
            g: 122,
            b: 72,
        },
        "Pixie" => Color::Rgb {
            r: 255,
            g: 152,
            b: 222,
        },
        "Cowardly Maya" => Color::Rgb {
            r: 172,
            g: 172,
            b: 182,
        },
        "Belligerent Maya" => Color::Rgb {
            r: 255,
            g: 112,
            b: 92,
        },
        "Laughing Table" => Color::Rgb {
            r: 232,
            g: 202,
            b: 122,
        },
        "Black Raven" => Color::Rgb {
            r: 162,
            g: 162,
            b: 176,
        },
        "Maniac Book" => Color::Rgb {
            r: 202,
            g: 122,
            b: 232,
        },
        "Naga" => Color::Rgb {
            r: 122,
            g: 222,
            b: 132,
        },
        "Succubus" => Color::Rgb {
            r: 255,
            g: 132,
            b: 192,
        },
        "Chimera" => Color::Rgb {
            r: 255,
            g: 222,
            b: 112,
        },
        "Weeping Tiara" => Color::Rgb {
            r: 142,
            g: 212,
            b: 255,
        },
        "Lilim" => Color::Rgb {
            r: 255,
            g: 222,
            b: 132,
        },
        "Guillotine" => ROJO_COMBATE,
        _ => GRIS_SUAVE,
    }
}

/// Color del sprite de una Persona aliada.
pub fn color_sprite_persona(persona: &str) -> Color {
    match persona {
        "Jack Frost" => Color::Rgb {
            r: 150,
            g: 220,
            b: 255,
        },
        "Pyro Jack" => Color::Rgb {
            r: 255,
            g: 122,
            b: 72,
        },
        "Pixie" => Color::Rgb {
            r: 255,
            g: 152,
            b: 222,
        },
        "Orpheus" => Color::Rgb {
            r: 210,
            g: 214,
            b: 230,
        },
        _ => Color::Rgb {
            r: 202,
            g: 202,
            b: 224,
        },
    }
}

/// Color de una carta del Shuffle Time.
pub fn color_carta(carta: &CartaShuffle) -> Color {
    match carta {
        CartaShuffle::Persona(p) => p.arcana.map(color_arcana).unwrap_or(BLANCO_SUAVE),
        CartaShuffle::Copa => Color::Rgb {
            r: 128,
            g: 232,
            b: 150,
        },
        CartaShuffle::Vara => Color::Rgb {
            r: 255,
            g: 182,
            b: 92,
        },
        CartaShuffle::Vacia => GRIS_SUAVE,
    }
}
