// original: 0x0059e0f0 poll_mode_is_two
// True when the chained state is live and its mode byte equals 2.
//
// Same shape as poll_mode_is_one, including the always-false secondary
// probe path for a zero mode byte; only the compared constant differs.
export!(cdecl, rw_0059E0F0() -> u32 {
    unsafe {
        let o1 = *global::<u32>(0x18B6C98);
        let o1v = *((o1 + 0x1E4) as *const u32);
        if o1v == 0 {
            return 0;
        }
        let o2 = *((o1v + 0x1E0) as *const u32);
        if o2 == 0 {
            return 0;
        }
        let mode = *((o2 + 0x21C) as *const u8);
        if mode != 0 {
            return u32::from(mode == 2);
        }
        let g2 = *global::<u32>(0x18B6C8C);
        // Volatile for the same reason as poll_mode_is_one: keep the
        // original's probe reads (and their null-base fault) intact.
        let w0 = std::ptr::read_volatile((g2 + 0x200) as *const u32);
        if w0 != 0 {
            return 0;
        }
        let w1 = std::ptr::read_volatile((g2 + 0x204) as *const u32);
        if w1 != 0 {
            return 0;
        }
        0
    }
});
