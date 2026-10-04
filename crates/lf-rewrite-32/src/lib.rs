//! `lf-rewrite-32`: the 32-bit replacement DLL.
//!
//! README for future lanes:
//! - During the rewrite this DLL is injected into the running game and
//!   swaps accepted Rust rewrites in per function (see plan.md). Each
//!   replacement keeps a switch so regressions are bisected by switching
//!   halves off.
//! - Only the `i686-pc-windows-msvc` build of this crate ever ships or
//!   gets injected. It compiles on other targets so lints and layout
//!   checks still run everywhere, but those builds are never packaged.
//! - This crate is never a dependency of engine crates or the game binary.
//! - The per-function switch table lives here once the first replacement
//!   lands. Until then this crate exports only a version symbol.

/// Returns the replacement-set version (0 until the first switch lands).
#[must_use]
// JUSTIFICATION: DLL exports need `no_mangle`, which the 2024 edition
// treats as unsafe; the exported symbol is this crate's documented ABI,
// and the function body itself is safe code.
#[allow(unsafe_code)]
#[unsafe(no_mangle)]
pub extern "system" fn lf_rewrite_version() -> u32 {
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn version_starts_at_zero() {
        assert_eq!(super::lf_rewrite_version(), 0);
    }
}
