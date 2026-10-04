//! Proxy build script: generate winmm forwarding from the real DLL.
//!
//! Reads the export table of the system's 32-bit winmm.dll and generates:
//! * `stubs.inc` — one `jmp dword ptr [target]` stub per export plus a
//!   `.drectve` section with one `-export:` directive per export, included
//!   via `global_asm!` (nothing hand-written, nothing to drift). The
//!   directives are the linker's native export mechanism (the same one the
//!   C compiler emits); a second `/DEF` file or command-line `/EXPORT`
//!   cannot resolve Rust-built object symbols and must not be used.
//! * `forward_table.rs` — names/ordinals for run-time resolution.
//!
//! Resolution at run time uses the absolute system path, never a bare name,
//! so the proxy cannot load itself in a loop.

#[allow(dead_code)]
#[path = "../lf-hook/src/pe.rs"]
mod pe;

use std::env;
use std::path::PathBuf;

fn find_system_winmm() -> Option<PathBuf> {
    if let Ok(p) = env::var("LF_WINMM_PATH") {
        let p = PathBuf::from(p);
        if p.is_file() {
            return Some(p);
        }
    }
    let sysroot = env::var("SystemRoot").unwrap_or_else(|_| r"C:\Windows".to_string());
    for sub in ["SysWOW64", "System32"] {
        let p = PathBuf::from(&sysroot).join(sub).join("winmm.dll");
        if p.is_file() {
            return Some(p);
        }
    }
    None
}

fn sanitize(name: &str) -> String {
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

fn main() {
    use std::fmt::Write as _;
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=LF_WINMM_PATH");
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let target = env::var("TARGET").unwrap_or_default();

    // Stubs are 32-bit; other targets get empty files so host `cargo build`
    // and `cargo test` keep working (the proxy only ships as i686).
    if target != "i686-pc-windows-msvc" {
        std::fs::write(out.join("stubs.inc"), "").unwrap();
        std::fs::write(
            out.join("forward_table.rs"),
            "/// Forwarded exports on this target (always 0 off x86).\n\
             pub const FORWARD_COUNT: usize = 0;\n\
             /// Export names (empty off x86).\n\
             pub const FORWARD_NAMES: [Option<&str>; 0] = [];\n\
             /// Export ordinals (empty off x86).\n\
             pub const FORWARD_ORDINALS: [u32; 0] = [];\n",
        )
        .unwrap();
        return;
    }

    let dll = find_system_winmm().expect("no system winmm.dll found (set LF_WINMM_PATH?)");
    let data = std::fs::read(&dll).expect("read winmm.dll");
    let mut exports = pe::file_exports(&data).expect("parse winmm export table");
    assert!(!exports.is_empty(), "winmm has no exports?");
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

    for (i, e) in exports.iter().enumerate() {
        let stub = match &e.name {
            Some(n) => format!("_lf_stub_{}", sanitize(n)),
            None => format!("_lf_stub_ord_{}", e.ordinal),
        };
        if let Some(n) = &e.name {
            writeln!(drectve, "    .ascii \" -export:{n}={stub},@{}\"", e.ordinal).unwrap();
            writeln!(names, "    Some({n:?}),").unwrap();
        } else {
            writeln!(
                drectve,
                "    .ascii \" -export:{stub},@{0},NONAME\"",
                e.ordinal
            )
            .unwrap();
            names.push_str("    None,\n");
        }
        writeln!(ords, "    {},", e.ordinal).unwrap();
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

    std::fs::write(out.join("stubs.inc"), &asm).unwrap();
    std::fs::write(
        out.join("forward_table.rs"),
        format!(
            "/// Number of forwarded system exports.\n\
             pub const FORWARD_COUNT: usize = {};\n{names}{ords}",
            exports.len()
        ),
    )
    .unwrap();

    println!(
        "cargo:warning=proxying {} winmm exports from {}",
        exports.len(),
        dll.display()
    );
}
