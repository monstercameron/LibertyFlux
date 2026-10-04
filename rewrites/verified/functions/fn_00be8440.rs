// original: 0x00BE8440 timer_refresh_all (proposed)
/// Refresh the head timer and every live slot timer.
///
/// `obj` points to the timer block. The any-live byte at `+0x154` and
/// the head flag at `+0x3e` are cleared first. When the head word at
/// `+0x00` is nonzero the head refresh callee runs (thiscall, no stack
/// words) and both flags are set. Then each of the four slot records at
/// `+0x44`, stepping `+0x44`, has its done byte at slot `+0x40` cleared;
/// a slot whose head word is nonzero is refreshed through the slot
/// callee (thiscall on the slot, with its word at `+0x14`), and its done
/// byte and the any-live byte are set. No result.
///
/// Original: 0x00BE8440 (thiscall, no stack words). The two callees are
/// the neighbouring functions 0x00BE84A0 and 0x00BE8510, intercepted here.
lf_checker_rt::export!(thiscall, rw_00BE8440(obj: u32) -> u32 {
    unsafe {
        const HEAD: u32 = 0x00;
        const HEAD_FLAG: u32 = 0x3e;
        const ANY_LIVE: u32 = 0x154;
        const SLOTS: u32 = 0x44;
        const STRIDE: u32 = 0x44;
        const COUNT: u32 = 4;
        const SLOT_WORD: u32 = 0x14;
        const SLOT_DONE: u32 = 0x40;
        const HEAD_REFRESH: u32 = 1;
        const SLOT_REFRESH: u32 = 2;
        ((obj + ANY_LIVE) as *mut u8).write(0);
        ((obj + HEAD_FLAG) as *mut u8).write(0);
        if ((obj + HEAD) as *const u32).read_unaligned() != 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(HEAD_REFRESH, u32, obj);
            ((obj + HEAD_FLAG) as *mut u8).write(1);
            ((obj + ANY_LIVE) as *mut u8).write(1);
        }
        let mut slot = obj.wrapping_add(SLOTS);
        let mut i = 0u32;
        while i < COUNT {
            ((slot + SLOT_DONE) as *mut u8).write(0);
            if (slot as *const u32).read_unaligned() != 0 {
                let w = ((slot + SLOT_WORD) as *const u16).read_unaligned() as u32;
                let _: u32 = lf_checker_rt::callee_thiscall!(SLOT_REFRESH, u32, slot, w);
                ((slot + SLOT_DONE) as *mut u8).write(1);
                ((obj + ANY_LIVE) as *mut u8).write(1);
            }
            slot = slot.wrapping_add(STRIDE);
            i += 1;
        }
        0
    }
});

