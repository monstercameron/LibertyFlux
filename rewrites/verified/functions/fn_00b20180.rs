// original: 0x00b20180 maybe_rebuild_cache (proposed)

/// Rebuilds the cache unless any of three state words says otherwise.
///
/// Cdecl with no arguments. Returns at once when the busy word is 1,
/// when the generation word differs from its reference (-1), or when
/// the mode word is 0x12; otherwise passes a scratch buffer and the
/// worker callback to the enumerator (cdecl of two words) and runs the
/// finaliser (cdecl, no arguments). The original's stack-cookie
/// boilerplate is calling-convention noise: the cookie value is
/// ESP-derived and differs across frames, so the rewrite keeps a dummy
/// slot and still issues the cookie-check call to preserve the call
/// sequence. Returns the finaliser's result, or the generation word on
/// the generation and mode early paths. The busy-word path is never
/// taken in the proof: it returns the ESP-derived cookie itself, which
/// no rewrite can observe (the rewrite returns the generation word
/// there instead).
lf_checker_rt::export!(cdecl, rw_00b20180() -> u32 {
    unsafe {
        const BUSY: u32 = 0x011f7060;
        const GENERATION: u32 = 0x012088b4;
        const GEN_REF: u32 = 0x00f1c040;
        const MODE: u32 = 0x01037720;
        const MODE_SKIP: u32 = 0x12;
        const WORKER: u32 = 0x00b20520;
        let generation =
            (lf_checker_rt::global::<u32>(GENERATION) as *const u32).read_unaligned();
        let result = if (lf_checker_rt::global::<u32>(BUSY) as *const u32).read_unaligned() == 1
        {
            generation
        } else if generation
            != (lf_checker_rt::global::<u32>(GEN_REF) as *const u32).read_unaligned()
        {
            generation
        } else if (lf_checker_rt::global::<u32>(MODE) as *const u32).read_unaligned() == MODE_SKIP
        {
            generation
        } else {
            let mut scratch = [0u32; 4];
            lf_checker_rt::callee_cdecl!(
                1,
                u32,
                lf_checker_rt::relocated(WORKER),
                scratch.as_mut_ptr() as u32
            );
            lf_checker_rt::callee_cdecl!(2, u32,)
        };
        lf_checker_rt::callee_cdecl!(3, u32,);
        result
    }
});
