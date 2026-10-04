// original: 0x0059e180 poll_any_activity
// True when the mode byte is nonzero or either secondary state word is set.
//
// Unlike its siblings this one performs no null checks: a null link faults
// on both sides. The secondary words are live here (not a dead probe).
export!(cdecl, rw_0059E180() -> u32 {
    unsafe {
        let o1 = *global::<u32>(0x18B6C98);
        let o1v = *((o1 + 0x1E4) as *const u32);
        let o2 = *((o1v + 0x1E0) as *const u32);
        let mode = *((o2 + 0x21C) as *const u8);
        if mode != 0 {
            return 1;
        }
        let g2 = *global::<u32>(0x18B6C8C);
        let w0 = *((g2 + 0x200) as *const u32);
        if w0 != 0 {
            return 1;
        }
        let w1 = *((g2 + 0x204) as *const u32);
        u32::from(w1 != 0)
    }
});
