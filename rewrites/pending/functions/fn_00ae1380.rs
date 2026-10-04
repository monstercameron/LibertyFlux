// original: 0x00ae1380 ui_pressed_or_state_bit1
/// Input predicate: shared bit 1 set, or the device state says active.
///
/// Bit-1 twin of `ui_pressed_or_state_bit0`, reading its checksum from the
/// neighbouring state bytes. The upper 24 bits repeat the worker answer.
export!(cdecl, rw_00ae1380() -> u32 {
    unsafe {
        const SHARED: u32 = 0x018B7A88;
        let dev = callee_cdecl!(1, u32, 0);
        let out = if *global::<u8>(SHARED) & 2 != 0 {
            1
        } else if *((dev + 0x328c) as *const u8) == 0 {
            0
        } else {
            let sum = *((dev + 0x2bde) as *const u8) ^ *((dev + 0x2bdc) as *const u8);
            (sum > 0x7f) as u32
        };
        (dev & 0xFFFFFF00) | out
    }
});
