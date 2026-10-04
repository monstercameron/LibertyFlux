//! Build script of the checker worker: link the 32-bit worker away from the
//! conventional executable base.
//!
//! The worker holds the original's preferred-base window (from 0x400000) so
//! that unrelocated absolute accesses by the original fault, or read the
//! optional read-only shadow, instead of reading worker memory. An
//! executable linked at the default base would sit inside that window (and
//! with address randomisation, somewhere near it), so the i686 worker is
//! linked at a fixed high base instead. Other targets are unaffected.

/// Base address of the worker executable on the i686 Windows target.
const WORKER_BASE: &str = "0x60000000";

fn main() {
    if std::env::var("TARGET").as_deref() == Ok("i686-pc-windows-msvc") {
        println!("cargo:rustc-link-arg-bins=/BASE:{WORKER_BASE}");
        println!("cargo:rustc-link-arg-bins=/DYNAMICBASE:NO");
    }
    println!("cargo:rerun-if-changed=build.rs");
}
