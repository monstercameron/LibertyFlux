//! `popcycle.dat`: ped/vehicle density by zone and time.
//!
//! Grammar: `//` comments, whitespace-separated rows. One block per
//! `POPCYCLE_ZONE_*`, opened by a `// POPCYCLE_ZONE_*` comment line, with
//! 24 rows: 12 weekday slots then 12 weekend slots in 2-hour steps from
//! midnight. Each row starts with 6 density numbers (`#Peds`,
//! `#Scenario`, `#Cars`, `#ParkedCars`, `%CopCars`, `%CopPeds`) then one
//! weight per population group (42 in shipped files; the header comment
//! abbreviates the group list, so weights are positional).

use crate::text::{CommentStyle, logical_lines, parse_f32, parse_i32, split_ws};
use crate::{Error, ErrorKind, Result, decode};

/// How many slots per day-part block (12 two-hour slots).
pub const SLOTS_PER_DAYPART: usize = 12;

/// One density row.
#[derive(Debug, Clone)]
pub struct DensityRow {
    /// Max peds.
    pub peds: i32,
    /// Max scenario peds.
    pub scenario: i32,
    /// Max cars.
    pub cars: i32,
    /// Max parked cars.
    pub parked_cars: i32,
    /// Percent cop cars.
    pub cop_cars: i32,
    /// Percent cop peds.
    pub cop_peds: i32,
    /// Per-group weights (42 in shipped files).
    pub weights: Vec<f32>,
}

/// One zone: weekday and weekend rows.
#[derive(Debug, Clone)]
pub struct PopZone {
    /// Zone name.
    pub name: String,
    /// 12 weekday rows, midnight to 10PM in 2-hour steps.
    pub weekday: Vec<DensityRow>,
    /// 12 weekend rows.
    pub weekend: Vec<DensityRow>,
}

/// Parse a `popcycle.dat` file from bytes.
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn parse_popcycle(file: &str, bytes: &[u8]) -> Result<Vec<PopZone>> {
    let text = decode(file, bytes)?;
    let mut out: Vec<PopZone> = Vec::new();
    for l in logical_lines(&text, CommentStyle::SLASHES, true) {
        if l.code.is_empty() {
            if let Some(c) = &l.comment
                && let Some(name) = c.split_whitespace().next()
                && name.starts_with("POPCYCLE_ZONE_")
            {
                out.push(PopZone {
                    name: name.to_string(),
                    weekday: Vec::new(),
                    weekend: Vec::new(),
                });
            }
            continue;
        }
        let zone = out.last_mut().ok_or_else(|| {
            Error::new(
                file,
                l.num,
                ErrorKind::UnknownMarker {
                    marker: "row before first zone".to_string(),
                },
            )
        })?;
        let f = split_ws(&l.code);
        if f.len() < 7 {
            return Err(Error::new(
                file,
                l.num,
                ErrorKind::FieldCount {
                    expected: None,
                    found: f.len(),
                },
            ));
        }
        let n = l.num;
        let mut weights = Vec::with_capacity(f.len() - 6);
        for i in 6..f.len() {
            weights.push(parse_f32(file, n, &f, i)?);
        }
        let row = DensityRow {
            peds: parse_i32(file, n, &f, 0)?,
            scenario: parse_i32(file, n, &f, 1)?,
            cars: parse_i32(file, n, &f, 2)?,
            parked_cars: parse_i32(file, n, &f, 3)?,
            cop_cars: parse_i32(file, n, &f, 4)?,
            cop_peds: parse_i32(file, n, &f, 5)?,
            weights,
        };
        if zone.weekday.len() < SLOTS_PER_DAYPART {
            zone.weekday.push(row);
        } else {
            zone.weekend.push(row);
        }
    }
    for z in &out {
        if z.weekday.len() != SLOTS_PER_DAYPART || z.weekend.len() != SLOTS_PER_DAYPART {
            return Err(Error::whole_file(
                file,
                ErrorKind::BadHeader {
                    want: "24 rows per zone",
                },
            ));
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_one_zone() {
        let mut src = String::from("// POPCYCLE_ZONE_ACTER\r\n// Weekday\r\n");
        for _ in 0..24 {
            src.push_str("15 15 12 25 3 0 0 0 75 25 0 0\r\n");
        }
        let z = parse_popcycle("popcycle.dat", src.as_bytes()).unwrap();
        assert_eq!(z.len(), 1);
        assert_eq!(z[0].weekday.len(), 12);
        assert_eq!(z[0].weekend.len(), 12);
        assert_eq!(z[0].weekday[0].weights.len(), 6);
    }

    #[test]
    fn short_zone_is_error() {
        let src = "// POPCYCLE_ZONE_X\r\n1 2 3 4 5 6 7\r\n";
        let e = parse_popcycle("popcycle.dat", src.as_bytes()).unwrap_err();
        assert!(matches!(e.kind, ErrorKind::BadHeader { .. }));
    }
}
