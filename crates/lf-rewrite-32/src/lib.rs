//! `lf-rewrite-32`: the 32-bit replacement library's runtime and ABI.
//!
//! README for future lanes:
//! - During the rewrite the verified Rust rewrites are injected into the
//!   running game and swapped in per function (see plan.md). Each
//!   replacement keeps a switch so regressions are bisected by switching
//!   halves off (`lf_registry::bisect`).
//! - Only the `i686-pc-windows-msvc` build ever ships or gets injected. It
//!   compiles on other targets so lints and tests still run everywhere,
//!   but those builds are never packaged.
//! - This crate is never a dependency of engine crates or the game binary.
//!
//! # How the library is assembled
//!
//! The rewrites are not compiled here. `scripts/assemble/assemble.py`
//! generates, under `.artifacts/assembled/` (git-ignored, never
//! committed), a set of shard crates of a few hundred rewrites each, cut
//! by address range so no single crate is large, plus one top `cdylib`
//! crate (`lf-rewrites`, built as `lf_rewrites.dll`) that depends on the
//! shards and on this crate. Every shard lists its rewrites as
//! [`RewriteEntry`] rows; the top crate joins them with [`export_table!`].
//! The generator's manifest records every rewrite left out and why.
//!
//! The assembled library exports:
//! - `lf_rewrite_version() -> u32`: [`TABLE_ABI_VERSION`] (this crate).
//! - `lf_rewrite_init(exe_base: u32) -> u32`: points the runtime at the
//!   game's actual image base (this crate); call it once before any hook
//!   is enabled.
//! - `lf_rewrite_table(out_len: *mut u32) -> *const RewriteEntry`: the
//!   replacement table (generated top crate, via [`export_table!`]).
//!   `lf_registry::table::read_rows` and `lf_registry::table::plan` turn
//!   it into registry hooks, one control-file switch each, all off at
//!   start.
//!
//! This crate is an rlib only: its two exports reach the DLL through the
//! top crate (unmangled symbols of every linked crate are exported from a
//! `cdylib`). A library without `lf_rewrite_table` is treated by the loader
//! as an empty table.
//!
//! # The production runtime: what changes from the checker
//!
//! Rewrites are written against `lf-checker-rt`, the checker's runtime,
//! and compiled unchanged except for one mechanism:
//! - `relocated()`/`global()` convert a file address (image base
//!   `0x400000`) through `CHECKER_XBASE`. In the checker the worker sets it
//!   to its private mapping; in the game [`lf_rewrite_init`] sets it to the
//!   game's own base, so the same code reaches the game's real data.
//! - Callees. In the checker, `callee_cdecl!(id, ...)` and friends call
//!   through the worker's stub table, slot `id`, and the slot numbers are
//!   local to each function's contract. In the game slot `id` must be the
//!   original callee's real address. That mapping lives in the contracts,
//!   which are not tracked, so the generator takes it as an optional
//!   callee-map file (supplied by the coordinator) and rewrites each
//!   callee call to a per-function resolver in the generated copy only
//!   (the tracked rewrite never changes). A rewrite that uses callee slots
//!   the map does not cover, or a checker-only transport (the scripted
//!   XMM or TLS mirrors, or a callee the checker fed through a register
//!   transport), is still compiled and listed, but without
//!   [`FLAG_SWITCHABLE`]: the loader never hooks it.
//!
//! Proof limits carry over unchanged: a rewrite is exactly as trustworthy
//! in the game as its checker result says, including any narrowing that
//! result lists. The per-function switch is the safety net.

pub use lf_registry::table::{
    CONV_CDECL, CONV_FASTCALL, CONV_STDCALL, CONV_THISCALL, FLAG_CALLEES_MAPPED, FLAG_CHECKER_ONLY,
    FLAG_SWITCHABLE, FLAG_USES_CALLEES, PREFERRED_BASE, RewriteEntry, TABLE_ABI_VERSION,
};

/// Returns the replacement-table ABI version ([`TABLE_ABI_VERSION`]).
#[must_use]
// JUSTIFICATION: DLL exports need `no_mangle`, which the 2024 edition
// treats as unsafe; the exported symbol is this crate's documented ABI,
// and the function body itself is safe code.
#[allow(unsafe_code)]
#[unsafe(no_mangle)]
pub extern "system" fn lf_rewrite_version() -> u32 {
    TABLE_ABI_VERSION
}

/// Point the rewrites' address conversion at the game's image base.
///
/// `relocated(va)` then yields `va - 0x400000 + exe_base`. Call once, from
/// the loader's init thread, before any hook is enabled; no rewrite can
/// run before then, because every hook starts off. Returns
/// [`TABLE_ABI_VERSION`].
// JUSTIFICATION: exported like `lf_rewrite_version`; the one unsafe write
// stores into the runtime's base word, which is only read by rewrites,
// and no rewrite runs before the loader has called this (all hooks start
// off), so the write races with nothing.
#[allow(unsafe_code)]
#[unsafe(no_mangle)]
pub extern "system" fn lf_rewrite_init(exe_base: u32) -> u32 {
    // SAFETY: see the justification above; the static is a plain u32.
    unsafe { core::ptr::addr_of_mut!(lf_checker_rt::CHECKER_XBASE).write(exe_base) };
    TABLE_ABI_VERSION
}

/// Join shard tables, in the order given, into one boxed table.
#[must_use]
pub fn join_tables(shards: &[fn() -> Vec<RewriteEntry>]) -> Box<[RewriteEntry]> {
    let mut all = Vec::new();
    for shard in shards {
        all.extend(shard());
    }
    all.into_boxed_slice()
}

/// Store `len` through `out_len` unless it is null.
///
/// # Safety
///
/// A non-null `out_len` must be valid for one aligned `u32` write.
// JUSTIFICATION: the exported table function receives the length slot
// from the loader as a raw pointer; this is the one place it is written.
#[allow(unsafe_code)]
pub unsafe fn store_len(out_len: *mut u32, len: usize) {
    if !out_len.is_null() {
        // SAFETY: the caller guarantees a valid, aligned slot.
        unsafe { out_len.write(u32::try_from(len).unwrap_or(u32::MAX)) };
    }
}

/// Define the exported `lf_rewrite_table` from shard table functions.
///
/// Used once, by the generated top crate:
/// `lf_rewrite_32::export_table!(lf_rw_s000::entries, lf_rw_s001::entries);`
/// The table is built on first call and lives for the library's lifetime.
#[macro_export]
macro_rules! export_table {
    ($($shard:path),* $(,)?) => {
        static LF_TABLE: ::std::sync::OnceLock<::std::boxed::Box<[$crate::RewriteEntry]>> =
            ::std::sync::OnceLock::new();

        /// The replacement table: writes the row count through `out_len`
        /// (when non-null) and returns the first row.
        #[allow(unsafe_code)]
        #[unsafe(no_mangle)]
        pub extern "system" fn lf_rewrite_table(out_len: *mut u32) -> *const $crate::RewriteEntry {
            let table = LF_TABLE.get_or_init(|| $crate::join_tables(&[$($shard),*]));
            // SAFETY: the loader passes a valid length slot or null.
            unsafe { $crate::store_len(out_len, table.len()) };
            table.as_ptr()
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shard_a() -> Vec<RewriteEntry> {
        vec![RewriteEntry {
            address: 0x0040_1000,
            flags: FLAG_SWITCHABLE,
            ..RewriteEntry::default()
        }]
    }

    fn shard_b() -> Vec<RewriteEntry> {
        vec![
            RewriteEntry {
                address: 0x0050_0000,
                shard: 1,
                ..RewriteEntry::default()
            },
            RewriteEntry {
                address: 0x0050_0010,
                shard: 1,
                ..RewriteEntry::default()
            },
        ]
    }

    mod exported {
        crate::export_table!(super::shard_a, super::shard_b);
    }

    #[test]
    fn version_is_the_table_abi() {
        assert_eq!(lf_rewrite_version(), TABLE_ABI_VERSION);
        assert_eq!(TABLE_ABI_VERSION, 1);
    }

    #[test]
    fn init_sets_the_relocation_base() {
        assert_eq!(lf_rewrite_init(0x0110_0000), TABLE_ABI_VERSION);
        assert_eq!(lf_checker_rt::relocated(0x0040_1234), 0x0110_1234);
        assert_eq!(lf_rewrite_init(0x0040_0000), TABLE_ABI_VERSION);
        assert_eq!(lf_checker_rt::relocated(0x0040_1234), 0x0040_1234);
    }

    // The test reads the exported table back through its raw pointer, as
    // the loader does.
    #[allow(unsafe_code)]
    #[test]
    fn exported_table_joins_shards_in_order() {
        let mut len = 0u32;
        let first = exported::lf_rewrite_table(&raw mut len);
        assert_eq!(len, 3);
        // SAFETY: the table holds `len` rows for the library's lifetime.
        let rows = unsafe { std::slice::from_raw_parts(first, len as usize) };
        let addresses: Vec<u32> = rows.iter().map(|r| r.address).collect();
        assert_eq!(addresses, vec![0x0040_1000, 0x0050_0000, 0x0050_0010]);
        // A null length slot is accepted, and the table is built once.
        assert_eq!(exported::lf_rewrite_table(std::ptr::null_mut()), first);
    }
}
