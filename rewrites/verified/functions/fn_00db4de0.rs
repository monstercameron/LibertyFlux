// original: 0x00DB4DE0 uimouse_clear_pair

/// Clear the two trailing slots of a mouse-cursor object.
///
/// `this` points to the object; the words at `+0x200` and `+0x204` are set
/// to 0. Nothing else is read or written and there is no return value (the
/// original leaves `eax` untouched, so the return channel is not compared).
///
/// Original: 0x00DB4DE0 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00db4de0(this: u32) -> u32 {
    unsafe {
        const SLOT_A: u32 = 0x200;
        const SLOT_B: u32 = 0x204;
        ((this + SLOT_A) as *mut u32).write_unaligned(0);
        ((this + SLOT_B) as *mut u32).write_unaligned(0);
        0
    }
});
