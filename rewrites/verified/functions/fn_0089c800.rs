// original: 0x0089c800 audio_tag_lookup (proposed)

/// Two-stage tag lookup returning a 16-bit tag, or 0xFFFF when absent.
///
/// Hashes each input word (callee 1, cdecl/2: the first call hashes `a0`,
/// the second call hashes the second stack word `a1`), then searches the
/// table described by the globals `COUNT_GLOB`/`BASE_GLOB` (callee 2, cdecl/3,
/// taking the first hash, the base and the count) and scans the hit (callee 3,
/// cdecl/2, taking the hit pointer and the second hash). Either stage
/// returning null yields `MISSING` (0xFFFF); otherwise the tag helper
/// (callee 4, cdecl/1) runs on the found record's first word and its low 16
/// bits are returned (`(an instruction of the original)`).
///
/// Original: 0x0089c800 (cdecl, two stack words).
lf_checker_rt::export!(cdecl, rw_0089c800(a0: u32, a1: u32) -> u32 {
    unsafe {
        const COUNT_GLOB: u32 = 0x0115f82c;
        const BASE_GLOB: u32 = 0x0115f830;
        const MISSING: u32 = 0xffff;
        const HASH: u32 = 1;
        const SEARCH: u32 = 2;
        const SCAN: u32 = 3;
        const TAG: u32 = 4;
        let h1: u32 = lf_checker_rt::callee_cdecl!(HASH, u32, a0, 0);
        let count = lf_checker_rt::global::<u32>(COUNT_GLOB).read_unaligned();
        let base = lf_checker_rt::global::<u32>(BASE_GLOB).read_unaligned();
        let hit: u32 = lf_checker_rt::callee_cdecl!(SEARCH, u32, h1, base, count);
        if hit == 0 {
            return MISSING;
        }
        let h2: u32 = lf_checker_rt::callee_cdecl!(HASH, u32, a1, 0);
        let found: u32 = lf_checker_rt::callee_cdecl!(SCAN, u32, hit, h2);
        if found == 0 {
            return MISSING;
        }
        let word = (found as *const u32).read_unaligned();
        let tag: u32 = lf_checker_rt::callee_cdecl!(TAG, u32, word);
        tag & 0xffff
    }
});
