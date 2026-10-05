// original: 0x00c6ca30 anim_find_call (proposed)

/// Invoke the handler of record `id` with sub-key `sub`, if it exists.
///
/// Missing (-1) ids and failed lookups return 0 without further calls;
/// otherwise the record is looked up a second time and
/// `METHOD(record, sub)` runs, its answer returned.
///
/// Original: cdecl with two stack words, three call sites.
lf_checker_rt::export!(cdecl, rw_00c6ca30(id: u32, sub: u32) -> u32 {
    unsafe {
        const LOOKUP: u32 = 1;
        const METHOD: u32 = 2;
        const MISSING: u32 = 0xFFFF_FFFF;

        if id == MISSING {
            return 0;
        }
        let probe: u32 = lf_checker_rt::callee_cdecl!(LOOKUP, u32, id);
        if probe == 0 {
            return 0;
        }
        let rec: u32 = lf_checker_rt::callee_cdecl!(LOOKUP, u32, id);
        lf_checker_rt::callee_thiscall!(METHOD, u32, rec, sub)
    }
});
