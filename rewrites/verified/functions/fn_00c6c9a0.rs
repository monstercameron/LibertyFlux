// original: 0x00c6c9a0 anim_open_call (proposed)

/// Open a record by name and invoke its handler with a cooked sub-key.
///
/// `TO_INDEX(key)` maps the name to an id (-1 means missing, return 0);
/// `LOOKUP(id)` must find a record (else return 0); `COOK(sub, 0)`
/// prepares the second argument; `METHOD(record, cooked)` runs and its
/// answer is returned.
///
/// Original: cdecl with two stack words, four calls.
lf_checker_rt::export!(cdecl, rw_00c6c9a0(key: u32, sub: u32) -> u32 {
    unsafe {
        const TO_INDEX: u32 = 1;
        const LOOKUP: u32 = 2;
        const COOK: u32 = 3;
        const METHOD: u32 = 4;
        const MISSING: u32 = 0xFFFF_FFFF;

        let id: u32 = lf_checker_rt::callee_cdecl!(TO_INDEX, u32, key);
        if id == MISSING {
            return 0;
        }
        let rec: u32 = lf_checker_rt::callee_cdecl!(LOOKUP, u32, id);
        if rec == 0 {
            return 0;
        }
        let cooked: u32 = lf_checker_rt::callee_cdecl!(COOK, u32, sub, 0);
        lf_checker_rt::callee_thiscall!(METHOD, u32, rec, cooked)
    }
});
