// original: 0x008f0210 validate_window_c
/// Validate window C: same window check over its own object offset.
export!(thiscall, rw_008f0210(this: u32) -> u32 {
    callee_thiscall!(1, u32, this.wrapping_add(0x2DB8), 0x3E8, 0x80, 0xFF)
});
