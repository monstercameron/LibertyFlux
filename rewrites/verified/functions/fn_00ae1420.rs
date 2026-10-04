// original: 0x00ae1420 ui_edge2_or_state_bit1
/// Input predicate: falling-side edge on line 2 bit 1, or the state gate.
///
/// Bit-1 twin of `ui_edge2_or_state_bit0`. The upper 24 bits of the result
/// repeat the worker answer.
export!(cdecl, rw_00ae1420() -> u32 {
    unsafe {
        const PREV: u32 = 0x018B7A84;
        const CUR: u32 = 0x018B7A88;
        let dev = callee_cdecl!(1, u32, 0);
        let prev = *global::<u32>(PREV);
        let edge = (*global::<u32>(CUR) ^ prev) & prev;
        let out = if edge & 2 != 0 {
            1
        } else if *((dev + 0x328c) as *const u8) == 0 {
            0
        } else {
            let refb = *((dev + 0x2bdc) as *const u8);
            let d1 = *((dev + 0x2bde) as *const u8) ^ refb;
            if d1 > 0x7f {
                0
            } else {
                let d2 = *((dev + 0x2bdf) as *const u8) ^ refb;
                (d2 > 0x7f) as u32
            }
        };
        (dev & 0xFFFFFF00) | out
    }
});
