//! `water.dat`: water surface quads.
//!
//! Grammar: no comments, no header, whitespace-separated rows. Every
//! shipped row carries 30 values: four vertices of `(x, y, z, nx, ny,
//! nz, foam)` then a water-type id and a second flag value. The
//! per-vertex seventh value and the trailing flag are undocumented
//! (kept as `foam` and `flag` by role guess from values: the seventh
//! value reads like a foam/shore blend 0.0-1.0).

use crate::text::{CommentStyle, logical_lines, parse_f32, parse_i32, split_ws};
use crate::{Error, ErrorKind, Result, decode};

/// One water vertex: position, normal, foam blend.
#[derive(Debug, Clone, Default)]
pub struct WaterVertex {
    /// Position in world units.
    pub pos: [f32; 3],
    /// Normal.
    pub normal: [f32; 3],
    /// Seventh value, 0.0-1.0 in shipped rows (role unknown).
    pub foam: f32,
}

/// One water quad: four vertices plus type and flag.
#[derive(Debug, Clone)]
pub struct WaterQuad {
    /// Four corner vertices.
    pub verts: [WaterVertex; 4],
    /// Water type id (5 = sea, 13 = pools in shipped rows).
    pub kind: i32,
    /// Trailing flag value (0.0 or 1.0 in shipped rows; unknown meaning).
    pub flag: f32,
}

/// Parse a `water.dat` file from bytes.
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn parse_water(file: &str, bytes: &[u8]) -> Result<Vec<WaterQuad>> {
    let text = decode(file, bytes)?;
    let mut out = Vec::new();
    for l in logical_lines(&text, CommentStyle::NONE, false) {
        let f = split_ws(&l.code);
        if f.len() != 30 {
            return Err(Error::new(
                file,
                l.num,
                ErrorKind::FieldCount {
                    expected: Some(30),
                    found: f.len(),
                },
            ));
        }
        let n = l.num;
        let mut verts: [WaterVertex; 4] = [
            WaterVertex::default(),
            WaterVertex::default(),
            WaterVertex::default(),
            WaterVertex::default(),
        ];
        for (v, slot) in verts.iter_mut().enumerate() {
            let b = v * 7;
            *slot = WaterVertex {
                pos: [
                    parse_f32(file, n, &f, b)?,
                    parse_f32(file, n, &f, b + 1)?,
                    parse_f32(file, n, &f, b + 2)?,
                ],
                normal: [
                    parse_f32(file, n, &f, b + 3)?,
                    parse_f32(file, n, &f, b + 4)?,
                    parse_f32(file, n, &f, b + 5)?,
                ],
                foam: parse_f32(file, n, &f, b + 6)?,
            };
        }
        out.push(WaterQuad {
            verts,
            kind: parse_i32(file, n, &f, 28)?,
            flag: parse_f32(file, n, &f, 29)?,
        });
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_quad() {
        let mut v: Vec<&str> = Vec::new();
        for _ in 0..4 {
            v.extend(["1.0", "2.0", "3.0", "0.0", "0.0", "1.0", "0.5"]);
        }
        v.extend(["5", "1.0"]);
        let q = parse_water("water.dat", v.join(" ").as_bytes()).unwrap();
        assert_eq!(q.len(), 1);
        assert_eq!(q[0].kind, 5);
        assert_eq!(q[0].verts[3].foam, 0.5);
    }

    #[test]
    fn bad_width_errors() {
        let e = parse_water("water.dat", b"1.0 2.0").unwrap_err();
        assert!(matches!(e.kind, ErrorKind::FieldCount { .. }));
    }
}
