// original: 0x009B0930 audPoliceScanner::alloc_report_slot

/// Selects a scanner slot from the 32-entry ring at `this + 0x70`, with
/// entries spaced `0x1a0` bytes apart. If the current index at `+0x2c` equals
/// the boundary word at `+0x30`, it advances once before selecting. It then
/// stores the selected index plus one, wrapped to five bits, and returns the
/// selected entry address. All index arithmetic follows 32-bit wrapping
/// behavior.
lf_checker_rt::export!(thiscall, rw_009b0930(this: u32) -> u32 {
    const SLOT_INDEX: u32 = 0x2c;
    const SLOT_BOUNDARY: u32 = 0x30;
    const SLOTS: u32 = 0x70;
    const SLOT_STRIDE: u32 = 0x1a0;
    const SLOT_MASK: u32 = 0x1f;

    unsafe {
        let current = ((this + SLOT_INDEX) as *const u32).read_unaligned();
        let boundary = ((this + SLOT_BOUNDARY) as *const u32).read_unaligned();
        let selected = if current == boundary { current.wrapping_add(1) } else { current };
        let next = selected.wrapping_add(1) & SLOT_MASK;
        ((this + SLOT_INDEX) as *mut u32).write_unaligned(next);
        this.wrapping_add(SLOTS).wrapping_add(selected.wrapping_mul(SLOT_STRIDE))
    }
});
