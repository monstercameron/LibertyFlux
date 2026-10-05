// original: 0x00ae1280 ui_pressed_or_state_bit0
/// Input predicate: shared bit 0 set, or the device state says active.
///
/// Returns 1 when bit 0 of the shared word is set; otherwise, when the
/// device-enable byte is nonzero, when the state's xor checksum exceeds
/// 0x7f. The upper 24 bits of the result repeat the worker answer.
export!(cdecl, rw_00ae1280() -> u32 {
    unsafe {
        const SHARED: u32 = 0x018B7A88;
        let dev = callee_cdecl!(1, u32, 0);
        let out = if *global::<u8>(SHARED) & 1 != 0 {
            1
        } else if *((dev + 0x328c) as *const u8) == 0 {
            0
        } else {
            let sum = *((dev + 0x2bce) as *const u8) ^ *((dev + 0x2bcc) as *const u8);
            (sum > 0x7f) as u32
        };
        (dev & 0xFFFFFF00) | out
    }
});
