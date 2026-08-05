use std::fs;
use std::io;
use std::path::Path;

use crate::data::{Arcana, Efecto, Elemento, Personaje, Skill};

const SEP_LINEA: char = '\t';
const SEP_PERSONA: char = '|';
const SEP_SKILL: char = ';';
const SEP_LISTA: char = ',';

pub fn guardar(ruta: &str, jugador: &Personaje, stock: &[Personaje], piso: u32) -> io::Result<()> {
    let mut contenido = String::new();
    contenido.push_str(&format!("TARTARUS\t{piso}\n"));
    contenido.push_str(&format!("PLAYER\t{}\n", serializar_personaje(jugador)));
    for persona in stock {
        contenido.push_str(&format!("STOCK\t{}\n", serializar_personaje(persona)));
    }
    fs::write(Path::new(ruta), contenido)
}

pub fn existe(ruta: &str) -> bool {
    Path::new(ruta).is_file()
}

pub fn cargar(ruta: &str) -> io::Result<(Personaje, Vec<Personaje>, u32)> {
    let contenido = fs::read_to_string(Path::new(ruta))?;
    let mut piso = None;
    let mut jugador = None;
    let mut stock = Vec::new();
    for linea in contenido.lines() {
        let (clave, valor) = linea
            .split_once(SEP_LINEA)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "línea malformada"))?;
        match clave {
            "TARTARUS" => {
                piso = valor.parse::<u32>().ok().or(Some(0));
            }
            "PLAYER" => {
                jugador = Some(deserializar_personaje(valor).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidData, "jugador inválido")
                })?);
            }
            "STOCK" => {
                let persona = deserializar_personaje(valor).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidData, "persona del stock inválida")
                })?;
                stock.push(persona);
            }
            _ => {}
        }
    }
    let jugador =
        jugador.ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "sin jugador"))?;
    Ok((jugador, stock, piso.unwrap_or(1)))
}

fn serializar_personaje(p: &Personaje) -> String {
    let habilidades: Vec<String> = p.habilidades.iter().map(serializar_habilidad).collect();
    format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
        p.nombre,
        p.persona,
        arcana_idx(p.arcana),
        p.hp,
        p.hp_max,
        p.mp,
        p.mp_max,
        p.ataque,
        p.defensa,
        p.nivel,
        p.experiencia,
        lista_elementos(&p.debilidades),
        lista_elementos(&p.resistencias),
        habilidades.join(&SEP_SKILL.to_string())
    )
}

fn serializar_habilidad(h: &Skill) -> String {
    format!(
        "{}{}{}{}{}{}{}{}{}{}{}",
        h.nombre,
        SEP_LINEA,
        h.coste_mp,
        SEP_LINEA,
        h.multiplicador_dano,
        SEP_LINEA,
        h.descripcion,
        SEP_LINEA,
        elemento_idx(h.elemento),
        SEP_LINEA,
        efecto_idx(h.efecto)
    )
}

// Una habilidad ocupa un solo campo de la línea de persona, separado de sus
// subcampos por tabulador; las habilidades del conjunto se separan con ';'.
fn deserializar_personaje(cuerpo: &str) -> Option<Personaje> {
    let campo: Vec<&str> = cuerpo.split(SEP_PERSONA).collect();
    if campo.len() != 14 {
        return None;
    }
    let skills = campo[13]
        .split(SEP_SKILL)
        .filter_map(deserializar_habilidad);
    Some(Personaje {
        nombre: campo[0].to_string(),
        persona: campo[1].to_string(),
        arcana: arcana_de(campo[2]),
        hp: campo[3].parse().ok()?,
        hp_max: campo[4].parse().ok()?,
        mp: campo[5].parse().ok()?,
        mp_max: campo[6].parse().ok()?,
        ataque: campo[7].parse().ok()?,
        defensa: campo[8].parse().ok()?,
        nivel: campo[9].parse().ok()?,
        experiencia: campo[10].parse().ok()?,
        habilidades: skills.collect(),
        debilidades: lista_de_elementos(campo[11]),
        resistencias: lista_de_elementos(campo[12]),
    })
}

fn deserializar_habilidad(token: &str) -> Option<Skill> {
    let mut partes = token.split(SEP_LINEA);
    Some(Skill {
        nombre: partes.next()?.to_string(),
        coste_mp: partes.next()?.parse().ok()?,
        multiplicador_dano: partes.next()?.parse().ok()?,
        descripcion: partes.next()?.to_string(),
        elemento: elemento_de(partes.next()?.parse().ok()?)?,
        efecto: efecto_de(partes.next()?.parse().ok()?)?,
    })
}

fn lista_elementos(elementos: &[Elemento]) -> String {
    elementos
        .iter()
        .map(|e| elemento_idx(*e).to_string())
        .collect::<Vec<_>>()
        .join(&SEP_LISTA.to_string())
}

fn lista_de_elementos(texto: &str) -> Vec<Elemento> {
    if texto.is_empty() {
        return Vec::new();
    }
    texto
        .split(SEP_LISTA)
        .filter_map(|n| n.parse::<u8>().ok())
        .filter_map(elemento_de)
        .collect()
}

fn arcana_idx(arcana: Option<Arcana>) -> String {
    match arcana {
        Some(a) => {
            use Arcana::*;
            let i = match a {
                Fool => 0,
                Magician => 1,
                Priestess => 2,
                Empress => 3,
                Emperor => 4,
                Hierophant => 5,
                Lovers => 6,
                Chariot => 7,
                Justice => 8,
                Hermit => 9,
                Fortune => 10,
                Strength => 11,
                Hanged => 12,
                Death => 13,
                Temperance => 14,
                Devil => 15,
                Tower => 16,
                Star => 17,
                Moon => 18,
                Sun => 19,
            };
            i.to_string()
        }
        None => String::new(),
    }
}

fn arcana_de(texto: &str) -> Option<Arcana> {
    use Arcana::*;
    let a = match texto.parse::<u8>().ok()? {
        0 => Fool,
        1 => Magician,
        2 => Priestess,
        3 => Empress,
        4 => Emperor,
        5 => Hierophant,
        6 => Lovers,
        7 => Chariot,
        8 => Justice,
        9 => Hermit,
        10 => Fortune,
        11 => Strength,
        12 => Hanged,
        13 => Death,
        14 => Temperance,
        15 => Devil,
        16 => Tower,
        17 => Star,
        18 => Moon,
        19 => Sun,
        _ => return None,
    };
    Some(a)
}

fn elemento_idx(e: Elemento) -> u8 {
    match e {
        Elemento::Fisico => 0,
        Elemento::Fuego => 1,
        Elemento::Hielo => 2,
        Elemento::Viento => 3,
        Elemento::Electrico => 4,
    }
}

fn elemento_de(idx: u8) -> Option<Elemento> {
    match idx {
        0 => Some(Elemento::Fisico),
        1 => Some(Elemento::Fuego),
        2 => Some(Elemento::Hielo),
        3 => Some(Elemento::Viento),
        4 => Some(Elemento::Electrico),
        _ => None,
    }
}

fn efecto_idx(e: Efecto) -> u8 {
    match e {
        Efecto::Danio => 0,
        Efecto::Curacion => 1,
        Efecto::BuffAtaque => 2,
        Efecto::BuffDefensa => 3,
        Efecto::DebuffAtaque => 4,
        Efecto::DebuffDefensa => 5,
    }
}

fn efecto_de(idx: u8) -> Option<Efecto> {
    match idx {
        0 => Some(Efecto::Danio),
        1 => Some(Efecto::Curacion),
        2 => Some(Efecto::BuffAtaque),
        3 => Some(Efecto::BuffDefensa),
        4 => Some(Efecto::DebuffAtaque),
        5 => Some(Efecto::DebuffDefensa),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::atomic::{AtomicU32, Ordering};

    use crate::data::{crear_jugador, roster_personas};

    fn ruta_unic() -> String {
        static CONTADOR: AtomicU32 = AtomicU32::new(0);
        let n = CONTADOR.fetch_add(1, Ordering::Relaxed);
        format!("/tmp/persona_rpg_save_test_{}.sav", n)
    }

    #[test]
    fn guardar_y_cargar_debe_preservar_el_estado() {
        let ruta = ruta_unic();
        let mut jugador = crear_jugador();
        jugador.nivel = 7;
        jugador.hp = 33;
        jugador.habilidades.truncate(3);
        let stock = roster_personas();
        guardar(&ruta, &jugador, &stock, 3).expect("guardar no debe fallar");
        let (j2, s2, piso) = cargar(&ruta).expect("cargar no debe fallar");
        assert_eq!(j2, jugador);
        assert_eq!(s2, stock);
        assert_eq!(piso, 3);
        let _ = fs::remove_file(&ruta);
    }

    #[test]
    fn cargar_con_stock_vacio_debe_devolver_lista_vacia() {
        let ruta = ruta_unic();
        guardar(&ruta, &crear_jugador(), &[], 1).expect("guardar");
        let (_, stock, _) = cargar(&ruta).expect("cargar");
        assert!(stock.is_empty());
        let _ = fs::remove_file(&ruta);
    }

    #[test]
    fn existe_debe_reflejar_el_archivo_en_disco() {
        let ruta = ruta_unic();
        assert!(!existe(&ruta));
        guardar(&ruta, &crear_jugador(), &[], 1).expect("guardar");
        assert!(existe(&ruta));
        let _ = fs::remove_file(&ruta);
    }
}
