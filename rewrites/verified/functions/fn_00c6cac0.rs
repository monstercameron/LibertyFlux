// original: 0x00c6cac0 anim_link_pair (proposed)

/// Link two records through the global registry and run the pair query.
///
/// Either id -1 returns 0 at once. Otherwise the registry (read from
/// its global) pairs the ids; a null pair returns 0. The first id is
/// then resolved alone and the pair query runs on (single, pair), its
/// answer returned.
///
/// Original: cdecl with two stack words, three calls, reads one global.
lf_checker_rt::export!(cdecl, rw_00c6cac0(first: u32, second: u32) -> u32 {
    unsafe {
        const REGISTRY: u32 = 0x016D_D63C;
        const PAIR: u32 = 1;
        const SINGLE: u32 = 2;
        const QUERY: u32 = 3;
        const MISSING: u32 = 0xFFFF_FFFF;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        if first == MISSING || second == MISSING {
            return 0;
        }
        let reg = rd32(lf_checker_rt::relocated(REGISTRY));
        let pair: u32 = lf_checker_rt::callee_thiscall!(PAIR, u32, reg, first, second);
        if pair == 0 {
            return 0;
        }
        let single: u32 = lf_checker_rt::callee_thiscall!(SINGLE, u32, reg, first);
        lf_checker_rt::callee_cdecl!(QUERY, u32, single, pair)
    }
});
