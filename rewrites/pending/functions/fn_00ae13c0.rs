// original: 0x00ae13c0 ui_edge1_or_state_bit1
/// Input predicate: rising edge on line 1 bit 1, or the two-byte state gate.
///
/// Bit-1 twin of `ui_edge1_or_state_bit0`. The upper 24 bits of the result
/// repeat the worker answer.
export!(cdecl, rw_00ae13c0() -> u32 {
    unsafe {
        const PREV: u32 = 0x018B7A84;
        const CUR: u32 = 0x018B7A88;
        let dev = callee_cdecl!(1, u32, 0);
        let cur = *global::<u32>(CUR);
        let rise = (cur ^ *global::<u32>(PREV)) & cur;
        let out = if rise & 2 != 0 {
            1
        } else if *((dev + 0x328c) as *const u8) == 0 {
            0
        } else {
            let refb = *((dev + 0x2bdc) as *const u8);
            let d1 = *((dev + 0x2bde) as *const u8) ^ refb;
            if d1 <= 0x7f {
                0
            } else {
                let d2 = *((dev + 0x2bdf) as *const u8) ^ refb;
                (d2 <= 0x7f) as u32
            }
        };
        (dev & 0xFFFFFF00) | out
    }
});
