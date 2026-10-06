// original: 0x009A56B0 audio_slot_alloc (proposed)

/// Allocate one slot in a fixed 32-entry global table and store a triplet.
///
/// The table at `TABLE` holds 32 entries of five words. The scan compares
/// each entry's first word against -1 with a signed less-than on the walking
/// pointer; the first entry whose marker is -1 wins. On a hit the three
/// stack arguments are stored as marker := `arg2` (offset +0), `arg0` at +4
/// and `arg1` at +8, and `arg0` is returned. When every marker differs from
/// -1 nothing is stored and the one-past-the-end address is returned.
/// Thiscall shape: plain stdcall of three words, callee pops 12.
lf_checker_rt::export!(stdcall, rw_009A56B0(arg0: u32, arg1: u32, arg2: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x012847D8;
        const ENTRIES: u32 = 32;
        const STRIDE: u32 = 20;
        const FREE: u32 = 0xFFFF_FFFF;
        const END_OFF: u32 = ENTRIES * STRIDE;
        let base = lf_checker_rt::relocated(TABLE);
        let end = base.wrapping_add(END_OFF);
        let mut cur = base;
        // Signed pointer comparison, as the original's `jl`.
        while (cur as i32) < (end as i32) {
            if ((cur as *const u32).read_unaligned() ^ FREE) == 0 {
                break;
            }
            cur = cur.wrapping_add(STRIDE);
        }
        if (cur as i32) >= (end as i32) {
            return end;
        }
        let idx = cur.wrapping_sub(base) / STRIDE;
        let slot = base.wrapping_add(idx.wrapping_mul(STRIDE)) as *mut u32;
        slot.add(2).write_unaligned(arg1);
        slot.add(0).write_unaligned(arg2);
        slot.add(1).write_unaligned(arg0);
        arg0
    }
});
