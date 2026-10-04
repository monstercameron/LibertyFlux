// original: 0x00B4FDC0 ped_set_dwords130_134

/// Store two words into a ped block at offsets 0x130 and 0x134.
///
/// Returns the second word (the original leaves it in eax).
///
/// Original: 0x00B4FDC0 (thiscall, `this` in ecx, two stack words).
export!(thiscall, rw_00b4fdc0(this: u32, first: u32, second: u32) -> u32 {
    const SLOT0: u32 = 0x130;
    const SLOT1: u32 = 0x134;
    unsafe {
        ((this + SLOT0) as *mut u32).write_unaligned(first);
        ((this + SLOT1) as *mut u32).write_unaligned(second);
    }
    second
});
