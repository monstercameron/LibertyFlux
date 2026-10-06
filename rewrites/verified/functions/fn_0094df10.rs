// original: 0x0094DF10 release_dead_handles (proposed)

/// Release the dead handles among the 16 handle words at `this`.
///
/// For each of the 16 dwords: -1 means empty and is skipped, otherwise the
/// handle is probed through callee 1 (one stack word). A zero answer means
/// dead: the handle is released through callee 2 (one stack word) and the
/// word is set to -1. A non-zero answer keeps it. Both answer tests are
/// full-word zero tests. No return channel is compared.
///
/// Original: 0x0094DF10 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_0094DF10(this: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 16;
        const EMPTY: u32 = 0xFFFF_FFFF;
        const PROBE: u32 = 1;
        const RELEASE: u32 = 2;
        let mut i = 0u32;
        while i < COUNT {
            let p = this.wrapping_add(i.wrapping_mul(4));
            let h = (p as *const u32).read_unaligned();
            if h != EMPTY {
                let r = lf_checker_rt::callee_cdecl!(PROBE, u32, h);
                if r == 0 {
                    lf_checker_rt::callee_cdecl!(RELEASE, u32, h);
                    (p as *mut u32).write_unaligned(EMPTY);
                }
            }
            i += 1;
        }
        0
    }
});
