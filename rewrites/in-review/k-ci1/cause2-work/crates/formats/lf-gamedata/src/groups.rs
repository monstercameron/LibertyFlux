//! Population groups: `cargrp.dat` and `pedgrp.dat`.
//!
//! Both files list the vehicle and ped models that spawn for each
//! `POPCYCLE_GROUP_*`. Grammar differs per file:
//!
//! * `cargrp.dat`: one group per line — comma-separated model names with
//!   the group name in a trailing `#` comment.
//! * `pedgrp.dat`: one model per line; groups are separated by blank lines
//!   and named by a preceding `# POPCYCLE_GROUP_*` comment line.

use crate::text::{CommentStyle, logical_lines, split_csv};
use crate::{Result, decode};

/// One named population group.
#[derive(Debug, Clone)]
pub struct PopGroup {
    /// Group name (`POPCYCLE_GROUP_*`), if the file states it.
    pub name: Option<String>,
    /// Model names in the group.
    pub models: Vec<String>,
}

/// Parse a `cargrp.dat` file from bytes.
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn parse_cargrp(file: &str, bytes: &[u8]) -> Result<Vec<PopGroup>> {
    let text = decode(file, bytes)?;
    let mut out = Vec::new();
    for l in logical_lines(&text, CommentStyle::HASH, true) {
        if l.code.is_empty() {
            continue;
        }
        let models: Vec<String> = split_csv(&l.code)
            .into_iter()
            .filter(|m| !m.is_empty())
            .collect();
        let name = l.comment.as_ref().and_then(|c| {
            // Trailing comment is the group name, sometimes with whitespace.
            let w: Vec<&str> = c.split_whitespace().collect();
            (!w.is_empty()).then(|| w[0].to_string())
        });
        out.push(PopGroup { name, models });
    }
    Ok(out)
}

/// Parse a `pedgrp.dat` file from bytes.
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn parse_pedgrp(file: &str, bytes: &[u8]) -> Result<Vec<PopGroup>> {
    let text = decode(file, bytes)?;
    let mut out: Vec<PopGroup> = Vec::new();
    let mut pending_name: Option<String> = None;
    for l in logical_lines(&text, CommentStyle::HASH, true) {
        if l.code.is_empty() {
            if let Some(c) = &l.comment {
                // A bare `# POPCYCLE_GROUP_X` line names the next group.
                if let Some(word) = c.split_whitespace().next()
                    && word.starts_with("POPCYCLE_GROUP_")
                {
                    pending_name = Some(word.to_string());
                }
            }
            continue;
        }
        let model = l.code.trim().to_string();
        if model.is_empty() {
            continue;
        }
        let start_new = pending_name.is_some() || out.is_empty();
        if start_new {
            out.push(PopGroup {
                name: pending_name.take(),
                models: Vec::new(),
            });
        }
        if let Some(g) = out.last_mut() {
            g.models.push(model);
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cargrp_reads_trailing_names() {
        let src = "buccaneer, voodoo, # POPCYCLE_GROUP_HARLEM\r\nfuto, hakumai, # POPCYCLE_GROUP_BRONX\r\n";
        let g = parse_cargrp("cargrp.dat", src.as_bytes()).unwrap();
        assert_eq!(g.len(), 2);
        assert_eq!(g[0].name.as_deref(), Some("POPCYCLE_GROUP_HARLEM"));
        assert_eq!(g[0].models, vec!["buccaneer", "voodoo"]);
    }

    #[test]
    fn pedgrp_groups_by_comment() {
        let src = "# POPCYCLE_GROUP_HARLEM\r\nF_M_PHarBron_01\r\nF_O_PHarBron_01\r\n# POPCYCLE_GROUP_BRONX\r\nM_Y_Bronx_01\r\n";
        let g = parse_pedgrp("pedgrp.dat", src.as_bytes()).unwrap();
        assert_eq!(g.len(), 2);
        assert_eq!(g[0].models.len(), 2);
        assert_eq!(g[1].name.as_deref(), Some("POPCYCLE_GROUP_BRONX"));
    }
}
