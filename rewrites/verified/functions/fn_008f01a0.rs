// original: 0x008f01a0 validate_window_a
/// Validate window A: runs the shared 64-slot window check over the window
/// at its object offset with the fixed (limit, low, high) triple.
export!(thiscall, rw_008f01a0(this: u32) -> u32 {
    callee_thiscall!(1, u32, this.wrapping_add(0x2EF8), 0x3E8, 0x80, 0xFF)
});
