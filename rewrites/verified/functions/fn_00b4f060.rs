// original: 0x00b4f060 get_model_or_49 (proposed)

/// Look up the sub-model for a ped's key, defaulting to 49.
///
/// The key at `this + 0x124` is offered to the primary table (callee 1,
/// cdecl); when that misses (null) the fallback table (callee 2, cdecl) is
/// tried. The word at `+0x04` of the hit is returned unless it is -1, in
/// which case 49 is returned.
///
/// Original: 0x00b4f060 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00b4f060(this: u32) -> u32 {
    unsafe {
        const KEY: u32 = 0x124;
        const SUB: u32 = 0x04;
        const DEFAULT_MODEL: u32 = 0x31;
        const NO_SUB: u32 = 0xffff_ffff;
        const PRIMARY: u32 = 1;
        const FALLBACK: u32 = 2;
        let key = ((this + KEY) as *const u32).read_unaligned();
        let mut hit = lf_checker_rt::callee_cdecl!(PRIMARY, u32, key);
        if hit == 0 {
            hit = lf_checker_rt::callee_cdecl!(FALLBACK, u32, key);
        }
        let sub = ((hit + SUB) as *const u32).read_unaligned();
        if sub == NO_SUB {
            DEFAULT_MODEL
        } else {
            sub
        }
    }
});
