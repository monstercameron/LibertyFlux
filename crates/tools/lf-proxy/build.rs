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

// The forwarding-table generation is pure logic, shared with lf-hook's unit
// tests (synthetic exports) so it can be checked on the host. Included the
// same way as pe.rs; its own `#[cfg(test)]` tests are inert here (a build
// script is compiled without `--test`).
#[allow(dead_code)]
#[path = "../lf-hook/src/forward.rs"]
mod forward;

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

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=LF_WINMM_PATH");
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let target = env::var("TARGET").unwrap_or_default();

    // Stubs are 32-bit; other targets get empty files so host `cargo build`
    // and `cargo test` keep working (the proxy only ships as i686).
    if target != "i686-pc-windows-msvc" {
        std::fs::write(out.join("stubs.inc"), "").unwrap();
        std::fs::write(out.join("forward_table.rs"), forward::empty_forward_table()).unwrap();
        return;
    }

    let dll = find_system_winmm().expect("no system winmm.dll found (set LF_WINMM_PATH?)");
    let data = std::fs::read(&dll).expect("read winmm.dll");
    let exports = pe::file_exports(&data).expect("parse winmm export table");
    assert!(!exports.is_empty(), "winmm has no exports?");
    // Pure, host-tested generation (see lf-hook's `forward` module).
    let forwards: Vec<forward::Forward> = exports
        .into_iter()
        .map(|e| forward::Forward {
            name: e.name,
            ordinal: e.ordinal,
        })
        .collect();
    let plan = forward::plan(&forwards);

    std::fs::write(out.join("stubs.inc"), &plan.asm).unwrap();
    std::fs::write(out.join("forward_table.rs"), &plan.forward_table).unwrap();

    println!(
        "cargo:warning=proxying {} winmm exports from {}",
        plan.count,
        dll.display()
    );
}
