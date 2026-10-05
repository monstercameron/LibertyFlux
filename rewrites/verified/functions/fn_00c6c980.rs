// original: 0x00c6c980 anim_resolve_call (proposed)

/// Resolve a record by key and invoke its handler with the sub-key.
///
/// `LOOKUP(key)` gives the record; `METHOD(record, sub)` (thiscall) is
/// then called and its answer returned.
///
/// Original: cdecl with two stack words, two calls.
lf_checker_rt::export!(cdecl, rw_00c6c980(key: u32, sub: u32) -> u32 {
    unsafe {
        const LOOKUP: u32 = 1;
        const METHOD: u32 = 2;

        let rec: u32 = lf_checker_rt::callee_cdecl!(LOOKUP, u32, key);
        lf_checker_rt::callee_thiscall!(METHOD, u32, rec, sub)
    }
});
