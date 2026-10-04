// original: 0x00d296e0 target_slot_refresh_all (proposed)

/// Refresh all 8 slot records for the given mode.
///
/// For each slot at `this + 0x28 + i * 0x40` whose link at `+0x0c` is
/// nonzero: clears bit 0 of the byte at `+0x24`, folds `mode` into the word
/// at `+0x20` keeping three bits (`w ^= (w ^ mode) & 7`), and then — for mode
/// 1 only — resets the float at `+0x1c` to 5.0, copies the linked point's
/// four words (past its link at `+0x20` plus `0x30`, or `+0x10` past the
/// object itself when null) into `+0x20 + i * 0x40 - 8` and runs the finish
/// helper (intercepted) on that address; any other mode just clears the word
/// at `+0x28`. The original leaves `eax` untouched, so no return channel is
/// compared.
///
/// Original: 0x00D296E0 (thiscall, one stack argument).
lf_checker_rt::export!(thiscall, rw_00d296e0(this: u32, mode: u32) -> u32 {
    unsafe {
        const SLOT_BASE: u32 = 0x28;
        const STRIDE: u32 = 0x40;
        const COUNT: u32 = 8;
        const LINK_OFF: u32 = 0x0c;
        const FLAG_OFF: u32 = 0x24;
        const FOLD_OFF: u32 = 0x20;
        const FOLD_MASK: u32 = 7;
        const FLOAT_OFF: u32 = 0x1c;
        const RESET_BITS: u32 = 0x40a0_0000; // 5.0f
        const CLEAR_OFF: u32 = 0x28;
        const PT_LINK_OFF: u32 = 0x20;
        const LINKED_PT_OFF: u32 = 0x30;
        const DIRECT_PT_OFF: u32 = 0x10;
        const REFRESH_MODE: u32 = 1;
        let mut i = 0u32;
        while i < COUNT {
            let slot = this + SLOT_BASE + i * STRIDE;
            let link = unsafe { ((slot + LINK_OFF) as *const u32).read_unaligned() };
            if link != 0 {
                unsafe {
                    ((slot + FLAG_OFF) as *mut u8)
                        .write(((slot + FLAG_OFF) as *const u8).read() & 0xfe)
                };
                let w = unsafe { ((slot + FOLD_OFF) as *const u32).read_unaligned() };
                unsafe { ((slot + FOLD_OFF) as *mut u32).write_unaligned(w ^ ((w ^ mode) & FOLD_MASK)) };
                if mode == REFRESH_MODE {
                    unsafe { ((slot + FLOAT_OFF) as *mut u32).write_unaligned(RESET_BITS) };
                    let pt = unsafe { ((link + PT_LINK_OFF) as *const u32).read_unaligned() };
                    let src = if pt != 0 { pt + LINKED_PT_OFF } else { link + DIRECT_PT_OFF };
                    let dst = slot - 8;
                    for off in [0u32, 4, 8, 12] {
                        let v = unsafe { ((src + off) as *const u32).read_unaligned() };
                        unsafe { ((dst + off) as *mut u32).write_unaligned(v) };
                    }
                    let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, dst);
                } else {
                    unsafe { ((slot + CLEAR_OFF) as *mut u32).write_unaligned(0) };
                }
            }
            i += 1;
        }
        0
    }
});
