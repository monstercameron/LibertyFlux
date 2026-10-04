// original: 0x00ae1320 ui_edge2_or_state_bit0
/// Input predicate: falling-side edge on line 2 bit 0, or the state gate.
///
/// Like its sibling but the edge term masks with the previous word, and the
/// state gate wants the first checksum at or below 0x7f with the second
/// above it. The upper 24 bits of the result repeat the worker answer.
export!(cdecl, rw_00ae1320() -> u32 {
    unsafe {
        const PREV: u32 = 0x018B7A84;
        const CUR: u32 = 0x018B7A88;
        let dev = callee_cdecl!(1, u32, 0);
        let prev = *global::<u32>(PREV);
        let edge = (*global::<u32>(CUR) ^ prev) & prev;
        let out = if edge & 1 != 0 {
            1
        } else if *((dev + 0x328c) as *const u8) == 0 {
            0
        } else {
            let refb = *((dev + 0x2bcc) as *const u8);
            let d1 = *((dev + 0x2bce) as *const u8) ^ refb;
            if d1 > 0x7f {
                0
            } else {
                let d2 = *((dev + 0x2bcf) as *const u8) ^ refb;
                (d2 > 0x7f) as u32
            }
        };
        (dev & 0xFFFFFF00) | out
    }
});
