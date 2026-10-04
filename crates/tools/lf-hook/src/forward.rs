//! Pure generation of the winmm proxy's export-forwarding table.
//!
//! The proxy (`lf-proxy`) ships as `winmm.dll` beside the game and must
//! export every symbol the real system `winmm.dll` exports, so a program
//! that loads it finds what it expects. For each real export this produces,
//! in ordinal order:
//! - a stub `jmp dword ptr [_LF_TARGETS + i*4]` that jumps to the address
//!   the loader resolved for export `i` against the system winmm;
//! - a linker `-export:` directive (`.drectve` section) that publishes the
//!   stub under the real export's name and ordinal (`,NONAME` for an
//!   ordinal-only export);
//! - its slot in `FORWARD_NAMES` / `FORWARD_ORDINALS`, which the loader
//!   walks to fill `_LF_TARGETS` with the system addresses, in this order.
//!
//! The index `i` is shared by the stub's `_LF_TARGETS` offset and the table
//! slot, so the stub for export `i` always jumps to the address resolved
//! for `FORWARD_NAMES[i]`. That is the correctness property these functions
//! and lf-hook's tests pin down.
//!
//! Dependency-free (plain data in, strings out) so `lf-proxy`'s build
//! script can `#[path]`-include this file, exactly as it includes `pe.rs`,
//! and the logic is unit-tested on the host through lf-hook.

/// One export to forward: its name (`None` for an ordinal-only export) and
/// its ordinal.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Forward {
    /// Export name, or `None` when the export has no name.
    pub name: Option<String>,
    /// Export ordinal.
    pub ordinal: u32,
}

/// Map any byte to one the assembler accepts in a symbol (`[A-Za-z0-9_]`).
#[must_use]
pub fn sanitize(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

/// Stub symbol for one export: `_lf_stub_<name>` (sanitized) or, for an
/// ordinal-only export, `_lf_stub_ord_<ordinal>`.
#[must_use]
pub fn stub_symbol(export: &Forward) -> String {
    match &export.name {
        Some(name) => format!("_lf_stub_{}", sanitize(name)),
        None => format!("_lf_stub_ord_{}", export.ordinal),
    }
}

/// The generated forwarding table, ready to write to the build's `OUT_DIR`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Plan {
    /// Number of forwarded exports.
    pub count: usize,
    /// `stubs.inc`: the `global_asm!` stubs and the `.drectve` directives.
    pub asm: String,
    /// `forward_table.rs`: `FORWARD_COUNT`, `FORWARD_NAMES`, `FORWARD_ORDINALS`.
    pub forward_table: String,
}

/// Build the forwarding table for `exports` (any order; sorted by ordinal
/// here, so the table and the stubs share one index).
#[must_use]
pub fn plan(exports: &[Forward]) -> Plan {
    use std::fmt::Write as _;
    let mut exports = exports.to_vec();
    exports.sort_by_key(|e| e.ordinal);

    let mut asm = String::new();
    let mut drectve = String::from(".section .drectve\n");
    let mut names = String::from(
        "/// Forwarded export names, ordinal order (`None` = ordinal-only).\n\
         pub const FORWARD_NAMES: [Option<&str>; FORWARD_COUNT] = [\n",
    );
    let mut ords = String::from(
        "/// Forwarded export ordinals, same order as `FORWARD_NAMES`.\n\
         pub const FORWARD_ORDINALS: [u32; FORWARD_COUNT] = [\n",
    );

    for (i, export) in exports.iter().enumerate() {
        let stub = stub_symbol(export);
        if let Some(name) = &export.name {
            writeln!(
                drectve,
                "    .ascii \" -export:{name}={stub},@{}\"",
                export.ordinal
            )
            .unwrap();
            writeln!(names, "    Some({name:?}),").unwrap();
        } else {
            writeln!(
                drectve,
                "    .ascii \" -export:{stub},@{},NONAME\"",
                export.ordinal
            )
            .unwrap();
            names.push_str("    None,\n");
        }
        writeln!(ords, "    {},", export.ordinal).unwrap();
        writeln!(
            asm,
            ".globl {stub}\n{stub}:\njmp dword ptr [_LF_TARGETS + {}]",
            i * 4
        )
        .unwrap();
    }
    names.push_str("];\n");
    ords.push_str("];\n");
    asm.push_str(&drectve);

    let forward_table = format!(
        "/// Number of forwarded system exports.\n\
         pub const FORWARD_COUNT: usize = {};\n{names}{ords}",
        exports.len()
    );
    Plan {
        count: exports.len(),
        asm,
        forward_table,
    }
}

/// The `forward_table.rs` an empty (non-x86) target gets: the loader code
/// is identical everywhere, so the arrays exist but are length zero.
#[must_use]
pub fn empty_forward_table() -> String {
    "/// Forwarded exports on this target (always 0 off x86).\n\
     pub const FORWARD_COUNT: usize = 0;\n\
     /// Export names (empty off x86).\n\
     pub const FORWARD_NAMES: [Option<&str>; 0] = [];\n\
     /// Export ordinals (empty off x86).\n\
     pub const FORWARD_ORDINALS: [u32; 0] = [];\n"
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn named(name: &str, ordinal: u32) -> Forward {
        Forward {
            name: Some(name.to_string()),
            ordinal,
        }
    }

    #[test]
    fn sanitize_keeps_only_symbol_characters() {
        assert_eq!(sanitize("timeGetTime"), "timeGetTime");
        assert_eq!(sanitize("mmio@Open.W"), "mmio_Open_W");
        assert_eq!(
            stub_symbol(&named("waveOutOpen", 13)),
            "_lf_stub_waveOutOpen"
        );
        assert_eq!(
            stub_symbol(&Forward {
                name: None,
                ordinal: 12
            }),
            "_lf_stub_ord_12"
        );
    }

    #[test]
    fn exports_are_sorted_by_ordinal() {
        let p = plan(&[named("c", 12), named("a", 2), named("b", 7)]);
        assert_eq!(p.count, 3);
        let order: Vec<&str> = p
            .asm
            .lines()
            .filter(|l| l.starts_with("_lf_stub_"))
            .map(|l| &l[9..l.len() - 1])
            .collect();
        assert_eq!(order, ["a", "b", "c"]);
        // FORWARD_ORDINALS follows the same order.
        let listed: Vec<&str> = p
            .forward_table
            .lines()
            .skip_while(|l| !l.contains("FORWARD_ORDINALS"))
            .filter(|l| l.trim_end().ends_with(','))
            .map(|l| l.trim().trim_end_matches(','))
            .collect();
        assert_eq!(listed, ["2", "7", "12"]);
    }

    #[test]
    fn each_stub_targets_its_own_table_slot() {
        // The property the loader relies on: stub i jumps to _LF_TARGETS+i*4,
        // and FORWARD_NAMES[i] is the export that slot resolves.
        let exports = [
            named("beta", 5),
            named("alpha", 3),
            Forward {
                name: None,
                ordinal: 9,
            },
        ];
        let p = plan(&exports);
        // Sorted: alpha(3) i=0, beta(5) i=1, ord-9 i=2.
        // Each export emits three lines: `.globl X`, `X:`, `jmp ... [_LF_TARGETS + off]`.
        let symbols: Vec<String> = p
            .asm
            .lines()
            .filter(|l| l.starts_with(".globl "))
            .map(|l| l[7..].to_string())
            .collect();
        let offsets: Vec<usize> = p
            .asm
            .lines()
            .filter_map(|l| l.rsplit_once("+ "))
            .map(|(_, rest)| rest.trim_end_matches(']').trim().parse().unwrap())
            .collect();
        let stubs: Vec<(String, usize)> = symbols.into_iter().zip(offsets).collect();
        assert_eq!(
            stubs,
            vec![
                ("_lf_stub_alpha".to_string(), 0),
                ("_lf_stub_beta".to_string(), 4),
                ("_lf_stub_ord_9".to_string(), 8),
            ]
        );
    }

    #[test]
    fn directives_publish_name_stub_and_ordinal() {
        let p = plan(&[
            named("timeGetTime", 10),
            Forward {
                name: None,
                ordinal: 12,
            },
        ]);
        assert!(p.asm.contains(".section .drectve"));
        assert!(
            p.asm
                .contains(r#".ascii " -export:timeGetTime=_lf_stub_timeGetTime,@10""#)
        );
        // Ordinal-only exports are forwarded by ordinal with NONAME.
        assert!(
            p.asm
                .contains(r#".ascii " -export:_lf_stub_ord_12,@12,NONAME""#)
        );
        assert!(
            p.forward_table
                .contains("pub const FORWARD_COUNT: usize = 2;")
        );
        assert!(p.forward_table.contains(r#"    Some("timeGetTime"),"#));
        assert!(p.forward_table.contains("    None,"));
    }

    #[test]
    fn a_name_needing_sanitizing_still_exports_under_its_real_name() {
        // The published name is the real export name; only the stub symbol is sanitized.
        let p = plan(&[named("odd.name", 4)]);
        assert!(
            p.asm
                .contains(r#".ascii " -export:odd.name=_lf_stub_odd_name,@4""#)
        );
        assert!(p.forward_table.contains(r#"    Some("odd.name"),"#));
    }

    #[test]
    fn empty_plan_and_empty_table() {
        let p = plan(&[]);
        assert_eq!(p.count, 0);
        assert!(p.asm.starts_with(".section .drectve"));
        assert!(p.forward_table.contains("FORWARD_COUNT: usize = 0;"));
        assert!(empty_forward_table().contains("[Option<&str>; 0] = [];"));
    }
}
