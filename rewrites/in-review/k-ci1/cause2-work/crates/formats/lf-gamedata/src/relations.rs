//! Relationship tables: `ped.dat` and `Relationships.dat`.
//!
//! Grammar: `#` comments, whitespace-separated. A bare group name opens a
//! group; following `Hate`/`Dislike`/`Like`/`Respect` lines list the
//! target groups. Both files share the shape; `Relationships.dat` uses
//! `GANG_*` names where `ped.dat` uses `GANG1..10`.

use crate::text::{CommentStyle, logical_lines, split_ws};
use crate::{Error, ErrorKind, Result, decode};

/// One acquaintance level plus its target groups.
#[derive(Debug, Clone)]
pub struct Relation {
    /// Level: `Hate`, `Dislike`, `Like` or `Respect`.
    pub level: String,
    /// Target group names.
    pub targets: Vec<String>,
}

/// One group: name plus relations.
#[derive(Debug, Clone)]
pub struct RelationGroup {
    /// Group name.
    pub name: String,
    /// Relations in file order.
    pub relations: Vec<Relation>,
}

/// Parse a relationship file from bytes.
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn parse_relations(file: &str, bytes: &[u8]) -> Result<Vec<RelationGroup>> {
    let text = decode(file, bytes)?;
    let mut out: Vec<RelationGroup> = Vec::new();
    for l in logical_lines(&text, CommentStyle::HASH, false) {
        let f = split_ws(&l.code);
        if f.is_empty() {
            continue;
        }
        if f.len() == 1 {
            out.push(RelationGroup {
                name: f[0].clone(),
                relations: Vec::new(),
            });
            continue;
        }
        let group = out.last_mut().ok_or_else(|| {
            Error::new(
                file,
                l.num,
                ErrorKind::UnknownMarker {
                    marker: "relation before first group".to_string(),
                },
            )
        })?;
        group.relations.push(Relation {
            level: f[0].clone(),
            targets: f[1..].to_vec(),
        });
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_groups() {
        let src = "# Acquaintance\r\nCOP\r\nHate CRIMINAL DEALER\r\nRespect MEDIC FIREMAN COP\r\n";
        let g = parse_relations("ped.dat", src.as_bytes()).unwrap();
        assert_eq!(g.len(), 1);
        assert_eq!(g[0].relations.len(), 2);
        assert_eq!(g[0].relations[0].targets.len(), 2);
    }
}
