// original: 0x00E667D0 BASE (proposed)

/// Resolve the hashed id of one audio label and cache it in its global slot.
///
/// Calls the name-hash callee with the label string pointer and a zero flag,
/// then stores the returned id at the label's cache word. The pushed flag is
/// the second argument (cdecl, caller cleans 8 bytes). No arguments; the id
/// stays in eax after the store, so the function returns it (cdecl).
lf_checker_rt::export!(cdecl, rw_00e667d0() -> u32 {
    unsafe {
        const LABEL: u32 = 0x00E94A3C;
        const CACHE: u32 = 0x0129243C;
        const HASH_CALLEE: u32 = 1;
        let id: u32 = lf_checker_rt::callee_cdecl!(HASH_CALLEE, u32, lf_checker_rt::relocated(LABEL), 0);
        (lf_checker_rt::global::<u32>(CACHE) as *mut u32).write_unaligned(id);
        id
    }
});
