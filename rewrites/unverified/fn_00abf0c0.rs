// original: 0x00ABF0C0 stream_touch_timed_key (proposed)

/// Refresh the timed key at or below `time`, or report it missing.
///
/// The original fetches the key-set cursor, compares its time with the float
/// argument (cdecl, one word) and returns 0x280 when the cursor time is not
/// strictly above it. Otherwise it prunes the key set through the prune
/// callee (passing two spill slots holding the id and time) and re-inserts
/// through the insert callee, returning the key id.
lf_checker_rt::export!(cdecl, rw_00ABF0C0(time: u32) -> u32 {
    unsafe {
        const KEYSET: u32 = 0x0150E278;
        const FETCH: u32 = 1;
        const PRUNE: u32 = 2;
        const INSERT: u32 = 3;
        const MISSING: u32 = 0x280;
        const ID_OFF: u32 = 0x10;
        const TIME_OFF: u32 = 0x14;
        let cur = lf_checker_rt::callee_cdecl!(FETCH, u32, KEYSET);
        let id = (cur.wrapping_add(ID_OFF) as *const u32).read_unaligned();
        let ct = (cur.wrapping_add(TIME_OFF) as *const f32).read_unaligned();
        if !(ct > f32::from_bits(time)) {
            return MISSING;
        }
        let mut w = [id, ct.to_bits()];
        lf_checker_rt::callee_thiscall!(PRUNE, u32, KEYSET, &mut w as *mut u32 as u32);
        lf_checker_rt::callee_cdecl!(INSERT, u32, id, time);
        id
    }
});
