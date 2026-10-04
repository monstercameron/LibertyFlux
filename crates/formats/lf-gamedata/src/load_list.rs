//! Load lists: `gta.dat`, `default.dat`, `images.txt` and friends.
//!
//! Grammar: `#` comments, whitespace-separated `KEY args...` directives.
//! Known keys (upper case in shipped files): `IDE`, `IPL`, `IMG`,
//! `IMGLIST`, `WATER`, `SPLINE`, `ANIMGRP`, `HANDLING`, `VEHICLEEXTRAS`,
//! `PLAYER`, `PEDGRP`, `CARGRP`, `RADIO`, `RADIOLOGOS`, `WEAPONINFO`,
//! `THROWNWEAPONINFO`, `PEDPERSONALITY`, `MELEEANIMS`, `ACTIONTABLE`,
//! `EXPLOSIONFX`, `VEHOFF`, `FMENUFILE`, `LBDATAFILE`, `LBICONSFILE`,
//! `MAP`, `INTERIOR`, plus `CULL`/`OCCL`-style map directives in `gta.dat`.
//! Unknown keys are kept, not rejected: episode and test files add more.
//!
//! `images.txt` and the `cdimages/*.txt` lists are bare paths with an
//! optional trailing `1` flag — the same shape with an empty key.

use crate::text::{CommentStyle, logical_lines, split_ws};
use crate::{Result, decode};

/// One load directive: keyword plus argument words.
#[derive(Debug, Clone)]
pub struct Directive {
    /// 1-based line number.
    pub line: usize,
    /// Keyword (`IDE`, `IMG`, ...); empty for bare-path lists.
    pub key: String,
    /// Argument words.
    pub args: Vec<String>,
}

/// Parse a load-list file from bytes.
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn parse_load_list(file: &str, bytes: &[u8]) -> Result<Vec<Directive>> {
    let text = decode(file, bytes)?;
    let mut out = Vec::new();
    for l in logical_lines(&text, CommentStyle::HASH, false) {
        let f = split_ws(&l.code);
        if f.is_empty() {
            continue;
        }
        // Bare-path lists have no keyword: a line with no all-caps first
        // word... in practice every directive key is ASCII uppercase, and
        // every bare path contains `/` or lowercase. Paths win the test.
        let looks_like_path = f[0].contains('/') || f[0].contains('\\');
        let (key, args) = if looks_like_path {
            (String::new(), f)
        } else {
            (f[0].clone(), f[1..].to_vec())
        };
        out.push(Directive {
            line: l.num,
            key,
            args,
        });
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_directives_and_bare_paths() {
        let src = "# c\r\nIMGLIST common:/data/images.txt\r\nIDE common:/DATA/gtxd.IDE\r\nplatformimg:/anim/cuts 1\r\n";
        let d = parse_load_list("gta.dat", src.as_bytes()).unwrap();
        assert_eq!(d.len(), 3);
        assert_eq!(d[0].key, "IMGLIST");
        assert_eq!(d[0].args, vec!["common:/data/images.txt"]);
        assert_eq!(d[2].key, "");
        assert_eq!(d[2].args.len(), 2);
    }
}
