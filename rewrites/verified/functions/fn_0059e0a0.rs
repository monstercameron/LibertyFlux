// original: 0x0059e0a0 poll_mode_is_one
// True when the chained state is live and its mode byte equals 1.
//
// With a zero mode byte the original probes two secondary state words, but
// the final comparison tests the mode byte (== 0) against 1, so that path
// always returns 0. The probe reads are kept for fault parity.
export!(cdecl, rw_0059E0A0() -> u32 {
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
            return u32::from(mode == 1);
        }
        let g2 = *global::<u32>(0x18B6C8C);
        // Volatile: every path below returns 0, so plain reads would be
        // deleted as dead (verified in the built DLL); the original still
        // performs these reads and faults on a null secondary base, and
        // that fault parity is behavior worth keeping.
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
