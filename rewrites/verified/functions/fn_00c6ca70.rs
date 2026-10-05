// original: 0x00c6ca70 anim_open_find_call (proposed)

/// Open a record by name, then invoke its handler with a cooked sub-key.
///
/// `TO_INDEX(key)` maps the name (-1 means missing, return 0);
/// `LOOKUP(id)` must find a record (else return 0); the record is
/// looked up again, `COOK(sub, 0)` prepares the second argument, and
/// `METHOD(record, cooked)` runs, its answer returned.
///
/// Original: cdecl with two stack words, five call sites.
lf_checker_rt::export!(cdecl, rw_00c6ca70(key: u32, sub: u32) -> u32 {
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
        let probe: u32 = lf_checker_rt::callee_cdecl!(LOOKUP, u32, id);
        if probe == 0 {
            return 0;
        }
        let rec: u32 = lf_checker_rt::callee_cdecl!(LOOKUP, u32, id);
        let cooked: u32 = lf_checker_rt::callee_cdecl!(COOK, u32, sub, 0);
        lf_checker_rt::callee_thiscall!(METHOD, u32, rec, cooked)
    }
});
