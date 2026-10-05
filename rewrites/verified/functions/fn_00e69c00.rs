// original: 0x00e69c00 TRACKID (symbols)

/// Resolves one vehicle asset identifier and stores it globally.
///
/// Calls the name lookup (a two-word cdecl callee) with the name
/// pointer `NAME` and a zero flags word, then stores the returned id
/// at the global `OUT`. Takes no arguments, returns nothing (cdecl/0).
/// The lookup result is opaque: any 32-bit value is stored as is.
///
/// Original: 0x00E69C00 (cdecl, no arguments, one call).
lf_checker_rt::export!(cdecl, rw_00e69c00() -> () {
    unsafe {
        /// Asset name pointer (file VA).
        const NAME: u32 = 0x00EB2434;
        /// Resolved-id destination global (file VA).
        const OUT: u32 = 0x0167CCA8;
        /// Name-lookup callee id.
        const LOOKUP: u32 = 1;
        let id: u32 = lf_checker_rt::callee_cdecl!(LOOKUP, u32, lf_checker_rt::relocated(NAME), 0);
        lf_checker_rt::global::<u32>(OUT).write_unaligned(id);
    }
});
