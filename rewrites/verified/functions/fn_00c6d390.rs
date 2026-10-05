// original: 0x00c6d390 anim_lookup_wrap (proposed)

/// Thin wrapper: run the single-key lookup against the global registry.
///
/// The registry pointer is read from its global each call and passed as
/// the object; the answer is returned unchanged.
///
/// Original: cdecl with one stack word, one thiscall, reads one global.
lf_checker_rt::export!(cdecl, rw_00c6d390(key: u32) -> u32 {
    unsafe {
        const REGISTRY: u32 = 0x016D_D63C;
        const LOOKUP: u32 = 1;

        let reg = unsafe { (lf_checker_rt::relocated(REGISTRY) as *const u32).read_unaligned() };
        lf_checker_rt::callee_thiscall!(LOOKUP, u32, reg, key)
    }
});
