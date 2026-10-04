//! `pedpersonality.dat`: per-model ped attributes.
//!
//! Grammar: `#` comments, comma-separated rows, one header comment line
//! naming the columns, no sections. Every shipped row has exactly 20
//! fields:
//!
//! `model, sex, age, sexiness, bravery, attack, defend, agility,`
//! `melee_martial_level, lang1, lang2,` then 7 move/hit/firearm style
//! tokens, a move token, and `+`-led behaviour flags
//! (`+cancarryweapons+gardening`, ...). `-` means none for the style
//! slots. Attack and defend are floats (`0.9`, `1.1`, `1.5`).

use crate::text::{CommentStyle, field, is_some_token, logical_lines, parse_f32, parse_i32};
use crate::{Error, ErrorKind, Result, decode};

/// One ped personality row.
#[derive(Debug, Clone)]
pub struct PedPersonality {
    /// Ped model name.
    pub model: String,
    /// `M` or `F`.
    pub sex: String,
    /// Age in years.
    pub age: i32,
    /// Sexiness rating.
    pub sexiness: i32,
    /// Bravery rating.
    pub bravery: i32,
    /// Attack rating (float in file: `1`, `0.9`, `1.1`).
    pub attack: f32,
    /// Defend rating (float in file).
    pub defend: f32,
    /// Agility rating.
    pub agility: i32,
    /// Melee martial level percent.
    pub melee_martial_level: i32,
    /// First language code.
    pub lang1: String,
    /// Second language code.
    pub lang2: String,
    /// Seven style slots (core moves, hits, unarmed, counters, firearm,
    /// two spares); `-` parses as `None`.
    pub styles: [Option<String>; 7],
    /// Movement style token.
    pub move_style: String,
    /// `+`-separated behaviour flags (`cancarryweapons`, `onfoot`, `smoker`,
    /// `gardening`, ...); empty when the row has no flag field.
    pub flags: Vec<String>,
}

impl PedPersonality {
    /// True when the row carries the `cancarryweapons` flag.
    #[must_use]
    pub fn can_carry_weapons(&self) -> bool {
        self.flags.iter().any(|f| f == "cancarryweapons")
    }
}

/// Parse a `pedpersonality.dat` file from bytes.
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn parse_ped_personality(file: &str, bytes: &[u8]) -> Result<Vec<PedPersonality>> {
    let text = decode(file, bytes)?;
    let mut out = Vec::new();
    for l in logical_lines(&text, CommentStyle::HASH, false) {
        // Keep the trailing empty field: flag-less rows end with a comma
        // and the flag slot is still field 19 (empty).
        let f: Vec<String> = l.code.split(',').map(|s| s.trim().to_string()).collect();
        if f.len() != 20 {
            return Err(Error::new(
                file,
                l.num,
                ErrorKind::FieldCount {
                    expected: Some(20),
                    found: f.len(),
                },
            ));
        }
        let n = l.num;
        let g = |i: usize| field(file, n, &f, i, Some(20)).map(std::string::ToString::to_string);
        let style = |i: usize| {
            let s = field(file, n, &f, i, Some(20))?;
            Ok::<_, Error>(is_some_token(s).then(|| s.to_string()))
        };
        // Last field holds `+`-led flags or is empty.
        let flag_field = g(19)?;
        let flags: Vec<String> = flag_field
            .split('+')
            .filter(|s| !s.is_empty())
            .map(std::string::ToString::to_string)
            .collect();
        out.push(PedPersonality {
            model: g(0)?,
            sex: g(1)?,
            age: parse_i32(file, n, &f, 2)?,
            sexiness: parse_i32(file, n, &f, 3)?,
            bravery: parse_i32(file, n, &f, 4)?,
            attack: parse_f32(file, n, &f, 5)?,
            defend: parse_f32(file, n, &f, 6)?,
            agility: parse_i32(file, n, &f, 7)?,
            melee_martial_level: parse_i32(file, n, &f, 8)?,
            lang1: g(9)?,
            lang2: g(10)?,
            styles: [
                style(11)?,
                style(12)?,
                style(13)?,
                style(14)?,
                style(15)?,
                style(16)?,
                style(17)?,
            ],
            move_style: g(18)?,
            flags,
        });
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_row_with_flag() {
        let src = "# Modelname Sex Age\r\nplayer, M, 30, 3, 5, 1, 0.4, 5, 50, E, E, core_moves, hits_pool, unarmed_player, counters_player, firearm_core, -, -, move_melee, +cancarryweapons\r\n";
        let p = parse_ped_personality("pedpersonality.dat", src.as_bytes()).unwrap();
        assert_eq!(p.len(), 1);
        assert_eq!(p[0].model, "player");
        assert!(p[0].can_carry_weapons());
        assert_eq!(p[0].styles[5], None);
        assert_eq!(p[0].move_style, "move_melee");
    }

    #[test]
    fn parses_row_without_flag() {
        let src = "ig_X, F, 27, 3, 4, 1, 1, 5, 50, E, E, core_moves, hits_pool, unarmed_ped, counters_player, firearm_core, -, -, move_rubbish,\r\n";
        let p = parse_ped_personality("pedpersonality.dat", src.as_bytes()).unwrap();
        assert!(!p[0].can_carry_weapons());
        assert!(p[0].flags.is_empty());
    }
}
