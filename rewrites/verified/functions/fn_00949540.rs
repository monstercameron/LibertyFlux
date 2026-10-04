// original: 0x00949540 obj_clear_slot_flags
/// Clear one of two per-slot flag bytes selected by the low byte of
/// `which`, and when both flags end up clear, clear the slot's 16-bit
/// state word too. Negative indexes do nothing.
export!(thiscall, rw_00949540(obj: *mut u8, idx: i32, which: u32) -> u32 {
    unsafe {
        if idx < 0 {
            return 0;
        }
        let base = obj.add(idx as usize);
        if (which & 0xFF) != 0 {
            *base.add(0xAF20) = 0;
        } else {
            *base.add(0xAF00) = 0;
        }
        if *base.add(0xAF20) != 0 || *base.add(0xAF00) != 0 {
            return 0;
        }
        let off = (idx as u32).wrapping_add(0xC0).wrapping_mul(0xC8);
        *(obj.add(off as usize) as *mut u16) = 0;
        0
    }
});
